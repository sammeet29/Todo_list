use dialoguer::Input;

fn main() {
  loop {
    
    let command: String = Input::new()
      .with_prompt("Enter a command:")
      .interact_text()
      .unwrap();

    println!("Command: {}", command);

    let mut exit = false;

    match command.as_str() {
      "exit" => exit = true,
      _ => ,
    }

    // if exit 
    // {
    break;
    // }
  }
}
