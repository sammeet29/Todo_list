
#[derive(Debug)]
pub enum Command {
	Exit,
	Add(Vec<String>),
	Remove(Vec<String>),
	RemoveIndex(u32),
	Help,
	Unknown(String),
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
		// Collect all other strings as argumnet to the command
		let args: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();

		match cmd.as_str() {
			"exit" | "e" => Command::Exit,
			"add" | "a" => Command::Add(args),
			"remove" | "r" => Command::Remove(args),
			"removeIndex" | "ri" => {
				if args.len() != 1 {
					return Command::Unknown(trimmed.to_string());
				}
				match args[0].parse::<u32>() {
					Ok(index) => Command::RemoveIndex(index),
					Err(_) => Command::Unknown(trimmed.to_string()),
				}
			},
			"help" | "h" => Command::Help,
			_ => Command::Unknown(trimmed.to_string()),
		}
	}
}