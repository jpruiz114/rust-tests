use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::{BufReader, BufWriter},
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Task {
    id: u64,
    text: String,
    done: bool,
    created_at: u64, // epoch seconds
}

#[derive(Parser)]
#[command(name = "todo", version, about = "Tiny JSON-backed to-do CLI in Rust")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new task
    Add { text: String },

    /// List tasks (use --all to include completed)
    List {
        /// Show completed too
        #[arg(long)]
        all: bool,
    },

    /// Mark a task as done by id
    Done { id: u64 },

    /// Remove a task by id
    Rm { id: u64 },

    /// Clear all completed tasks
    ClearDone,
}

fn data_file() -> Result<PathBuf> {
    let mut p = home_dir().context("Could not find home directory")?;
    p.push(".todo-rs");
    fs::create_dir_all(&p).ok(); // ensure dir exists
    p.push("tasks.json");
    Ok(p)
}

fn load_tasks() -> Result<Vec<Task>> {
    let path = data_file()?;
    if !path.exists() {
        return Ok(vec![]);
    }
    let f = File::open(&path).with_context(|| format!("Opening {}", path.display()))?;
    let r = BufReader::new(f);
    let tasks: Vec<Task> = serde_json::from_reader(r).with_context(|| "Parsing JSON")?;
    Ok(tasks)
}

fn save_tasks(tasks: &[Task]) -> Result<()> {
    let path = data_file()?;
    let f = File::create(&path).with_context(|| format!("Creating {}", path.display()))?;
    let w = BufWriter::new(f);
    serde_json::to_writer_pretty(w, tasks).with_context(|| "Writing JSON")?;
    Ok(())
}

fn now_epoch() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn next_id(tasks: &[Task]) -> u64 {
    tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut tasks = load_tasks()?;

    match cli.command {
        Commands::Add { text } => {
            let t = Task {
                id: next_id(&tasks),
                text,
                done: false,
                created_at: now_epoch(),
            };
            tasks.push(t);
            save_tasks(&tasks)?;
            println!("Added ✅");
        }
        Commands::List { all } => {
            let mut shown = 0usize;
            for t in tasks.iter().filter(|t| all || !t.done) {
                let status = if t.done { "✓" } else { " " };
                println!("[{}] {:>3}  {}", status, t.id, t.text);
                shown += 1;
            }
            if shown == 0 {
                println!("(no tasks{})", if all { "" } else { " — try --all" });
            }
        }
        Commands::Done { id } => {
            if let Some(t) = tasks.iter_mut().find(|t| t.id == id) {
                t.done = true;
                save_tasks(&tasks)?;
                println!("Marked as done: {}", id);
            } else {
                println!("No task with id {}", id);
            }
        }
        Commands::Rm { id } => {
            let before = tasks.len();
            tasks.retain(|t| t.id != id);
            if tasks.len() < before {
                save_tasks(&tasks)?;
                println!("Removed {}", id);
            } else {
                println!("No task with id {}", id);
            }
        }
        Commands::ClearDone => {
            let before = tasks.len();
            tasks.retain(|t| !t.done);
            let removed = before - tasks.len();
            save_tasks(&tasks)?;
            println!("Removed {} completed task(s)", removed);
        }
    }

    Ok(())
}
