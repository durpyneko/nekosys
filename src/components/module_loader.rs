use colored::Colorize;
use dotenvy::from_path_iter;
use log::{debug, error, info};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::process::{Command, Stdio};
use std::{collections::HashMap, fs};

use crate::components::startup_anim;

#[derive(Serialize, Deserialize, Debug)]
struct Index {
    modules: HashMap<String, Modules>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Modules {
    enabled: bool,
    module_path: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ModuleIndex {
    config: ModuleConfig,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum ModuleConfigType {
    Exe,
}

#[derive(Serialize, Deserialize, Debug)]
struct ModuleConfig {
    #[serde(rename = "type")]
    r#type: Option<ModuleConfigType>,
    exe_path: Option<String>,
    args: Option<Vec<String>>,
    env_path: Option<String>,
}

pub fn init() {
    log::info!("Loading modules...");
    startup_anim::animate();

    // ? should i hardcode the modules path or pass it?
    // ? maybe app config?
    let content: Index = read_toml("modules/index.toml");

    debug!("{:?}", content);

    for (name, module) in &content.modules {
        match module {
            Modules {
                enabled: true,
                module_path,
            } => {
                info!(
                    "Module '{}' is {}, module_path: {}",
                    name,
                    "enabled".bright_green(),
                    module_path.as_ref().unwrap()
                );
                let full_path =
                    format!("{}/{}", module_path.as_ref().unwrap(), "module.index.toml");
                let content: ModuleIndex = read_toml(full_path);
                debug!("{:?}", content.config);

                // if type exe
                if let (Some(ModuleConfigType::Exe), Some(exe_path), Some(args), Some(env_path)) = (
                    content.config.r#type,
                    content.config.exe_path,
                    content.config.args,
                    content.config.env_path,
                ) {
                    run_exe_thread(exe_path, args, env_path);
                }
            }
            Modules { enabled: false, .. } => {
                info!("Module '{}' is {}", name, "disabled".red());
            }
        }
    }
}

fn read_toml<S, T>(path: S) -> T
where
    S: AsRef<str>,
    T: DeserializeOwned,
{
    let content = fs::read_to_string(path.as_ref()).expect("Failed to read index file");
    toml::from_str::<T>(&content).expect("Failed to parse TOML")
}

fn run_exe_thread(exe_path: String, args: Vec<String>, env_path: String) {
    info!("Starting: {}...", exe_path);

    let _ = tokio::spawn(async move {
        let mut command = Command::new(exe_path);
        command
            .args(args)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .stdin(Stdio::null());

        if let Some(path) = Some(env_path) {
            match from_path_iter(&path) {
                Ok(iter) => {
                    for result in iter {
                        match result {
                            Ok((key, value)) => {
                                command.env(key, value); // inject into child process only
                            }
                            Err(e) => {
                                error!("Invalid line in .env file '{}': {}", path, e);
                            }
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to read .env file '{}': {}", path, e);
                }
            }
        }

        command
            .spawn()
            .expect("Failed to start process")
            .wait()
            .expect("Spawn process failed");
    });
}
