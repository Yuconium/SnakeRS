use ::rand::prelude::*;
use macroquad::prelude::*;

use crate::board::Pos;


pub struct Player {
	pub vel: Pos,
	pub head: Pos,
	pub tail: Vec<Pos>,
	tail_length: usize,
	move_timer: f64

}

impl Player {
	pub fn new() -> Player {
		Player {
			vel: Pos {
				x: 1,
				y: 0,
			},
			head: Pos {
				x: thread_rng().gen_range(0..20),
				y: thread_rng().gen_range(0..20)
			},
			tail: Vec::new(),
			tail_length: 2,
			move_timer: get_time()
		}
	}

	pub fn update(&mut self) {
		
		if self.move_timer + 0.2 <= get_time() {
			self.tail.push(Pos{
				x: self.head.x,
				y: self.head.y
			});
			self.head.x += self.vel.x;
			self.head.y += self.vel.y;

			self.move_timer = get_time();
		}


		self.check_input();

		if self.tail.len() >= self.tail_length {
			self.tail = self.tail[1..self.tail.len()].to_vec();
		}

		self.check_tail_collision();




		if self.head.x < 0 {	self.head.x = 19	}	
		else if self.head.x > 19 {	self.head.x = 0 }

		if self.head.y < 0 {	self.head.y = 19	}	
		else if self.head.y > 19 {	self.head.y = 0 }
	}

	pub fn grow(&mut self) {
		self.tail_length += 1;
	}




	pub fn check_input(&mut self) {
		if is_key_pressed(KeyCode::W) {
			if self.vel.y != 1 {
				self.vel.x = 0;
				self.vel.y = -1;
				println!("VELX: {} VELY: {}", self.vel.x, self.vel.y);
			}
		}	else if is_key_pressed(KeyCode::D) {
				if self.vel.x != -1 {
					self.vel.x = 1;
					self.vel.y = 0;
					println!("VELX: {} VELY: {}", self.vel.x, self.vel.y);
				}
		}	else if is_key_pressed(KeyCode::A) {
				if self.vel.x != 1 {
					self.vel.x = -1;
					self.vel.y = 0;
					println!("VELX: {} VELY: {}", self.vel.x, self.vel.y);
				}
		}	else if is_key_pressed(KeyCode::S) {
				if self.vel.y != -1 {
					self.vel.x = 0;
					self.vel.y = 1;
					println!("VELX: {} VELY: {}", self.vel.x, self.vel.y);
				}		
			}
	}

	fn check_tail_collision(&mut self){
		for i in self.tail.iter() {
			if self.head.x == i.x && self.head.y == i.y {
				self.tail_length = 2;
			}
		}
	}

}
