use clap::{Parser, Subcommand};
use colored::Colorize;

// TODO: Get version dynamically
pub static BANNER: &str = "

 ██████   █████          █████                █████████                    
░░██████ ░░███          ░░███                ███░░░░░███                   
 ░███░███ ░███   ██████  ░███ █████  ██████ ░███    ░░░  █████ ████  █████ 
 ░███░░███░███  ███░░███ ░███░░███  ███░░███░░█████████ ░░███ ░███  ███░░  
 ░███ ░░██████ ░███████  ░██████░  ░███ ░███ ░░░░░░░░███ ░███ ░███ ░░█████ 
 ░███  ░░█████ ░███░░░   ░███░░███ ░███ ░███ ███    ░███ ░███ ░███  ░░░░███
 █████  ░░█████░░██████  ████ █████░░██████ ░░█████████  ░░███████  ██████ 
░░░░░    ░░░░░  ░░░░░░  ░░░░ ░░░░░  ░░░░░░   ░░░░░░░░░    ░░░░░███ ░░░░░░  
                                                          ███ ░███         
                                                         ░░██████          
                                                          ░░░░░░           
                                                                                ♡ durpyneko / v0.1.0
";
pub fn colored_banner() -> String {
    BANNER.truecolor(132, 21, 83).to_string()
}

#[derive(Debug, Parser)]
#[command(name = "NekoSys", about = colored_banner())]
pub struct Cli {
    /* #[command(subcommand)]
    pub command: CliSubcommand, */
    #[arg(short = 'c', long = "config", value_name = "CONFIG")]
    pub config: Option<String>,

    #[arg(short = 'm', long = "model", value_name = "MODEL")]
    pub model: Option<String>,

    #[arg(short = 'w', long = "web", value_name = "WEBUI")]
    pub web: bool,
}

#[derive(Debug, Subcommand)]
pub enum CliSubcommand {}
