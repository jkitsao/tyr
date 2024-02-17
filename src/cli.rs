use crate::init;
// use crate::banner::draw_banner;
use crate::install;
use crate::resolve_package_from_registry;
use crate::scripts;
use clap::{Parser, Subcommand};
use console::{style, Emoji};
// use crate::dialogue;
/// Another Node resource Negotiator
#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "tyr")]
#[command(about = "Faster than NPM", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Debug, Subcommand)]
enum Commands {
    /// Used to scaffold a new Node.js project
    /// It prompts the user for essential project details and generates a package.json file with the provided information.
    #[command(arg_required_else_help = false)]
    Init {
        /// Project name not required
        #[arg(required = false)]
        name: Option<String>,
    },
    /// Add dependencies to the project:
    /// fetches a package from the NPM registry and updates the tyr.lock file to track package versions and dependencies.
    /// If the package is not already listed in the package.json file, it will be added as a dependency.
    #[command(arg_required_else_help = true)]
    Add {
        /// Package name to add from NPM is required
        #[arg(required = true)]
        packages: Vec<String>,
    },
    /// Install project dependencies:
    /// resolves dependencies listed in both the tyr.lock file and package.json file.
    /// It installs the necessary packages into the project's node_modules directory.
    Install,
    /// run specified scripts
    Run {
        /// script name specified on Package.json file
        #[arg(required = true)]
        name: Vec<String>,
    },
}
//
static TRUCK: Emoji<'_, '_> = Emoji("🚚  ", "");
pub fn initialize_command_arguments() {
    let args = Cli::parse();
    //
    match args.command {
        Commands::Add { packages } => {
            //loop over packages and install each
            for package in packages.iter() {
                let msg = format!(
                    "{} Fetching version information for {} \n",
                    TRUCK,
                    package.clone()
                );
                println!("{}", style(msg).bold().bright().yellow());
                resolve_package_from_registry(package.to_owned(), true)
            }
        }
        Commands::Init { name } => {
            // println!("Initializing {:?}", name);
            // println!("Initializing: {}",name.unwrap().clone());
            init::init_new_project(name);
            // dialogue::dialogue();
        }
        Commands::Run { name } => {
            //
            let _ = scripts::execute_script(name[0].as_str());
        }
        Commands::Install => {
            println!("Installing Dependencies");
            let lockfile_path = "./tyr.lock";
            install::load_entries_from_lockfile(lockfile_path);
            //get a set of packages to install by computing the sym difference between
            //lock file and json file
        }
    }
}
