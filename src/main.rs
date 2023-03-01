use ::rand::prelude::*;
use macroquad::prelude::*;


mod board;
mod player;
mod fruit;


use crate::board::Board;
use crate::player::Player;
use crate::fruit::Fruit;

#[macroquad::main("basicshape")]
async fn main() {
	let mut color_counter: f32 = 0.0;

	let mut board: Board = Board::new();
	let mut player: Player = Player::new();
	let mut fruit: Fruit = Fruit::new();
	board.set_positions();

	loop {
		clear_background(WHITE);
		board.draw();
		for i in player.tail.iter(){
			board.paint(i.x, i.y, BLACK);
		}
		board.paint(player.head.x, player.head.y, BLACK);
		player.update();
		board.paint(player.head.x, player.head.y, PURPLE);
		color_counter = 0.9;
		for i in player.tail.iter(){

			board.paint(i.x, i.y, Color::new(color_counter, color_counter, color_counter, 1.0));
			color_counter -= 0.01;
		}
		board.paint(fruit.pos.x, fruit.pos.y, RED);
		if fruit.check_position(player.head.x, player.head.y) == true { 
			player.grow();
		}
		for i in player.tail.iter(){
			fruit.check_position(i.x, i.y);
		}

		next_frame().await
	}



}