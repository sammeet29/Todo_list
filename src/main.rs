mod command;
mod item;

use dialoguer::Input;
use command::Command;
use item::Item;

// Todo: move this to command
fn print_help(){
	println!("Available commands:");
	println!("  add <item> - Add an item to the todo list");
	println!("  remove (r) <item> - Remove an item from the todo list");
	println!("  removeIndex (ri) <index> - Remove an item from the todo list by index");
	println!("  check (c) <item> - Mark an item as completed");
	println!("  checkIndex (ci) <index> - Mark an item as completed by index");
	println!("  uncheck (u) <item> - Mark an item as not completed");
	println!("  uncheckIndex (ui) <index> - Mark an item as not completed by index");
	println!("  exit - Exit the program");
}

// Remove one or more items from the list.
fn remove_items(todo_list: &mut Vec<Item>, items: &Vec<String>) {
	for item in items {
		if let Some(pos) = todo_list.iter().position(|x| x.has_value(item)) {
			todo_list.remove(pos);
			println!("Removed item '{}' from the todo list.", item);
		} else {
			println!("Item '{}' not found in the todo list.", item);
		}
	}
}

// Remove the given item index from the list.
fn remove_item_by_index(todo_list: &mut Vec<Item>, item_indexes: u32) {
	let index = item_indexes as usize;
	if index == 0 || index > todo_list.len() {
		println!("Error: Index {} is out of bounds.", item_indexes);
	} else {
		let removed_item = todo_list.remove(index - 1);
		println!("Removed item '{}' from the todo list.", removed_item);
	}
}

// Add one or more items to the list
fn add_to_list(todo_list: &mut Vec<Item>, items_to_add: Vec<String>)
{
	for item_value in items_to_add{
		use item::new_item;
		todo_list.push(new_item(item_value));
	}
}

fn check_index(todo_list: &mut Vec<Item>, index:u32)
{
	let index = index as usize;
	if index == 0 || index > todo_list.len() {
		println!("Error: Index {} is out of bounds", index)
	} else {
		todo_list[index - 1].check()
	}
}

fn check_item(todo_list: &mut Vec<Item>, item_value:String)
{
	for item in todo_list {
		if item.has_value(&item_value) {
			item.check()
		}
	}
}

fn uncheck_index(todo_list: &mut Vec<Item>, index:u32)
{
	let index = index as usize;
	if index == 0 || index > todo_list.len() {
		println!("Error: Index {} is out of bounds", index)
	} else {
		todo_list[index - 1].uncheck()
	}
}

fn uncheck_item(todo_list: &mut Vec<Item>, item_value:String)
{
	for item in todo_list {
		if item.has_value(&item_value) {
			item.uncheck();
			return;
		}
	}
}


fn main() {
	let mut todo_list: Vec<Item> = Vec::new();

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
			Command::Add(items) => {
                if items.is_empty() {
					println!("Error: 'add' command requires at least one item to add.");
				} else {
					println!("Adding {} item(s) to the list.", items.len());
					add_to_list(&mut todo_list, items)
				}
			}
			Command::CheckIndex(item_index) => check_index(&mut todo_list, item_index),
			Command::Check(item_value) => check_item(&mut todo_list, item_value),
			Command::Exit => break,
			Command::Remove(items) => {
				remove_items(&mut todo_list, &items);
			}
			Command::RemoveIndex(item_index)=> remove_item_by_index(&mut todo_list, item_index),
			Command::Help => print_help(),
            Command::Uncheck(item_value) => uncheck_item(&mut todo_list, item_value),
			Command::UncheckIndex(item_index) => uncheck_index(&mut todo_list, item_index),
			Command::Unknown(input) => {
				println!("Unknown command: '{}'", input);
				print_help()
			}
		}
	}
}
