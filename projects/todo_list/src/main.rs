use std::env;

// struct used to populate our todo list
struct TodoItem {
	name: String,
	status: char
}

impl TodoItem {
	fn new(name: String) -> TodoItem {
		return TodoItem{
			name: name,
			status: ' '
		}
	}
}

struct TodoList {
	list: Vec<TodoItem>
}

impl TodoList {
	fn new() -> TodoList {
		return TodoList { list: Vec::new()};
	}

	fn add_to_list(&mut self, name: String) {
		let item = TodoItem::new(name);
		self.list.push(item);
	}

	fn print(&self){
		for item in &self.list {
			println!("[{}] - {}", item.status, item.name);
		}
	}
}

enum Command {
	Get,
	Add(String)
}

fn main() {
    let args: Vec<String> = env::args().collect();
	
	let command = match args[1].as_str() {
		"get" => Command::Get,
		"add" => Command::Add(args[2].clone()),
		_ => panic!("You must provide a proper command")
	};
	
	// our actual list of tasks
	let mut todo_list = TodoList::new();
	
	// creating the first item in the todo list
	todo_list.add_to_list("This is the first item".to_string());
	todo_list.add_to_list("This is a second item".to_string());

	match command {
		Command::Get  => todo_list.print(),
		Command::Add(task) => {
			todo_list.add_to_list(task.to_string());
			todo_list.print();
		}
	}
}
