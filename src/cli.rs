use crate::init;
use crate::install;
use crate::resolve_package_from_registry;
use clap::{Parser, Subcommand};
/// A fictional versioning CLI
#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "tyr")]
#[command(about = "Faster than NPM", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Debug, Subcommand)]
enum Commands {
    /// Clones repos
    #[command(arg_required_else_help = true)]
    Init {
        /// The remote to clone
        #[arg(required = false)]
        name: String,
    },
    /// adds things
    #[command(arg_required_else_help = true)]
    Add {
        /// Stuff to add
        #[arg(required = true)]
        packages: Vec<String>,
    },
    Install,
}
//
pub fn initialize_command_arguments() {
    let args = Cli::parse();
    //
    match args.command {
        Commands::Add { packages } => {
            //loop over packages and install each
            for package in packages.iter() {
                // println!("Resolving: {}", package);
                resolve_package_from_registry(package.to_owned(),true)
            }
        }
        Commands::Init { name } => {
            println!("Initializing {:?}", name);
            init::init_new_project(name);
        }
        Commands::Install => {
            println!("Installing Dependencies");
            let lockfile_path = "./node_tests/tyr.lock";
            install::load_entries_from_lockfile(lockfile_path)
        }
    }
}
