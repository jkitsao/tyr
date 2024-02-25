<img src="https://static.wikia.nocookie.net/godofwar/images/e/e3/Fj1v1EXaAAA6H7Y.jpeg/revision/latest/scale-to-width-down/1000?cb=20230107133419" height="220" width="450"/>

# Tyr - A Node.js Package Manager Written in Rust

Tyr (referred to as 'Tyrr' in the npm ecosystem due to naming conflicts) is a lightweight, fast, and reliable package manager for Node.js projects, implemented in Rust. It aims to provide efficient dependency management, project initialization, and package installation workflows.

# Features

- `Init:` Initialize a new Node.js project with a basic project structure and a package.json file.
- `Add:` Fetch dependencies from npm and update the package.json file.
- `Lock File Generation:` Create a lock file (tyr.lock) to track dependency versions.
- `Script Execution:` Ability to execute scripts defined in the package.json file.
- `Install Command:` Install dependencies specified in the tyr.lock file.
- `Versioning:` Semantic versioning support for package management and updates.

## Installation

### Using npm

To begin using Tyrr, simply install it globally using npm:

```sh

npm install -g tyrr

```

### Using npx

Alternatively, you can use `npx` to run Tyrr without installing it globally:

```sh

npx tyrr <command>

```

## Running from Source

To run Tyr from the source code, ensure that you have Rust installed. You can install Rust by following the instructions on the [official website](https://www.rust-lang.org/tools/install).

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

Replace `command` with one of the supported commands, such as init, add, etc.

Choose the option that best suits your needs to start using Tyrr and enhance your Node.js development workflow.

## Commands

### 1\. Init Command

The `init` command is your gateway to quickly scaffolding a new Node.js project. It guides you through providing essential project details and generates a `package.json` file accordingly.

#### Usage

```

tyrr init

```

###### Example Generated

```js

{
  "name": "my-awesome-project",
  "version": "1.0.0",
  "description": "An awesome Node.js project managed with Tyrr",
  "main": "index.js",
  "author": "Your Name",
  "license": "MIT"
}

```

### 2\. Add Command

The `add` command allows you to fetch packages from the NPM registry and update the `tyr.lock` file to manage package versions and dependencies efficiently.

#### Usage

```sh

tyrr add <package-name>

```

### 3\. Install Command

The `install` command resolves dependencies listed in both the `tyr.lock` file and `package.json` file, ensuring your project has all the necessary dependencies installed.

#### Usage

```sh

tyrr install

```

## Example Usage

Let's walk through a typical workflow with Tyrr:

### Scaffold a New Node.js Project

```sh

tyrr init

```

### Add a Package from the NPM Registry

```sh

tyrr add react

```

### Install Dependencies

```sh

tyrr install

```

### Run Application

```sh

tyrr run dev

```

## Configuration

Tyrr doesn't require any additional configuration files. It leverages the `package.json` and `tyr.lock` files to manage project dependencies seamlessly.

# Additional Improvements

- `Dependency Management:` Support for updating, removing, and listing dependencies.
- `Registry Support:` Allow configuring custom registries for package installation.
- `Concurrency:` Parallel dependency resolution and installation for faster performance.

⚠️ **Warning: Do Not Use in Production**

Tyr is currently in early development and may not be suitable for use in production environments. Use it at your own risk.

---

## Contributing

Tyrr is an open-source project, and we welcome contributions from the community. If you encounter any bugs or have suggestions for improvements, please feel free to submit a pull request on the [GitHub repository](https://github.com/jkitsao/tyr).

## Feedback and Support

For feedback, suggestions, or support inquiries, don't hesitate to reach out to us via GitHub Issues or contact us via email at kitsaojackson22@gmail.com.

## License

This project is licensed under the MIT License. See the LICENSE file for details.
