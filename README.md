# Crust-Trust 

**Crust-Trust** is a Rust CLI tool designed to effortlessly create, manage, and optimize multi-crate workspaces. Whether you are building a small project or a large, modular system, Crust-Trust helps you scaffold and manage sub-crates, ensuring an efficient structure with minimal dependency duplication.

## Key Features:
- **Workspace Generation**: Quickly generate a Rust workspace with configurable sub-crates and dependencies, all through a single CLI command.
- **Flexible Crate Management**: Define multiple sub-crates with custom dependencies for each, allowing you to create modular projects that suit your needs.
- **Dependency Optimization**: Automatically detects and minimizes duplicate dependencies across sub-crates, optimizing space and performance.
- **Cross-Project Compatibility**: Designed to work with any Rust project, Crust-Trust dynamically adapts to user inputs and setups, making it a versatile tool for any developer.
- **CLI Driven**: Built with a user-friendly CLI interface, utilizing `clap` to provide clear and customizable options.
- **Automatic Workspace Configuration**: Seamlessly integrates generated sub-crates into the workspace Cargo.toml, ensuring everything works together out of the box.
- **Lightweight and Fast**: Designed with performance in mind, Crust-Trust efficiently manages workspace generation without sacrificing speed.

## Usage:

Install the Crust-Trust tool globally using Cargo:
```bash
cargo install crust-trust
```

Create a new Rust workspace and define sub-crates and their dependencies in a single command:
```bash
crust-trust --project my_project -c "ui:druid,gtk,piet" -c "core:serde,mongodb" -c "network:tokio,reqwest,redis"
```

This command will:
- Create a workspace called `my_project`
- Scaffold sub-crates like `ui`, `core`, and `network` with their own dependencies
- Automatically wire everything together in a workspace Cargo.toml

## Why Crust-Trust?

Managing multiple crates in a workspace can be tedious, especially when dealing with dependencies. Crust-Trust streamlines the process by automating the workspace creation and ensuring that dependencies are optimally managed between sub-crates. It saves time, improves workspace organization, and reduces errors, making it an essential tool for any Rust developer.

---
