# tyr

### Simple package manager for Javascript

# Milestone 1: Basic CLI Setup

Set up a basic command-line interface using a library like [clap](https://docs.rs/clap/latest/clap/).
Implement the init command to create a basic `package.json` file.

Switched from [clap](https://docs.rs/clap/latest/clap/) to `std::io` for basic CLI argument parsing.

Command `tyr init` is now supported for initializing a new project

This command walks you through an interactive session to create a package.json file. Some defaults such as the license and initial version are found in yarn’s init-\* config settings.

# Milestone 2: Add Command

Implement the `add` command to fetch and add packages to the project.
Support basic dependency resolution.

In general, a package is simply a folder with code and a package.json file that describes the contents. When you want to use another package, you first need to add it to your dependencies. This means running yarn add [package-name] to install it into your project.

This will also update your package.json and your yarn.lock/package.lock so that other developers working on the project will get the same dependencies as you when they run `install` command.

# Milestone 3: Improved Dependency Resolution

Enhance the dependency resolution algorithm.
Handle version constraints and dependencies' metadata
