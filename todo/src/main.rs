use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

struct TodoApp {
    todos: HashMap<u32, Todo>,
    next_id: u32,
}

impl TodoApp {
    fn new() -> Self {
        TodoApp {
            todos: HashMap::new(),
            next_id: 1,
        }
    }

    fn add_todo(&mut self, title: String) {
        let todo = Todo {
            id: self.next_id,
            title,
            completed: false,
        };
        self.todos.insert(self.next_id, todo);
        self.next_id += 1;
    }

    fn get_todo(&self, id: u32) -> Option<&Todo> {
        self.todos.get(&id)
    }

    fn update_todo(&mut self, id: u32, title: Option<String>, completed: Option<bool>) -> bool {
        if let Some(todo) = self.todos.get_mut(&id) {
            if let Some(t) = title {
                todo.title = t;
            }
            if let Some(c) = completed {
                todo.completed = c;
            }
            return true;
        }
        false
    }

    fn delete_todo(&mut self, id: u32) -> bool {
        self.todos.remove(&id).is_some()
    }

    fn list_todos(&self) {
        for todo in self.todos.values() {
            println!("{:?}", todo);
        }
    }
}

fn main() {}
