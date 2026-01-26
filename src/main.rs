mod command;

use dialoguer::Input;
use command::Command;

fn print_help(){
	println!("Available commands:");
	println!("  add <item> - Add an item to the todo list");
	println!("  remove (r) <item> - Remove an item from the todo list");
	println!("  removeIndex (ri) <index> - Remove an item from the todo list by index");
	println!("  exit - Exit the program");
}

fn remove_items(todo_list: &mut Vec<String>, items: &Vec<String>) {
	for item in items {
		if let Some(pos) = todo_list.iter().position(|x| x == item) {
			todo_list.remove(pos);
			println!("Removed item '{}' from the todo list.", item);
		} else {
			println!("Item '{}' not found in the todo list.", item);
		}
	}
}

fn remove_item_by_index(todo_list: &mut Vec<String>, item_indexes: u32) {
	let index = item_indexes as usize;
	if index == 0 || index > todo_list.len() {
		println!("Error: Index {} is out of bounds.", item_indexes);
	} else {
		let removed_item = todo_list.remove(index - 1);
		println!("Removed item '{}' from the todo list.", removed_item);
	}
}

fn main() {
	let mut todo_list: Vec<String> = Vec::new();

	loop {
		let mut index:u32 = 1;
		for list_item in &todo_list {
			println!("  {} {}", index, list_item);
			index += 1;
		}

		let command_input: String = Input::new()
			.with_prompt("Enter a command")
			.interact_text()
			.unwrap();

		let input_command = Command::from(command_input);
		match input_command {
			Command::Exit => break,
			Command::Add(items) => {
				if items.is_empty() {
					println!("Error: 'add' command requires at least one item to add.");
				} else {
					println!("Adding {} item(s) to the list.", items.len());
					todo_list.extend(items);
				}
			}
			Command::Remove(items) => {
				remove_items(&mut todo_list, &items);
			}
			Command::RemoveIndex(item_indexes)=> remove_item_by_index(&mut todo_list, item_indexes),
			Command::Help => print_help(),
			Command::Unknown(input) => {
				println!("Unknown command: '{}'", input);
				print_help()
			}
		}
	}
}
