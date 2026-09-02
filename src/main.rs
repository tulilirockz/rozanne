use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
    process::Stdio,
};

use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Module {
    name: String,
    description: Option<String>,
    program: String,
    context: ExecutionContext,
    args: Vec<String>,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    config_directory: Option<String>,

    #[clap(short, long)]
    #[arg(value_enum, default_value_t)]
    context: ExecutionContext,
}

#[derive(Serialize, Deserialize, Debug, Clone, clap::ValueEnum, Default, PartialEq)]
#[serde(rename_all = "snake_case")]
enum ExecutionContext {
    System,
    #[default]
    User,
}

#[derive(Debug, PartialEq)]
enum Config {
    NotFound,
}

// user_path -> $XDG_CONFIG_HOME/updater/modules.d (with fallback .config) -> /etc/updater/modules.d/ -> /usr/share/updater/modules.d
fn get_config_dir(extra_config_dir: Option<String>) -> Result<PathBuf, Config> {
    if let Some(config_dir) = extra_config_dir {
        let extra_config_dir_path = Path::new(&config_dir);
        if extra_config_dir_path.exists() {
            return Ok(extra_config_dir_path.to_path_buf());
        }
    }

    if let Ok(xdg_config_home) = std::env::var("XDG_CONFIG_HOME") {
        let chungus_path = Path::new(&xdg_config_home).join("chungusupdate/modules.d");
        if chungus_path.exists() {
            return Ok(chungus_path);
        }
    }

    let usr_path = Path::new("/usr/share/chungusupdate/modules.d");
    if usr_path.exists() {
        return Ok(usr_path.to_path_buf());
    }

    let etc_path = Path::new("/etc/chungusupdate/modules.d");
    if etc_path.exists() {
        return Ok(etc_path.to_path_buf());
    }

    Err(Config::NotFound)
}

fn main() {
    let parsed = Cli::parse();

    println!("Session is being ran in {:?} context", parsed.context);

    let config_dir = if let Ok(config) = get_config_dir(parsed.config_directory) {
        config
    } else {
        println!("Failed finding any configuration directory");
        return;
    };

    let modules: Vec<Module> = config_dir
        .read_dir()
        .expect("Failed reading modules directory")
        .filter_map(std::result::Result::ok)
        .filter(|file| {
            if let Ok(entry) = file.file_type() {
                return entry.is_file();
            }
            false
        })
        .map(|file| {
            let unserialized = read_to_string(file.path()).expect("Failed reading module file");

            toml::from_str(&unserialized).expect("Failed parsing module file")
        })
        .filter(|f: &Module| f.context == parsed.context)
        .collect();

    println!("Modules found and active for the session:");

    for module in &modules {
        let copied_module = module.clone();
        println!(
            "Running module {}, {}",
            copied_module.name,
            copied_module.description.expect("")
        );
        std::process::Command::new(copied_module.program)
            .args(copied_module.args)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .expect("Failed running chungus");
    }
}
