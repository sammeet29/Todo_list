
#[derive(Debug)]
pub enum Command {
	Add(Vec<String>),
	CheckIndex(u32),
	Check(String),
	Exit,
	Help,
	UncheckIndex(u32),
	Uncheck(String),
	Remove(Vec<String>),
	RemoveIndex(u32),
	Unknown(String),
}

enum CommandError {
	InvalidArgumentCounts,
	NotValidNumber,
}

fn get_number(args:Vec<String>) -> Result< u32, CommandError> {
	if args.len() != 1 {
		Err(CommandError::InvalidArgumentCounts)
	}else{
		let result = args[0].parse::<u32>();
		match result {
			Ok(index) => Ok(index),
			Err(_) => Err(CommandError::NotValidNumber),
		}
	}
}

impl From<String> for Command {
	fn from(s: String) -> Self {
		let trimmed = s.trim();

		if trimmed.is_empty() {
			return Command::Unknown(trimmed.to_string());
		}

		// Split by whitespace
		let parts: Vec<&str> = trimmed.split_whitespace().collect();

		// The 1st argument is command
		let cmd = parts[0].to_lowercase();

		// Collect all other strings as argument to the command
		let args: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();

		match cmd.as_str() {
			"add" | "a" => Command::Add(args),
			"check" | "c" => {
				Command::Check(parts[1].to_string())
			}
			"checkIndex" | "ci" => {
				match get_number(args) {
					Ok(number) => Command::CheckIndex(number),
					Err(_) => Command::Unknown("Invalid Argument".to_string()),
				}
			}
			"exit" | "e" => Command::Exit,
			"remove" | "r" => Command::Remove(args),
			"removeIndex" | "ri" => {
				match get_number(args) {
					Ok(number) => Command::RemoveIndex(number),
					Err(_) => Command::Unknown("Invalid Argument".to_string()),
				}
			},
			"help" | "h" => Command::Help,
			"uncheck" | "u" => {
				Command::Uncheck(parts[1].to_string())
			}
			"uncheckIndex" | "ui" => {
				match get_number(args) {
					Ok(number) => Command::UncheckIndex(number),
					Err(_) => Command::Unknown("Invalid Argument".to_string())
				}
			},

			_ => Command::Unknown(trimmed.to_string()),
		}
	}
}
