# Getting Started with Rust and Rocket - A Beginner's Guide

## 1️⃣ Title and Objective

**Title** - Building a minimal REST API in Rust using the Rocket web framework.

**Objectives**

- Learn Rust + Rocket using generative AI prompts
- Create a minimal runnable API that returns JSON
- Document the process for beginners to reproduce

**Why Rust?** 🤔

- Memory-safe systems programming without garbage collection
- Growing adoption in performance-critical applications
- Enables developers to build fast, secure and reliable web services

**End Goal:** Create a simple rust-rocket API with three endpoints: **Greetings**, **Congratulations**, and **Status**

---

## 2️⃣ Technology Overview

### What is Rust?

Rust is a modern systems programming language focused on **safety, speed, and concurrency**. It enforces memory safety without a garbage collector using its ownership model. Ideal for CLI tools, web servers, embedded systems, and high-performance applications.

### What is Rocket?

Rocket is a high-level web framework for Rust that makes it simple to write fast, type-safe, secure web applications with incredible usability, productivity and performance. It provides ergonomic routing and request handling while integrating seamlessly with Rust's type system.

### Real-world Example

Realtime chat application by Eleftheria Batsou: [GitHub Repository](https://github.com/EleftheriaBatsou/chat-app-rocket-rust/tree/main)

---

## 3️⃣ System Requirements

- **OS:** Linux, MacOS, or Windows (WSL)
- **Tools:**
  - Rustup (for Rust toolchain)
  - Cargo (Rust package manager)
  - Curl (for testing)
- **Editor:** VS Code or any text editor

---

## 4️⃣ Installation and Setup

### Clone This Repository

```bash
git clone https://github.com/Marion-saru/rust-rocket-beginner-toolkit.git
cd rust-rocket-beginner-toolkit/rust-rocket-api
```

### Or Start From Scratch

#### Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

#### Verify Installation

```bash
rustup show
rustc --version
cargo --version
```

#### Create New Project

```bash
cargo new rust-rocket-api
cd rust-rocket-api
```

#### Add Rocket Dependency

```bash
cargo install cargo-edit
cargo add rocket
```

**Alternative:** Manually edit `Cargo.toml`:

```toml
[dependencies]
rocket = "0.5.1"
```

#### Run the Project

```bash
cargo run
```

---

## 5️⃣ API Endpoints

The API includes three endpoints:

| Method | Path        | Description             | Response                                    |
| ------ | ----------- | ----------------------- | ------------------------------------------- |
| GET    | `/`         | JSON greeting           | `{ "message": "Hello from Rust!" }`         |
| GET    | `/congrats` | Congratulations message | `{ "message": "Wow! Well done Marion..." }` |
| GET    | `/health`   | Health check            | `{ "status": "ok" }`                        |

### Testing with Curl

```bash
curl localhost:8000/
curl localhost:8000/congrats
curl localhost:8000/health
```

See `TOOLKIT.md` code screenshots and detailed outputs.

---

## 6️⃣ Learning Approach with AI

This project was built using AI-assisted learning with structured prompts:

### Key Prompts Used:

1. **Conceptual Understanding**: Compared Rust philosophy with Python/Java to understand ownership, memory safety, and compile-time guarantees
2. **Language Fundamentals**: Learned Rust syntax, static typing, and function declarations
3. **Deepening Knowledge**: Focused on routing and request handling in Rocket framework

**Result:** By using existing Python/Java knowledge as anchors and iterating on AI prompts, achieved rapid understanding of Rust concepts and successfully built the API.

For full prompt journal and AI responses, see `TOOLKIT.md` .

---

## 7️⃣ Common Issues and Fixes

| Issue                                        | Solution                                                                                                |
| -------------------------------------------- | ------------------------------------------------------------------------------------------------------- |
| `rustup` not installed / `cargo` not found   | Install rustup from https://rustup.rs and restart shell                                                 |
| Rocket compilation errors (features/edition) | Ensure `edition = "2024"` in Cargo.toml; add `features = ["json"]`; run `rustup update`                 |
| Slow first build                             | Normal - first `cargo run` compiles dependencies; subsequent runs are faster                            |
| Port in use / cannot bind                    | Use different port: `ROCKET_PORT=8001 cargo run`                                                        |
| Dependency version mismatch                  | Run `cargo update` or match versions to latest Rocket release                                           |
| `serde::Serialize` not found                 | Add `serde = { version = "1.0", features = ["derive"] }` to Cargo.toml; use `rocket::serde::Serialize;` |

**Helpful Resources:**

- [Help](https://users.rust-lang.org/t/help-to-fix-serialize-not-implemented-for-std-error/104921) and [Help](https://medium.com/@murataslan1/serialization-and-deserialization-in-rust-a-comprehensive-guide-3eb249ae8ac6)

---

## 8️⃣ Project Structure

```
rust-rocket-beginner-toolkit/
├── README.md
├── TOOLKIT.md
├── Images/
│   └── [endpoint screenshots]
└── rust-rocket-api/
    ├── Cargo.toml
    └── src/
        └── main.rs
```

---

## 9️⃣ References

- [Rust Official Website](https://www.rust-lang.org)
- [Rocket Official Website](https://rocket.rs)
- [Serde Documentation](https://serde.rs)
- [Claude AI](https://claude.ai/new)

---

## Acknowledgement
### 👩‍💻 Author
Marion Saru Maghanga

### 🏫 Initiator
Moringa School in partnership with WeThinkCode : AI ACCESS PROGRAM

---

## 📝 Additional Documentation

For the **complete learning journey**, including:

- Detailed AI prompt journal with responses
- Step-by-step code explanations
- Screenshots of endpoints and outputs
- Reflective evaluation of AI assistance

**See:** `TOOLKIT.md`

