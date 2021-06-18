use mysql::*;
use mysql::prelude::*;

#[derive(Debug, PartialEq, Eq)]
struct ListItems {
	id: i32,
	tasks: String
}

fn main() {
	let url = "mysql://root:Spm!wel84@localhost:1401/TodoList"
	let pool = Pool::new(url)?;
	let mut conn = pool.get_conn()?;
}
