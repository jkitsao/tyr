use crate::init;
use crate::install;
use crate::resolve_package_from_registry;
use crate::scripts;
use clap::{Parser, Subcommand};
use console::{style, Emoji};
use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressStyle};
use std::time::Instant;
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
static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", ":-)");
pub fn initialize_command_arguments() {
    let args = Cli::parse();
    match args.command {
        Commands::Add { packages } => {
            //loop over packages and install each, this handles cases where the user adds
            // ...multiple packages as command args i.e tyrr add react next mantine
            for package in packages.iter() {
                //track the time
                let started = Instant::now();
                let msg = format!(
                    "{} Generating Dependency Graph for: {} \n",
                    TRUCK,
                    style(package.clone()).bright().green()
                );
                println!("{}", style(msg).bold().yellow());
                resolve_package_from_registry(package.to_owned(), true);
                //show elapsed time to the user
                println!(
                    "{} {} {}",
                    SPARKLE,
                    style("Done in").yellow().bold().bright(),
                    style(HumanDuration(started.elapsed()))
                        .yellow()
                        .bold()
                        .bright()
                );
            }
        }
        Commands::Init { name } => {
            init::init_new_project(name);
        }
        Commands::Run { name } => {
            // Script runner
            let _ = scripts::execute_script(name[0].as_str());
        }
        Commands::Install => {
            println!(
                "{}\n",
                style("Installing Dependencies").bold().bright().yellow()
            );
            let lockfile_path = "./tyr.lock";
            install::load_entries_from_lockfile(lockfile_path);
            //get a set of packages to install by computing the sym difference between...
            //...lock file and json file
        }
    }
}
