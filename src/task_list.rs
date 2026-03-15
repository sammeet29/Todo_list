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

	pub fn remove_index(&mut self, item_index:u32)
	{
		let index = item_index as usize;
		if index == 0 || index > self.items.len() {
			println!("Error: Index {} is out of bounds.", item_index);
		} else {
			self.items.remove(index - 1);
		}
	}

	pub fn check_item(&mut self, item_value:String)
	{
		for item in &mut self.items {
			if item.has_value(&item_value) {
				item.check()
			}
		}
	}

	// Todo: Change u32 to usize to avoid unnecessary type conversion
	pub fn check_index(&mut self, index:u32)
	{
		let index = index as usize;
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

	pub fn uncheck_index(&mut self, index:u32)
	{
		let index = index as usize;
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

