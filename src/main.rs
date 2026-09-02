use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
    process::Stdio,
};
use log::{
    debug,
    error,
    info
};

use clap::Parser;
use serde::{Deserialize, Serialize};

// Post-run/pre-run scripts
// Need general config
// Notification when "hey, updates are happening right now"

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct Module {
    name: String,
    description: Option<String>,
    program: String,
    context: ExecutionContext,
    run_in_unit: Option<bool>,
    args: Option<Vec<String>>,
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

    for path in [Path::new("/usr/share/rozanne/modules.d"),
        Path::new("/etc/rozanne/modules.d")] {
        if path.exists() {
            return Ok(path.to_path_buf());
        }
    }

    Err(Config::NotFound)
}

fn main() {
    let parsed = Cli::parse();

    env_logger::init();

    debug!("Session is being ran in {:?} context", parsed.context);

    let Ok(config_dir) = get_config_dir(parsed.config_directory) else {
        error!("Failed finding any configuration directory");
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

    info!("Modules found and active for the session:");

    for module in &modules {
        let copied_module = module.clone();
        info!(
            "Running module {}, {}",
            copied_module.name,
            copied_module.description.unwrap_or_default()
        );

        std::process::Command::new(copied_module.program)
            .args(copied_module.args.unwrap_or(vec![]))
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .expect("Failed running chungus");
    }
}
