use std::io;

struct Todo {
    name: String,
    is_complete: bool,
}

enum Action {
    List,
    Complete,
    Delete,
    Add,
    Quit,
}

impl Action {
    fn get_command(&self) -> char {
        match self {
            Action::List => 'L',
            Action::Complete => 'C',
            Action::Delete => 'D',
            Action::Add => 'A',
            Action::Quit => 'Q',
        }
    }

    fn from_char(c: char) -> Option<Action> {
        Action::all()
            .into_iter()
            .find(|action| action.get_command() == c.to_ascii_uppercase())
    }

    fn all() -> [Action; 5] {
        [
            Action::List,
            Action::Complete,
            Action::Delete,
            Action::Add,
            Action::Quit,
        ]
    }
}

struct TodoList {
    todos: Vec<Todo>,
}

impl TodoList {
    fn new() -> TodoList {
        Self { todos: vec![] }
    }
    fn list(&self) {
        self.todos.iter().enumerate().for_each(|(i, todo)| {
            println!(
                "{}. [{}] {}",
                i + 1,
                if todo.is_complete { "✓" } else { " " },
                todo.name
            )
        });
    }
    fn add(&mut self, name: String) {
        self.todos.push(Todo {
            name,
            is_complete: false,
        });
    }
    fn delete(&mut self, num: usize) -> bool {
        if num >= 1 && num <= self.todos.len() {
            self.todos.remove(num - 1);
            true
        } else {
            false
        }
    }
    fn complete(&mut self, num: usize) -> bool {
        match num.checked_sub(1).and_then(|i| self.todos.get_mut(i)) {
            Some(todo) => {
                todo.is_complete = !todo.is_complete;
                true
            }
            None => false,
        }
    }
}

fn read_line() -> Option<String> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(0) | Err(_) => None,
        Ok(_) => Some(input.trim().to_string()),
    }
}

fn read_number() -> Option<usize> {
    println!("Please enter the number of a todo: ");
    let input = read_line()?;
    match input.parse() {
        Ok(number) => Some(number),
        Err(_) => {
            println!("Invalid number {input}");
            None
        }
    }
}

fn main() {
    println!("Welcome to your todo app!");

    let mut todo_list = TodoList::new();

    loop {
        println!(
            "Please type a command ({}): ",
            Action::all()
                .iter()
                .map(|action| action.get_command().to_string())
                .collect::<Vec<String>>()
                .join(", ")
        );

        // End of input (e.g. Ctrl+D) quits
        let Some(input) = read_line() else { break };
        let Some(ch) = input.chars().next() else {
            continue;
        };

        let Some(action) = Action::from_char(ch) else {
            println!("Invalid command {ch}");
            continue;
        };

        match action {
            Action::List => todo_list.list(),
            Action::Complete => {
                if let Some(number) = read_number() {
                    if !todo_list.complete(number) {
                        println!("No todo with number {number}");
                    }
                }
            }
            Action::Add => {
                println!("Please enter the name of the new todo: ");
                let Some(name) = read_line() else { break };

                todo_list.add(name);
                todo_list.list();
            }
            Action::Delete => {
                if let Some(number) = read_number() {
                    if !todo_list.delete(number) {
                        println!("No todo with number {number}");
                    }
                }
            }
            Action::Quit => break,
        }
    }
}
