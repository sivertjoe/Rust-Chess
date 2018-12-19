extern crate sfml;
use sfml::graphics::{Sprite, RenderWindow, RenderTarget};
use sfml::system::Vector2u;
use std::collections::HashMap;
use new_piece_creator;
use pieces::Piece;
use square::Square;
use resources::Resources;   

use KEY;
#[allow(dead_code)]
pub struct Board<'a>
{
    squares: HashMap<Square, Piece<'a>>,
    board  : Sprite<'a>,
    size: Vector2u,
    scale: f32,
    
}

#[allow(dead_code)]
impl<'a> Board<'a>
{
    pub fn new(res: &'a Resources<KEY>, window: &RenderWindow) -> Self
    {
        let (board, scale) = new_piece_creator::create_board(res, window);
        Board {
            squares: HashMap::new(),
            board: board,
            size: window.size(),
            scale: scale
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
}


