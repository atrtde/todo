use anyhow::Result;
use inquire::MultiSelect;

use crate::todo::{read_todos, write_todos};

pub fn run() -> Result<()> {
    let mut todos = read_todos()?;
    if todos.is_empty() {
        println!("No tasks to remove.");
        return Ok(());
    }

    let options: Vec<String> = todos
        .iter()
        .map(|t| format!("[{}] {}", t.id, t.task))
        .collect();

    let selected = MultiSelect::new("Select tasks to remove:", options).prompt()?;
    let ids_to_remove: Vec<usize> = selected
        .iter()
        .map(|s| s.split(']').next().unwrap()[1..].parse::<usize>().unwrap())
        .collect();

    todos.retain(|t| !ids_to_remove.contains(&t.id));
    for (i, t) in todos.iter_mut().enumerate() {
        t.id = i + 1;
    }

    write_todos(&todos)?;
    println!("Task removed.");

    Ok(())
}
