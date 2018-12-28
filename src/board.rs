extern crate sfml;
use sfml::graphics::{Sprite, RenderWindow, RenderTarget};
use sfml::system::{Vector2u};
use std::collections::HashMap;
use new_piece_creator;
use pieces::Piece;
use square::Square;
use resources::Resources;   
use color::Color;

use KEY;
#[allow(dead_code)]
pub struct Board<'a>
{
    squares: HashMap<Square, Piece<'a>>,
    board  : Sprite<'a>,
    size: Vector2u,
    scale: f32,

    kings: HashMap<Color, Square>,
}

#[allow(dead_code)]
impl<'a> Board<'a>
{
    pub fn new(res: &'a Resources<KEY>, window: &RenderWindow) -> Self
    {
        let (board, scale) = new_piece_creator::create_board(res, window);
                // White King Pos   -   Black King Pos
        let pos = (Square::new(4, 7), Square::new(4, 0));
        let mut set = HashMap::new();
        set.insert(Color::White, pos.0);
        set.insert(Color::Black, pos.1);
        Board {
            squares: HashMap::new(),
            board: board,
            size: window.size(),
            scale: scale,

            kings: set,
        }
    }

    pub fn display(&self, window: &mut RenderWindow)
    {
        window.draw(&self.board);
        self.squares.values().for_each(|p| window.draw(&p.sprite))
    }

    pub fn place(&mut self, p: Piece<'a>, s: Square)
    {
        self.squares.insert(s, p);
    }

    pub fn scale(&self) -> f32
    {
        self.scale
    }
    pub fn board_size(&self) -> Vector2u
    {
        self.size
    }
    pub fn get_board(&mut self) -> &mut HashMap<Square, Piece<'a>>
    {
        &mut self.squares
    }
    
    pub fn board(&self) -> &HashMap<Square, Piece<'a>>
    {
        &self.squares
    }

    pub fn set_king(&mut self, color: Color, square: Square)
    {
        self.kings.insert(color, square);
    }
    pub fn update_king(&mut self, c: &Color, new_pos: &Square)
    {
        self.kings.get_mut(c).unwrap().set(new_pos.col, new_pos.row);
    }

    pub fn get_king(&self, color: &Color) -> &Square
    {
        self.kings.get(color).unwrap()
    }


}


