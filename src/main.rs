mod command;
mod task_list;

use dialoguer::Input;
use command::Command;
use task_list::TaskList;

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

fn main() {
	let mut todo_list: TaskList = TaskList::new();

	loop {

		todo_list.print();

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
					todo_list.add(items);
				}
			}
			Command::CheckIndex(item_index) => todo_list.check_index(item_index),
			Command::Check(item_value) => todo_list.check_item(item_value),
			Command::Exit => break,
			Command::Remove(items) => {
				todo_list.remove_items(items);
			}
			Command::RemoveIndex(item_index)=> {
				todo_list.remove_index(item_index);
			}
			Command::Help => print_help(),
            Command::Uncheck(item_value) => todo_list.uncheck_item(item_value),
			Command::UncheckIndex(item_index) => {
				todo_list.uncheck_index(item_index);
			}
			Command::Unknown(input) => {
				println!("Unknown command: '{}'", input);
				print_help()
			}
		}
	}
}
