use ::rand::prelude::*;
use macroquad::prelude::*;

use crate::board::Pos;

pub struct Fruit {
	pub pos: Pos
}

impl Fruit {
	pub fn new() -> Fruit {
		Fruit {
			pos: Pos {
				x: thread_rng().gen_range(0..20),
				y: thread_rng().gen_range(0..20)
			}
		}
	}

	pub fn check_position(&mut self, x: isize, y: isize) -> bool{
		if x == self.pos.x && y == self.pos.y {
			self.pos.x = thread_rng().gen_range(0..20);
			self.pos.y = thread_rng().gen_range(0..20);
			return true
		}
		return false
	}
}