pub struct Item{
	value: String,
	is_checked : bool,
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
