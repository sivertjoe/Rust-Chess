extern crate sfml;
use sfml::graphics::{Sprite, RenderWindow, RenderTarget, Transformable};
use sfml::system::{Vector2u, Vector2f};
use std::collections::HashMap;
use new_piece_creator;
use pieces::Piece;
use square::Square;
use resources::Resources;   
use color::Color;
use std::rc::Rc;
use std::cell::RefCell;

use KEY;
pub struct Board<'a>
{
    squares: HashMap<Square, Rc<RefCell<Piece<'a>>>>,
    board  : Sprite<'a>,
    size: Vector2u,
    scale: f32,



    king_pos: HashMap<Color, Rc<RefCell<Piece<'a>>>>,
}

impl<'a> Board<'a>
{
    pub fn new(res: &'a Resources<KEY>, window: &RenderWindow) -> Self
    {
        let (board, scale) = new_piece_creator::create_board(res, window);
        
        Board {
            squares: HashMap::new(),
            board: board,
            size: window.size(),
            scale: scale,

            king_pos: HashMap::new(),
        }
    }
    #[inline]
    pub fn get_piece(&mut self, square: &Square) -> Option<Rc<RefCell<Piece<'a>>>>
    {
        self.squares.remove(square)
    }

    #[inline]
    pub fn display_board(&self, window: &mut RenderWindow)
    {
        window.draw(&self.board);
    }
    #[inline]
    pub fn display_pieces(&self, window: &mut RenderWindow)
    {
        self.squares.values().for_each(|p|
        {
            window.draw(&p.borrow().sprite);
        });
    }

    #[inline]
    pub fn place(&mut self, p: Piece<'a>, s: Square)
    {
        self.squares.insert(s, Rc::new(RefCell::new(p)));
    }

    #[inline]
    pub fn rc_place(&mut self, p: Rc<RefCell<Piece<'a>>>, s: Square)
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
    pub fn board_mut(&mut self) -> &mut HashMap<Square, Rc<RefCell<Piece<'a>>>>
    {
        &mut self.squares
    }
    
    #[inline]
    pub fn board(&self) -> &HashMap<Square, Rc<RefCell<Piece<'a>>>>
    {
        &self.squares
    }

    #[inline]
    pub fn set_kings(&mut self)
    {
        let pos = (Square::new(4, 7), Square::new(4, 0));

        let bk = Rc::clone( self.squares.get(&pos.1).unwrap() );
        self.king_pos.insert(Color::Black, bk);

        let wk = Rc::clone( self.squares.get(&pos.0).unwrap() );
        self.king_pos.insert(Color::White, wk);
    }

    #[inline]
    pub fn get_kingpos(&self, color: &Color) -> Vector2f
    {
        self.king_pos.get(color).unwrap().borrow().sprite.position()
    }


}


