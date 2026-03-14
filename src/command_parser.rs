use clap::{Parser, Subcommand};

#[derive(Clone, Subcommand)]
pub enum ListCommand{
    #[command(visible_alias = "a")]
    Add{item:String},
    #[command(visible_alias = "r")]
    Remove {item: String},
    #[command(visible_alias = "ri")]
    RemoveIndex {item_index: usize},
    #[command(visible_alias = "c")]
    Check{ item: String },
    #[command(visible_alias = "ci")]
    CheckIndex { item_number: usize },
    #[command(visible_alias = "u")]
    Uncheck {item: String },
    #[command(visible_alias = "ui")]
    UncheckIndex { item_number: usize },
    Exit,
}

#[derive(Parser)]
// #[command(Version, about, long_about = None)]
pub struct Cli{
    #[command(subcommand)]
    list_command: ListCommand,
}

pub fn parse_command(input: String) -> Result<ListCommand, clap::error::Error>
{
    let input_vec= input.split_whitespace();
    let args = Cli::parse_from(input_vec);
    Ok(args.list_command)
}

#[cfg(test)]
mod tests{
    use super::*;    pub fn parse_command(input: String) -> Result<ListCommand, clap::error::Error> {
        // Clap expects the 1st name to be program name. Add a dummy name to iterator
        let args
            = std::iter::once("dummy").chain(input.split_whitespace());

        let cli = Cli::parse_from(args);
        Ok(cli.list_command)
    }

    struct ItemTestCase{
        command: String,
        expected_item: String,
    }

    struct IndexTestCase{
        command: String,
        expected_index: usize,
    }

    #[test]
    fn test_parse_add_command() {

        let tests: Vec<ItemTestCase> = vec![
            ItemTestCase{command: String::from("add milk"), expected_item: String::from("milk")},
            ItemTestCase{command: String::from("a milk"), expected_item: String::from("milk")},
        ];

        for test in tests {
            let result = parse_command(test.command);
            assert!(result.is_ok());
            match result.unwrap() {
                ListCommand::Add {item} => assert_eq!(item, test.expected_item),
                _ => panic!("Expected Add command"),
            }
        }
    }

    #[test]
    fn test_parse_remove_command() {

        let tests: Vec<ItemTestCase> = vec![
            ItemTestCase{command: String::from("remove milk"), expected_item: String::from("milk")},
            ItemTestCase{command: String::from("r milk"), expected_item: String::from("milk")},
        ];

        for test in tests{
            let result = parse_command(test.command);
            assert!(result.is_ok());
            match result.unwrap() {
                ListCommand::Remove { item } => assert_eq!(item, test.expected_item),
                _ => panic!("Expected remove command"),
            }
        }
    }

    #[test]
    fn test_remove_index()
    {
        let tests: Vec<IndexTestCase> = vec![
            IndexTestCase{command: String::from("remove-index 6"), expected_index: 6},
            IndexTestCase{command: String::from("ri 6"), expected_index: 6},
        ];

        for test in tests{
            let result = parse_command(test.command);
            assert!(result.is_ok());
            match result.unwrap() {
                ListCommand::RemoveIndex { item_index } => {
                    assert_eq!(item_index, test.expected_index);
                }
                _ => panic!("Expected remove index command"),
            }
        }
    }

    #[test]
    fn test_check()
    {
        let tests: Vec<ItemTestCase> = vec![
            ItemTestCase{command: String::from("check milk"), expected_item: String::from("milk")},
            ItemTestCase{command: String::from("c milk"), expected_item: String::from("milk")}
        ];

        for test in tests {
            let result = parse_command(test.command);
            assert!(result.is_ok());
            match result.unwrap() {
                ListCommand::Check{item} => assert_eq!(item, test.expected_item),
                _ => panic!("Expected Check command"),
            }
        }
    }

    #[test]
    fn test_parse_command_check_index()
    {
        let tests: Vec<IndexTestCase> = vec![
            IndexTestCase{command: String::from("check-index 1"), expected_index: 1},
            IndexTestCase{command: String::from("ci 1"), expected_index: 1},
        ];

        for test in tests{
            let result = parse_command(test.command);
            assert!(result.is_ok());
            match result.unwrap() {
                ListCommand::CheckIndex { item_number } => assert_eq!(item_number, test.expected_index),
                _ => panic!("Expected CheckIndex command")
            }
        }
    }

    #[test]
    fn test_parse_uncheck_command()
    {
        let tests: Vec<ItemTestCase> = vec![
            ItemTestCase{command: String::from("uncheck milk"), expected_item: String::from("milk")},
            ItemTestCase{command: String::from("u milk"), expected_item: String::from("milk")},
        ];

        for test in tests {
            let result = parse_command(test.command);
            assert!(result.is_ok());
            match result.unwrap() {
                ListCommand::Uncheck{item} => assert_eq!(item, test.expected_item),
                _ => panic!("Expected Check command"),
            }
        }
    }

    #[test]
    fn test_parse_uncheck_index()
    {
        let tests: Vec<IndexTestCase> = vec![
            IndexTestCase{command: String::from("uncheck-index 3"), expected_index: 3},
            IndexTestCase{command: String::from("ui 3"), expected_index: 3},
        ];

        for test in tests{
            let result = parse_command(test.command);
            assert!(result.is_ok());
            match result.unwrap() {
                ListCommand::UncheckIndex { item_number } => assert_eq!(item_number, test.expected_index),
                _ => panic!("Expected Uncheck index command")
            }
        }
    }

    #[test]
    fn test_parse_exit_command() {
        let result = parse_command(String::from("exit"));
        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), ListCommand::Exit));
    }

    // #[test]
    // fn test_parse_invalid_command() {
    //     let result = parse_commands("invalid command");
    //     assert!(result.is_err());
    // }

    // #[test]
    // fn test_parse_missing_arguments() {
    //     let result = parse_commands("add");
    //     assert!(result.is_err());
    // }

    // // Test all variants exist
    // #[test]
    // fn test_all_list_command_variants() {
    //     let _add = ListCommand::Add { item: "test".to_string() };
    //     let _remove = ListCommand::Remove { item: "test".to_string() };
    //     let _check_item = ListCommand::CheckItem { item: "test".to_string() };
    //     let _check = ListCommand::Check { item_number: 1 };
    //     let _uncheck = ListCommand::Uncheck { item_number: 1 };
    //     let _uncheck_item = ListCommand::UncheckItem { item: "test".to_string() };
    //     let _exit = ListCommand::Exit;

    //     // If this compiles, all variants exist
    //     assert!(true);
    // }

    // // Parameterized testing approach
    // #[test]
    // fn test_command_parsing_parameterized() {
    //     let test_cases = vec![
    //         ("add apple", ListCommand::Add { item: "apple".to_string() }),
    //         ("remove banana", ListCommand::Remove { item: "banana".to_string() }),
    //         ("check-item orange", ListCommand::CheckItem { item: "orange".to_string() }),
    //         ("check 1", ListCommand::Check { item_number: 1 }),
    //         ("uncheck 2", ListCommand::Uncheck { item_number: 2 }),
    //         ("uncheck-item grape", ListCommand::UncheckItem { item: "grape".to_string() }),
    //         ("exit", ListCommand::Exit),
    //     ];

    //     for (input, expected) in test_cases {
    //         let result = parse_commands(input);
    //         assert!(result.is_ok(), "Failed to parse: {}", input);
    //         assert_eq!(format!("{:?}", result.unwrap()), format!("{:?}", expected),
    //                   "Failed for input: {}", input);
    //     }
    // }
}
