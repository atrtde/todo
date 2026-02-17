use crate::todo::read_todos;

use anyhow::Result;

pub fn run() -> Result<()> {
    let todos = read_todos()?;
    if todos.is_empty() {
        println!("No tasks found.");
        return Ok(());
    }

    for todo in &todos {
        let status = if todo.completed { "" } else { "❌" };
        println!("[{}] {} {}", todo.id, status, todo.task);
    }

    Ok(())
}
