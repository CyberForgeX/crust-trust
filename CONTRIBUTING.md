### Command to create and push a `dev` branch:
```bash
git checkout -b dev && git push -u origin dev
```

### Updated **Contributing.md** focusing on the `dev` branch:
---

# Contributing to Crust-Trust

🎉 Thank you for considering contributing to **Crust-Trust**! 🎉  
We welcome contributions that help improve this workspace management tool and keep it focused on its goal of creating a streamlined, efficient Rust workspace management tool **without** external crates. Below you will find guidelines for contributing.

## Table of Contents
- [Contributing to Crust-Trust](#contributing-to-crust-trust)
  - [Table of Contents](#table-of-contents)
  - [Getting Started](#getting-started)
  - [Contribution Guidelines](#contribution-guidelines)
    - [🚫 **No External Crates**](#-no-external-crates)
    - [📋 **Stay On-Target**](#-stay-on-target)
    - [✅ **Writing Clean Code**](#-writing-clean-code)
    - [🚧 **No Unnecessary Complexity**](#-no-unnecessary-complexity)
  - [Branch Structure](#branch-structure)
  - [Code of Conduct](#code-of-conduct)
  - [Submitting Issues](#submitting-issues)
  - [Pull Request Process](#pull-request-process)
  - [Development Workflow](#development-workflow)
  - [License](#license)

## Getting Started

To get started with contributing, you will need to:

1. **Fork** the repository on GitHub.
2. **Clone** your fork locally:
   ```bash
   git clone https://github.com/cyberforgex/crust-trust.git
   cd crust-trust
   ```
3. Ensure you have the latest **Rust** version installed. You can install Rust from [rust-lang.org](https://www.rust-lang.org/).

4. Start working on your changes in the **dev** branch:
   ```bash
   git checkout -b dev
   ```

## Contribution Guidelines

### 🚫 **No External Crates** 
This project is focused on **writing pure Rust code** without the use of **external crates**. Please do not include any external crates in your contributions. If your change can be made with Rust’s standard library, we strongly encourage you to take that approach. We want to maintain simplicity and avoid dependencies.

### 📋 **Stay On-Target**
We ask that contributors keep the project's original goals in mind when contributing. **Crust-Trust** is designed to:
- Manage Rust workspaces efficiently.
- Provide tools to optimize, audit, benchmark, and automatically handle dependencies.
- Not diverge into unrelated features or stray from its intended purpose.

Contributions that deviate from these goals may not be accepted.

### ✅ **Writing Clean Code**
- Follow Rust's coding conventions and idioms.
- Write clear, concise code.
- Ensure code is **well-documented** and easy to understand.
- **Tests are required** for any new features or changes. If you add a feature, include corresponding tests.
- Ensure your code passes all existing tests and does not introduce regressions.

### 🚧 **No Unnecessary Complexity**
Please avoid adding overly complex solutions. We value solutions that are easy to maintain, extend, and understand.

## Branch Structure

To streamline development, all work should be done on the **dev** branch. 

1. **dev**  
   Use this branch for all development, including new features, bug fixes, optimizations, and enhancements.

To create and push the `dev` branch:
```bash
git checkout -b dev && git push -u origin dev
```

## Code of Conduct

Be kind and respectful to everyone. Any harassment, hate speech, or inappropriate behavior will not be tolerated.

## Submitting Issues

If you discover bugs, issues, or have suggestions, feel free to submit an issue in the [GitHub Issues](https://github.com/cyberforgex/crust-trust/issues). Please ensure the issue is clear, with steps to reproduce if applicable.

## Pull Request Process

1. Ensure any new code has been tested locally.
2. Ensure the project builds cleanly using:
   ```bash
   cargo check
   ```
3. Submit a Pull Request (PR) from your fork to the `dev` branch.
4. All pull requests must be reviewed before they are merged.

## Development Workflow

When working on the project:

1. **Start from the latest `dev` branch**:
   ```bash
   git checkout dev
   git pull origin dev
   ```

2. **Isolate your changes** to the feature or bug fix you're working on.
   
3. **Run tests** before submitting a pull request:
   ```bash
   cargo test
   ```

4. If you add any new files, ensure they fit within the project’s structure and follow naming conventions.

## License

By contributing to **Crust-Trust**, you agree that your contributions will be licensed under the MIT license.

---

Thank you for your contributions, and we look forward to working with you to improve **Crust-Trust**! 💻✨
