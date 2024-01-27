<img src="https://static.wikia.nocookie.net/godofwar/images/e/e3/Fj1v1EXaAAA6H7Y.jpeg/revision/latest/scale-to-width-down/1000?cb=20230107133419" height="300" width="700"/>

# Tyr - A Node.js Package Manager Written in Rust

Tyr is a lightweight, fast, and reliable package manager for Node.js projects, implemented in Rust. It aims to provide efficient dependency management, project initialization, and package installation workflows.

## Running from Source

To run Tyr from the source code on GitHub, ensure that you have Rust installed. You can install Rust by following the instructions on the [official website](https://www.rust-lang.org/tools/install).

Clone the repository:

```sh
git clone https://github.com/jkitsao/tyr.git
```

Navigate to the project directory:

```sh
cd tyr
```

Run Tyr with Cargo:

```sh
cargo run <command>
```

Replace <command> with one of the supported commands, such as init, add, etc.

# Features

- **`init`:** Command: Initialize a new Node.js project with a basic project structure and configuration.
- **`add`** Command: Fetch new packages from npm and
  generate a lock file `tyr.lock` to ensure reproducible builds.
- **`install`** Command: Work in progress. Install dependencies specified in the tyr.lock file.

# Roadmap

## Minimum Viable Product (MVP)

- `Init` Command: Initialize a new Node.js project with a basic project structure and a package.json file.
- `Add` Command: Fetch dependencies from npm and update the package.json file.
- `Lock File Generation:` Create a lock file (tyr.lock) to track dependency versions.
- `Dependency Resolution:` Ensure consistent dependency versions across different environments.
- `Install Command:` Install dependencies specified in the tyr.lock file.

# Additional Features

- `Dependency Management:` Support for updating, removing, and listing dependencies.
- `Script Execution:` Ability to execute scripts defined in the package.json file.
- `Registry Support:` Allow configuring custom registries for package installation.
- `Concurrency:` Parallel dependency resolution and installation for faster performance.
- `Versioning:` Semantic versioning support for package management and updates.

⚠️ **Warning: Do Not Use in Production**

Tyr is currently in early development and may not be suitable for use in production environments. Use it at your own risk.

---
