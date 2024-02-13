<img src="https://static.wikia.nocookie.net/godofwar/images/e/e3/Fj1v1EXaAAA6H7Y.jpeg/revision/latest/scale-to-width-down/1000?cb=20230107133419" height="220" width="450"/>

# Tyrr Package Manager Documentation

## Introduction

Tyrr is a package manager designed to simplify the process of managing dependencies and scaffolding Node.js projects. It is written in **Rust** and is actively developed to provide efficient and reliable functionality. While the package name on npm is 'tyrr' due to naming conflicts, it's referred to as 'Tyr' for clarity throughout this documentation.

### Features

- Init Command: Scaffold a new Node.js project by generating a `package.json` file with essential project details. This command prompts the user for necessary information.
- Add Command: Fetch packages from the NPM registry and update the `tyr.lock` file to track package versions and dependencies.
- Install Command: Resolve dependencies listed in both the `tyr.lock` file and `package.json` file.

## Getting Started

To start using Tyr, you need to install it globally using npm:

`npm install -g tyrr`

After installation, you can use Tyr via the command line interface (`CLI`) using the `tyrr` command.

## Commands

### 1\. Init Command

The `init` command is used to scaffold a new Node.js project. It prompts the user for essential project details such as project name, description, author, etc., and generates a `package.json` file with the provided information.

#### Usage

`tyrr init`

### 2\. Add Command

The `add` command fetches a package from the NPM registry and updates the `tyr.lock` file to track package versions and dependencies.

#### Usage

`tyrr add <package-name>`

### 3\. Install Command

The `install` command resolves dependencies listed in both the `tyr.lock` file and `package.json` file.

#### Usage

`tyrr install`

### Example Usage

## Scaffold a new Node.js project

`tyrr init`

## Add a package from the NPM registry

`tyrr add react`

## Install dependencies

`tyrr install`

## Configuration

Tyr doesn't require any additional configuration files. However, it relies on the `package.json` and `tyr.lock` files to manage project dependencies.

## Contributing

Tyr is an open-source project, and contributions are welcome. If you encounter any bugs or have suggestions for improvements, please feel free to submit a pull request on the [GitHub repository](https://github.com/jkitsao/tyr).

## Feedback and Support

For any feedback, suggestions, or support inquiries, please reach out to us through GitHub Issues or contact us via email at kitsaojackson22@gmail.com.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
