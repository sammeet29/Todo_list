mod command_parser;
mod task_list;

use dialoguer::Input;
use task_list::TaskList;
use command_parser::{ListCommand, parse_command};

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
		match input_command.unwrap().list_command {
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
