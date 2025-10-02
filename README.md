# todo-rs

A simple command-line todo application written in Rust. This tool allows you to manage your tasks from the terminal with persistent storage using JSON.

## Features

- Add new tasks
- List tasks (with option to show completed tasks)
- Mark tasks as done
- Remove tasks
- Clear all completed tasks
- Persistent storage in JSON format
- Data stored in `~/.todo-rs/tasks.json`

## Installation

### Prerequisites

- Rust 1.70+ (2021 edition)
- Cargo

### Building from Source

1. Clone the repository:
```bash
git clone <repository-url>
cd todo-rs
```

2. Build the project:
```bash
cargo build --release
```

3. Run the executable:
```bash
./target/release/todo-rs
```

Or install it globally:
```bash
cargo install --path .
```

## Usage

The application uses subcommands to manage tasks:

### Add a new task
```bash
todo-rs add "Your task description here"
```

### List tasks
```bash
# List only pending tasks
todo-rs list

# List all tasks (including completed)
todo-rs list --all
```

### Mark a task as done
```bash
todo-rs done <task-id>
```

### Remove a task
```bash
todo-rs rm <task-id>
```

### Clear completed tasks
```bash
todo-rs clear-done
```

### Get help
```bash
todo-rs --help
```

## Data Storage

Tasks are stored in JSON format at `~/.todo-rs/tasks.json`. The file is automatically created when you add your first task. Each task includes:

- `id`: Unique identifier
- `text`: Task description
- `done`: Completion status
- `created_at`: Unix timestamp of creation

## Dependencies

- `clap`: Command-line argument parsing
- `serde`: Serialization/deserialization
- `serde_json`: JSON support
- `dirs`: Home directory detection
- `anyhow`: Error handling

## Development

### Running in Development Mode

```bash
cargo run -- <command>
```

For example:
```bash
cargo run -- add "Test task"
cargo run -- list
```

### Building for Release

```bash
cargo build --release
```

## License

This project is open source. Please check the license file for details.
