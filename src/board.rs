use macroquad::prelude::*;
use ::rand::prelude::*;

#[derive(Copy, Clone)]
pub struct Tile {
    pub width: f32,
    pub height: f32,
    pub xpos: f32,
    pub ypos: f32,
    pub color: Color,
}

impl Tile {
    pub fn set_position(&mut self, xpos: f32, ypos: f32) {
        self.xpos = xpos;
        self.ypos = ypos;
    }
}

pub struct Board {
    pub field: [[Tile; 20]; 20],
}

impl Board {
    pub fn new() -> Board {
        Board {
            field: [[Tile {
                width: screen_width() / 20.0,
                height: screen_height() / 20.0,
                xpos: 0.0,
                ypos: 0.0,
                color: BLACK,
            }; 20]; 20],
        }
    }

    pub fn paint(&mut self, x: isize, y: isize, color: Color) {
        self.field[y as usize][x as usize].color = color;
    }

    pub fn set_positions(&mut self) {
        let mut xpos: f32 = 0.0;
        let mut ypos: f32 = 0.0;
        println!("{} {}", xpos, ypos);
        for y in self.field.iter_mut() {
            for x in y {
                println!("{} {} {} {}", x.xpos, x.ypos, xpos, ypos);
                x.set_position(xpos, ypos);
                println!("{} {} {} {}", x.xpos, x.ypos, xpos, ypos);
                xpos += screen_width() / 20.0;
            }
            xpos = 0.0;
            ypos += screen_height() / 20.0;
        }
    }

    pub fn draw(&mut self) {
        for y in self.field.iter() {
            for x in y {
                draw_rectangle(x.xpos, x.ypos, x.width, x.height, x.color);
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct Pos {
    pub x: isize,
    pub y: isize,
}
