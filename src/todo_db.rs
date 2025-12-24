use crate::todo::Todo;
use serde::{ Deserialize, Serialize };
use std::fs::File;
use std::io::{ Read, Write };

#[derive(Serialize, Deserialize)]
pub struct TodoDb {
    todos: Vec<Todo>,
}

impl TodoDb {
    pub fn new() -> Self {
        Self { todos: Vec::new() }
    }

    pub fn add(&mut self, todo: Todo) {
        self.todos.push(todo);
    }

    pub fn list(&self) -> &Vec<Todo> {
        &self.todos
    }

    pub fn is_empty(&self) -> bool {
        self.todos.is_empty()
    }

    pub fn delete_by_index(&mut self, index: usize) -> bool {
        if index < self.todos.len() {
            self.todos.remove(index);
            true
        } else {
            false
        }
    }

    pub fn save(&self, filename: &str) -> std::io::Result<()> {
        let json = serde_json
            ::to_string_pretty(self)
            .map_err(|e| 
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                     e))?;
        let mut file = File::create(filename)?;
        file.write_all(json.as_bytes())?;
        println!("Todos salvas em '{}'", filename);
        Ok(())
    }

    pub fn load(filename: &str) -> std::io::Result<Self> {
        let mut file = File::open(filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        serde_json
            ::from_str(&contents)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    }
}
