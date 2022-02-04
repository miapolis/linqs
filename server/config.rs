use colored::*;
use serde::Deserialize;
use std::fmt;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Clone)]
pub struct Config {
    pub options: Options,
}

#[derive(Deserialize)]
struct RawConfig {
    pub options: RawOptions,
}

#[derive(Debug, Clone)]
pub struct Options {
    pub new_accounts: bool,
}

#[derive(Deserialize)]
struct RawOptions {
    pub new_accounts: Option<bool>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            options: Options { new_accounts: true },
        }
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.options)
    }
}

impl Config {
    pub fn create() -> Self {
        if let Ok(mut file) = File::open("linqs.toml") {
            let mut buffer = String::new();
            file.read_to_string(&mut buffer).unwrap();

            let raw_res: Result<RawConfig, toml::de::Error> = toml::from_str(&buffer);
            if raw_res.is_err() {
                println!("{}", "Error in config file linqs.toml".white().bold());
                println!("{}", format!("{}", &raw_res.err().unwrap()).red());
                std::process::exit(2);
            } else {
                Self::from_raw(&raw_res.unwrap())
            }
        } else {
            Self::default()
        }
    }

    fn from_raw(raw: &RawConfig) -> Self {
        Self {
            options: Options::from_raw(&raw.options),
        }
    }

    pub fn print(&self) {
        let cfg_str = "CONFIG:".white().bold();
        let struct_str = format!("{}", self).green();
        println!("\n{} {{ {} }}\n", cfg_str, struct_str);
    }
}

impl fmt::Display for Options {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "NEW ACCOUNTS: {}",
            format!("{}", self.new_accounts).blue()
        )
    }
}

impl Options {
    fn from_raw(raw: &RawOptions) -> Self {
        Self {
            new_accounts: raw.new_accounts.unwrap_or(true),
        }
    }
}
