/*
 *
 *    ██████╗ ██████╗ ███╗   ██╗███████╗██╗ ██████╗       ███╗   ██╗███████╗██╗  ██╗ ██████╗     ██╗   ██╗██████╗
 *   ██╔════╝██╔═══██╗████╗  ██║██╔════╝██║██╔════╝       ████╗  ██║██╔════╝██║ ██╔╝██╔═══██╗    ██║   ██║╚════██╗
 *   ██║     ██║   ██║██╔██╗ ██║█████╗  ██║██║  ███╗█████╗██╔██╗ ██║█████╗  █████╔╝ ██║   ██║    ██║   ██║ █████╔╝
 *   ██║     ██║   ██║██║╚██╗██║██╔══╝  ██║██║   ██║╚════╝██║╚██╗██║██╔══╝  ██╔═██╗ ██║   ██║    ╚██╗ ██╔╝██╔═══╝
 *   ╚██████╗╚██████╔╝██║ ╚████║██║     ██║╚██████╔╝      ██║ ╚████║███████╗██║  ██╗╚██████╔╝     ╚████╔╝ ███████╗
 *    ╚═════╝ ╚═════╝ ╚═╝  ╚═══╝╚═╝     ╚═╝ ╚═════╝       ╚═╝  ╚═══╝╚══════╝╚═╝  ╚═╝ ╚═════╝       ╚═══╝  ╚══════╝
 *                                                                                                                       :3
*/

use std::marker::PhantomData;
use std::path::PathBuf;
use std::{env, fs};

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ConfigNeko<T> {
    filename: String,
    location: PathBuf,
    custom: PathBuf,
    _marker: PhantomData<T>,
}

impl<T> ConfigNeko<T>
where
    T: Serialize + DeserializeOwned + Default,
{
    pub fn new() -> Self {
        let cwd = PathBuf::from(env::args().next().unwrap())
            .canonicalize()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf();

        Self {
            filename: String::new(),
            location: PathBuf::from(cwd),
            custom: PathBuf::new(),
            _marker: PhantomData::default(),
        }
    }

    pub fn filename<S: AsRef<str>>(mut self, filename: S) -> Self {
        self.filename = filename.as_ref().to_string();
        self
    }

    pub fn location(mut self, location: &str) -> Self {
        self.location = PathBuf::from(&location);
        self
    }

    pub fn custom(mut self, full_path: &str) -> Self {
        let path = PathBuf::from(full_path);

        if let Some(parent) = path.parent() {
            self.location = parent.to_path_buf();
        }

        if let Some(file_name) = path.file_name() {
            self.filename = file_name.to_string_lossy().to_string();
        }

        self.custom = path;

        self
    }

    fn path(&self) -> PathBuf {
        if !self.custom.exists() {
            self.location.join(format!("{}{}", &self.filename, ".json"))
        } else {
            self.custom.clone()
        }
    }

    pub fn init(self) -> Self {
        let config_path = self.path();

        if !config_path.exists() {
            let default_config = T::default();
            let pretty_json = serde_json::to_string_pretty(&default_config)
                .expect("Failed to serialize default config");

            fs::create_dir_all(&self.location).expect("Failed to create config directory");
            fs::write(&config_path, pretty_json).expect("Could not write config!");
        }

        self
    }

    pub fn read(&self) -> T {
        let content = fs::read_to_string(self.path()).expect("Failed to read config");
        serde_json::from_str(&content).expect("Failed to deserialize config")
    }

    pub fn read_key<F, R>(&self, get_fn: F) -> R
    where
        F: FnOnce(&T) -> R,
    {
        get_fn(&self.read())
    }

    pub fn write(&self, data: &T) {
        let json = serde_json::to_string_pretty(data).expect("Failed to serialize config");
        fs::write(self.path(), json).expect("Failed to write config");
    }

    pub fn set<K, V>(&self, key: K, value: V)
    where
        K: AsRef<str>,
        V: Into<Value>,
    {
        let path = self.path();

        let content = fs::read_to_string(&path).expect("Failed to read config!");
        let mut json: Value = serde_json::from_str(&content).expect("Failed to parse config!");

        if let Value::Object(ref mut map) = json {
            map.insert(key.as_ref().to_string(), value.into());
        } else {
            panic!("Config file is not a valid JSON object");
        }

        let updated =
            serde_json::to_string_pretty(&json).expect("Failed to serialize updated config");

        fs::write(&path, updated).expect("Failed to write updated config");
    }
}

pub fn init<T>() -> ConfigNeko<T>
where
    T: Serialize + DeserializeOwned + Default,
{
    ConfigNeko::new()
}
