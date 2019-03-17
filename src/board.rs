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

    //pub king_pos: HashMap<Color, Box<*const Piece<'a>>>
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
            //king_pos: HashMap::new(),
        }
    }


    #[inline]
    pub fn display_board(&self, window: &mut RenderWindow)
    {
        window.draw(&self.board);
    }
    #[inline]
    pub fn display_pieces(&self, window: &mut RenderWindow)
    {
        self.squares.values().for_each(|p| window.draw(&p.sprite))
    }

    #[inline]
    pub fn place(&mut self, p: Piece<'a>, s: Square)
    {
        self.squares.insert(s, p);
    }

    #[inline]
    pub fn scale(&self) -> f32
    {
        self.scale
    }
    #[inline]
    pub fn board_size(&self) -> Vector2u
    {
        self.size
    }
    #[inline]
    pub fn board_mut(&mut self) -> &mut HashMap<Square, Piece<'a>>
    {
        &mut self.squares
    }
    
    #[inline]
    pub fn board(&self) -> &HashMap<Square, Piece<'a>>
    {
        &self.squares
    }

    #[inline]
    pub fn set_kings(&mut self)
    {
        /*let pos = (Square::new(4, 7), Square::new(4, 0));

        let wking = self.squares.remove(&pos.0).unwrap();
        let king_pointer: *const Piece= &wking;
        self.king_pos.insert(Color::White, Box::new(king_pointer));

        let bking = self.squares.remove(&pos.1).unwrap();
        let king_pointer: *const Piece= &wking;
        self.king_pos.insert(Color::Black, king_pointer);

        self.squares.insert(pos.0, wking);
        self.squares.insert(pos.1, bking);*/
    }
    #[inline]
    pub fn update_king(&mut self, c: &Color, new_pos: &Square)
    {
        self.kings.get_mut(c).unwrap().set(new_pos.col, new_pos.row);
    }

    #[inline]
    pub fn get_king(&self, color: &Color) -> &Square
    {
        self.kings.get(color).unwrap()
    }


}


