mod command;
mod task_list;
mod command_parser;

use dialoguer::Input;
use task_list::TaskList;
use command_parser::{ListCommand, parse_command};

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

		let input_command = parse_command(command_input);
		if input_command.is_err()
		{
			continue;
		}
		match input_command.unwrap() {
			ListCommand::Add{item} => {
				todo_list.add_item(item);
			}
			ListCommand::CheckIndex{ item_number} => todo_list.check_index(item_number),
			ListCommand::Check{item} => todo_list.check_item(item),
			ListCommand::Remove { item } => {
				todo_list.remove_item(&item);
			}
			ListCommand::RemoveIndex { item_index} => {
				todo_list.remove_index(item_index);
			}
			// Command::Help => print_help(),
			ListCommand::Uncheck{item} => todo_list.uncheck_item(item),
			ListCommand::UncheckIndex{ item_number } => {
				todo_list.uncheck_index(item_number);
			}
			ListCommand::Exit => break
		}
	}
}
