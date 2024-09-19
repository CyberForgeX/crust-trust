# 🦀 **Crust-Trust** 🚀

A blazing-fast tool to **auto-generate, manage, and optimize** Rust workspaces with ease! `Crust-Trust` empowers Rust developers to simplify their multi-crate projects with smooth **dependency handling**, **automatic version management**, **benchmarking**, and even a handy **crate dependency graph**. 

Build smarter, build faster, and let **Crust-Trust** handle the hassle. 🙌

## 🎯 **Why Crust-Trust?**

🔧 **Automatic Workspace Creation**: Seamlessly generate and manage multi-crate Rust projects in seconds!<br>
⚡ **Effortless Dependency Handling**: Automatic dependency management keeps your projects up-to-date.<br>
🔒 **Security Audits**: Keep your project dependencies secure and up-to-date.<br>
📊 **Benchmarking**: Measure performance with Rust's built-in benchmarking tools.<br>
🔍 **Dependency Graph**: Visualize and explore how your crates interact.<br>
🎯 **Optimize Crates**: Ensure smooth collaboration between crates with intelligent optimization.

---

## 🚀 **Getting Started**

### Installation

Simply clone this repository and build it using `cargo`!

```bash
git clone https://github.com/cyberforgex/crust-trust.git
cd crust-trust
cargo build --release
```

### Usage

```bash
./crust-trust <project-name> <crate:dependencies>
```

- **project-name**: The name of your Rust workspace.
- **crate:dependencies**: A comma-separated list of crates and their dependencies.

#### Example

```bash
./crust-trust my_workspace ui:druid,gtk,piet core:serde,mongodb storage:sysinfo,zfs
```

This will create a Rust workspace `my_workspace` with three crates (`ui`, `core`, `storage`), each with its own dependencies! 🚀

---

## 🛠️ **Features**

### 🌐 **Create and Manage Workspaces**  
Easily set up and manage Rust workspaces with minimal configuration.

### 🔧 **Automatic Dependency Management**  
Automatically updates your dependencies to the latest versions with no hassle.

### 🛡️ **Security Audits**  
Simulate auditing of your dependencies to ensure your project stays secure.

### 🏎️ **Benchmarking**  
Run Rust's `cargo bench` to evaluate performance in your workspace.

### 📊 **Dependency Graph**  
Visualize crate relationships in your workspace with `cargo metadata`.

---

## 🌟 **Show Your Support!**

If you find this project useful, consider buying me a coffee! ☕ It helps me maintain and improve `Crust-Trust` for the community.

<a href="https://www.buymeacoffee.com/cyberforgex" target="_blank"><img src="https://img.shields.io/badge/Buy%20Me%20A-Coffee-orange?style=for-the-badge&logo=buy-me-a-coffee" alt="Buy Me A Coffee"></a>

---

## 🚀 **Roadmap & Future Features**

Here's what's on the horizon for Crust-Trust:

- 🧠 **AI-driven code optimization**  
- 📈 **Automated project analysis**  
- 🛡️ **Advanced security scanning**

Stay tuned!

---

## 💬 **Contributing**

Contributions are always welcome! Check out the [contribution guidelines](CONTRIBUTING.md) for more info.

---

## 📄 **License**

This project is licensed under the MIT License – see the [LICENSE](LICENSE) file for details.

---

**Crust-Trust** – Making your Rust workspaces smarter, faster, and more secure! 🔥
