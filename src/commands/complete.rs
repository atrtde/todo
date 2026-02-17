use anyhow::Result;
use inquire::MultiSelect;

use crate::todo::{read_todos, write_todos};

pub fn run() -> Result<()> {
    let mut todos = read_todos()?;
    let incomplete: Vec<_> = todos.iter().filter(|t| !t.completed).collect();

    if incomplete.is_empty() {
        println!("All tasks are already completed!");
        return Ok(());
    }

    let options: Vec<String> = incomplete
        .iter()
        .map(|t| format!("[{}] {}", t.id, t.task))
        .collect();

    let selected = MultiSelect::new("Select tasks to mark as completed:", options).prompt()?;
    let ids_to_complete: Vec<usize> = selected
        .iter()
        .map(|s| s.split(']').next().unwrap()[1..].parse::<usize>().unwrap())
        .collect();

    for todo in &mut todos {
        if ids_to_complete.contains(&todo.id) {
            todo.completed = true;
        }
    }
    write_todos(&todos)?;
    println!("Selected tasks marked as completed!");

    Ok(())
}
