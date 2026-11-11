use std::{
    collections::HashSet, fs::read_to_string, path::{Path, PathBuf}, process::Stdio,
};
use log::{
    debug,
    error,
    info
};

use clap::Parser;
use serde::{Deserialize, Serialize};

// Straight from stackoverflow yay
fn has_duplicates(vec: &[String]) -> bool {
    let mut seen = HashSet::new();
    // .any() short-circuits and returns true as soon as insert() returns false
    vec.iter().any(|item| !seen.insert(item))
}
// Post-run/pre-run scripts

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct Main {
    name: String,
    description: Option<String>,
    context: ExecutionContext,
    program: String,
    args: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct Check {
    program: String,
    args: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct Notify {
    program: String,
    args: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct Module {
    main: Main,
    notify: Option<Notify>,
    check: Option<Check>,
}


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    modules_directory: Option<String>,

    #[clap(short, long)]
    config: Option<String>,

    #[clap(long)]
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
fn get_modules_dir(extra_config_dir: Option<String>) -> Result<PathBuf, Config> {
    if let Some(config_dir) = extra_config_dir {
        let extra_config_dir_path = Path::new(&config_dir);
        if extra_config_dir_path.exists() {
            return Ok(extra_config_dir_path.to_path_buf());
        }
    }

    let home_path = format!("{}/.config/rozanne/modules.d", std::env::var("HOME").unwrap());
    let user_config_dir = Path::new(&home_path);

    for path in [Path::new("/usr/share/rozanne/modules.d"),
        Path::new("/etc/rozanne/modules.d"), user_config_dir] {
        if path.exists() {
            return Ok(path.to_path_buf());
        }
    }

    Err(Config::NotFound)
}

fn main() {
    let parsed_cli = Cli::parse();

    env_logger::init();

    debug!("Session is being ran in {:?} context", parsed_cli.context);

    let Ok(config_dir) = get_modules_dir(parsed_cli.modules_directory) else {
        error!("Failed finding any module directory");
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
        .filter(|f: &Module| f.main.context == parsed_cli.context)
        .collect();

    let module_names: Vec<String> = modules.iter().map(|module| module.main.name.clone()).collect();
    info!("Modules found and active for the session: {module_names:?}");

    if has_duplicates(&module_names) {
        error!("Duplicate modules found, exiting.");
        return;
    }

    for module in &modules {
        let copied_module = module.clone();
        info!(
            "Running module {}, {}",
            copied_module.main.name,
            copied_module.main.description.unwrap_or_default()
        );

        // This is terrible, I really despise the code structure here
        // FIXME: figure out a fancier way of doing this
        if let Some(check_module) = copied_module.check {
            debug!(
                "Executing check for {}",
                copied_module.main.name,
            );

            debug!("Executing: {} {:?}", check_module.program, check_module.args.clone().unwrap_or(vec![]));
            if let Ok(status) = std::process::Command::new(check_module.program)
                .args(check_module.args.unwrap_or(vec![]))
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status() {
                if !status.success() {
                    error!("Check failed for module {}, skipping.", copied_module.main.program);
                    continue;
                }
                info!("Successfully ran check for {}", copied_module.main.program);
            } else {
                error!("Check failed for module {}, skipping.", copied_module.main.program);
                continue;
            }
        } else {
            debug!("No check found for module");
        }

        debug!("Executing: {} {:?}", copied_module.main.program, copied_module.main.args.clone().unwrap_or(vec![]));
        if let Ok(status) = std::process::Command::new(copied_module.main.program)
            .args(copied_module.main.args.unwrap_or(vec![]))
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status() {

            if !status.success() {
                error!("Failure while running module {}", copied_module.main.name);
                continue;
            }
            
            info!("Module {} properly executed.", copied_module.main.name);
        }  else {
            error!("Failure while running module {}", copied_module.main.name);

            if let Some(notify_module) = copied_module.notify {
                debug!(
                    "Showing failure notification for {}",
                    copied_module.main.name,
                );

                debug!("Executing: {} {:?}", notify_module.program, notify_module.args.clone().unwrap_or(vec![]));
                if let Ok(status_notify) = std::process::Command::new(notify_module.program)
                    .args(notify_module.args.unwrap_or(vec![]))
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .status() {
                    if status_notify.success() {
                        error!("Failure notifying failure on module {}", copied_module.main.name);
                        continue;
                    }
                    debug!("Successfully notified user of failure on module {}", copied_module.main.name);
                } else {
                    error!("Failure notifying failure on module {}", copied_module.main.name);
                } 
            } else {
                debug!("Notifications on failure for module {} are disabled, not notifying", copied_module.main.name);
            }

        }
    }

    info!("Finalized execution of all modules");
}
