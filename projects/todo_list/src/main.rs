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

fn main() {
    let args: Vec<String> = env::args().collect();
	
	let cmd = &args[1];
	// creating the first item in the todo list
	let todo_item_1 = TodoItem::new("This is a test".to_string());
	let todo_item_2 = TodoItem::new("Here is another one".to_string());
	
	// our actual list of tasks
	let todo_list = vec![todo_item_1, todo_item_2];

	if cmd == "get" {
		println!("Get command called");
		for item in todo_list {
			println!("[{}] - {}", item.status, item.name); 
		}
	}
}
