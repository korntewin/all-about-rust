# 🦀 All About Rust

Welcome to my Rust playground! 🎮 This is where I experiment, learn, and build cool stuff with Rust using a Cargo workspace setup as monorepo!

## 🏗️ What's This All About?

This repository serves as my personal Rust laboratory where I explore different aspects of the language, try out new crates, implement algorithms, and build opensource projects that I am interested 🔥. Everything is organized in a very cool [Cargo Workspaces] feature which making monorepo easier to do!

## 📁 Current Projects

Here's what's currently in this workspace:

### 🧮 `algorithms/`
My algorithmic practicing with Rust.
- The purpose of this project is to practicing Rust basic syntax and data structures.
- More algorithms coming soon as I dive deeper into data structures and problem-solving patterns

### 🤖 `local-llm-comparison/`
A project to compare local LLM inference engines! 
- **LlamaEdge vs. Ollama vs. LMStudio** performance benchmarking
- Load testing with concurrent users using `Goose`, Rust load testing framework
- Running on MacBook Pro M4 Max 💪
- Uses Llama-3.2-3B-Instruct model for testing

## 🚀 Getting Started

Since this is a Cargo workspace, you can:

```bash
# Run specific projects
cargo run --bin twosum

# Run tests across the workspace
cargo test

# For LLM local comparison, please follow the project README for more details
```

## 🤝 Contributing

This is my personal learning playground, but if you found something interesting or have suggestions, feel free to open an issue or start a discussion! Learning is always better with friends 🎓

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details. 

---

*Happy coding! 🦀✨*

[Cargo Workspaces]: https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html