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
    fn get_command(&self) -> String {
        match self {
            Action::List => String::from("L"),
            Action::Complete => String::from("C"),
            Action::Delete => String::from("D"),
            Action::Add => String::from("A"),
            Action::Quit => String::from("Q"),
        }
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
                if todo.is_complete { "✓" } else { "X" },
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
    fn delete(&mut self, num: u8) {
        self.todos.remove((num + 1).into());
    }
    fn complete(&mut self, num: u8) {
        for (i, todo) in self.todos.iter_mut().enumerate() {
            if i - 1 == num.into() {
                todo.is_complete = !todo.is_complete;
            }
        }
    }
}

fn main() {
    println!("Welcome to your todo app!");

    let mut todo_list = TodoList::new();

    loop {
        let mut input = String::new();
        println!(
            "Please type a command ({}): ",
            Action::all()
                .iter()
                .map(|action| action.get_command())
                .collect::<Vec<String>>()
                .join(", ")
        );
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read command");

        let ch = input.trim().chars().next().unwrap();

        match ch {
            'L' => todo_list.list(),
            'C' => {
                println!("Please enter the number of a todo: ");
                let mut input = String::new();

                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read number");

                let number: u8 = input.parse().expect("Invalid number");

                todo_list.complete(number);
            }
            'A' => {
                println!("Please enter the name of the new todo: ");
                let mut input = String::new();

                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read number");

                todo_list.add(input);
                todo_list.list();
            }
            'D' => {
                println!("Please enter the number of a todo: ");
                let mut input = String::new();

                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read number");

                let number: u8 = input.parse().expect("Invalid number");

                todo_list.delete(number);
            }
            'Q' => break,
            _ => {
                println!("Invalid command {ch}");
                continue;
            }
        }
    }
}
