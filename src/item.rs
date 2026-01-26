enum Status{
	UNCHECKED,
	CHECKED,
}

pub struct Item{
	value: String,
	status : Status,
}

impl Item {

	pub fn check(&mut self)
	{
		self.status = Status::CHECKED;
	}

	pub fn uncheck(&mut self)
	{
		self.status = Status::UNCHECKED;
	}

	pub fn has_value(&self, value_to_check:&str) -> bool
	{
		value_to_check == self.value
	}

}

use std::fmt;
impl fmt::Display for Item {
	fn fmt(&self, f:&mut fmt::Formatter<'_>)-> fmt::Result{
		write!(f, "{}", self.value)
	}
}

pub fn new_item(value:String) -> Item
{
	Item {value, status:Status::UNCHECKED}
}