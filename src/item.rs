pub struct Item{
	value: String,
	pub is_checked : bool,
}

impl Item {

	pub fn check(&mut self)
	{
		self.is_checked = true;
	}

	pub fn uncheck(&mut self)
	{
		self.is_checked = false;
	}

	pub fn has_value(&self, value_to_check:&str) -> bool
	{
		value_to_check == self.value
	}
}

use std::fmt;
impl fmt::Display for Item {
	fn fmt(&self, f:&mut fmt::Formatter<'_>)-> fmt::Result{
		if self.is_checked {
			write!(f, "~{}~", self.value)
		}else {
			write!(f, "{}", self.value)
		}
	}
}

pub fn new_item(value:String) -> Item
{
	Item {value, is_checked:false}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_new_item(){
		let value: String = "Mango".to_string();
		let uut: Item = new_item(value.clone());
		assert!(!uut.is_checked);
		assert!(uut.has_value(&value));
	}

	#[test]
	fn test_check_uncheck(){
		let value: String = "Mango".to_string();
		let mut uut: Item = new_item(value);

		assert!(!uut.is_checked);

		uut.check();
		assert!(uut.is_checked);

		uut.uncheck();
		assert!(!uut.is_checked);
	}

}
