// Implements a To Do List
mod item;

use item::Item;

pub struct TaskList {
	items: Vec<Item>,
}

impl TaskList {
	pub fn new() -> Self {
		TaskList {
			items: Vec::new(),
		}
	}

	// To do:: Use &str instead of String
	pub fn add(&mut self, items_to_add: Vec<String>)
	{
		for item in items_to_add {
			self.items.push(item::new_item(item));
		}
	}

	pub fn remove_items(&mut self, items_to_remove: Vec<String>)
	{
		for item in &items_to_remove {
			if let Some(pos) = self.items.iter().position(|x| x.has_value(item)){
				self.items.remove(pos);
				println!("Removed item '{}' from the todo list.", item);
			} else {
				println!("Item '{}' not found in the todo list.", item);
			}
		}
	}

	pub fn remove_index(&mut self, index:usize)
	{
		if index == 0 || index > self.items.len() {
			println!("Error: Index {} is out of bounds.", index);
		} else {
			self.items.remove(index - 1);
		}
	}

	pub fn check_item(&mut self, item_value:String)
	{
		for current_item in &mut self.items {
			if current_item.has_value(&item_value) {
				current_item.check()
			}
		}
	}

	pub fn check_index(&mut self, index:usize)
	{
		if index == 0 || index > self.items.len() {
			println!("Error: Index {} is out of bounds", index)
		} else {
			self.items[index - 1].check()
		}
	}

	pub fn uncheck_item(&mut self, item_value:String)
	{
		for item in &mut self.items {
			if item.has_value(&item_value) {
				item.uncheck()
			}
		}
	}

	pub fn uncheck_index(&mut self, index:usize)
	{
		if index == 0 || index > self.items.len() {
			println!("Error: Index {} is out of bounds", index)
		} else {
			self.items[index - 1].uncheck()
		}
	}

	pub fn print(&self)
	{
		for (index, item) in self.items.iter().enumerate() {
			println!("{}. {}", index + 1, item);
		}
	}
} // End of TaskList implementation

#[cfg(test)]
mod unit_tests {
	use super::*;

	// ToDo: The default list is not known to the test function. How do we do that?
	fn setup() -> TaskList {
		let mut list = TaskList::new();

		let default_items = vec![
			String::from("Mango"),
			String::from("Banana"),
		];

		list.add(default_items);
		return list;
	}

	fn is_in_list(list: &TaskList, item_to_search: &str) -> bool
	{
		// Iterate by reference so we don’t take ownership of `list.items`.
		// This keeps the caller able to use `list` after the call.
		for current_item in &list.items {
			if current_item.has_value(item_to_search)
			{
				return true
			}
		}
		false
	}

	#[test]
	fn test_add() {

		let test_list = setup();

		assert_eq!(test_list.items.len(), 2);
		assert!(test_list.items[0].has_value("Mango"));
		assert!(test_list.items[1].has_value("Banana"));
	}

	#[test]
	fn test_remove() {
		let mut test_list = setup();

		let test_item = "Mango";
		assert!(is_in_list(&test_list, test_item));

		test_list.remove_items(vec![String::from(test_item)]);
		assert!(!is_in_list(&test_list, test_item));
	}

	#[test]
	fn test_remove_index() {
		let mut test_list = setup();

		let first_item = test_list.items[0].clone();
		test_list.remove_index(1);

		assert_ne!(test_list.items[0], first_item);
	}

	#[test]
	fn test_check_uncheck_item(){
		let mut test_list = setup();

		let test_item = "Mango";
		assert!(test_list.items[0].has_value(test_item));
		assert!(!test_list.items[0].is_checked);

		test_list.check_item(test_item.to_string());
		assert!(test_list.items[0].is_checked);

		test_list.uncheck_item(test_item.to_string());
		assert!(!test_list.items[0].is_checked);
	}

	#[test]
	fn test_check_uncheck_index(){
		let mut test_list = setup();
		let test_index = 1;

		assert!(!test_list.items[test_index - 1].is_checked);

		test_list.check_index(test_index);
		assert!(test_list.items[test_index - 1].is_checked);

		test_list.uncheck_index(test_index);
		assert!(!test_list.items[test_index - 1].is_checked);
	}

}
