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
		for (index, item) in self.list.iter().enumerate() {
			println!("{} [{}] - {}", index, item.status, item.name);
		}
	}

	fn mark_done(&mut self, index:usize) {
		if self.list[index].status == ' ' {
			self.list[index].status = 'x';
		}
		else {
			self.list[index].status = ' ';
		}
	}

	fn remove_task(&mut self, index:usize) {
		self.list.remove(index);
	}
}

enum Command {
	Get,
	Add(String),
	Done(usize),
	Remove(usize)
}

fn main() {
    let args: Vec<String> = env::args().collect();
	
	let command = match args[1].as_str() {
		"get" => Command::Get,
		"add" => Command::Add(args[2].clone()),
		"done" => Command::Done(args[2].parse().expect("error converting to integert")),
		"remove" => Command::Remove(args[2].parse().expect("error deleting task")),
		_ => panic!("You must provide a proper command")
	};
	
	// our actual list of tasks
	let mut todo_list = TodoList::new();
	
	// creating the first item in the todo list
	todo_list.add_to_list("This is the first item".to_string());
	todo_list.add_to_list("This is a second item".to_string());
	todo_list.mark_done(1);

	match command {
		Command::Get  => todo_list.print(),
		Command::Add(task) => {
			todo_list.add_to_list(task.to_string());
			todo_list.print();
		},
		Command::Done(index) => {
			todo_list.mark_done(index);
			todo_list.print();
		},
		Command::Remove(index) => {
			todo_list.remove_task(index);
			todo_list.print();
		}
	}
}
