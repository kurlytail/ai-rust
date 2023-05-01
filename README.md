# AI Rust - A Concurrent Agent Registry System

AI Rust is a concurrent agent registry system built with Rust. It allows registering and running different types of agents concurrently, including Local and OpenAI agents.

## Features

- **Agent Registry**: Central registry to register and manage different types of agents.
- **Concurrent Execution**: Agents are run concurrently utilizing async functionality in Rust.
- **OpenAI Agents**: It has built-in support for creating OpenAI agents that can communicate with the OpenAI GPT-3 API.
- **Local Agents**: It supports local agents for tasks that do not require interaction with the OpenAI API.
- **Extensibility**: The system is designed to be extensible, so new types of agents can be added easily.

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

- Rust: You can download Rust from the official website [here](https://www.rust-lang.org/tools/install).

### Installing

1. Clone this repository
```bash
git clone https://github.com/yourusername/ai-rust.git
```
2. Build the project
```bash
cargo build
```
3. Run the project
```bash
cargo run
```

## Usage

You can register new agents by modifying the `main.rs` file. Here's an example:

```rust
agent_registry.register(
    "OpenAI",
    "OpenAI Agent",
    vec![
        String::from("Translate 'Hello, world!' to French."),
        String::from("What is the weather like today?"),
    ],
);
```

You can also run all the registered agents using the `run_all` function:

```rust
agent_registry.run_all().await;
```

## Contributing

Please read [CONTRIBUTING.md](https://github.com/kurlytail/ai-rust/CONTRIBUTING.md) for details on our code of conduct, and the process for submitting pull requests to us.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.
