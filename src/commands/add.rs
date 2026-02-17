use inquire::Text;

use crate::todo::{TodoItem, read_todos, write_todos};

use anyhow::Result;

pub fn run(task: Option<String>) -> Result<()> {
    let task = match task {
        Some(t) => t,
        None => Text::new("What task would you like to add?").prompt()?,
    };

    let mut todos = read_todos()?;
    let next_id = todos.last().map(|t| t.id + 1).unwrap_or(1);
    todos.push(TodoItem {
        id: next_id,
        task,
        completed: false,
    });
    write_todos(&todos)?;
    println!("Task added!");

    Ok(())
}
