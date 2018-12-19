#![allow(dead_code)]
extern crate sfml;
extern crate futures;

use sfml::graphics::{RenderWindow, RenderTarget, Transformable};
use resources::Resources;
use color::Color;
use recorder::{Recorder, ChessSet, Move};

use KEY;
use new_index::*;
use square::Square;
use pieces::Piece;


use utility;
struct TempMove<'a>
{
    piece: Option<Piece<'a>>,
    old_pos: Option<Square>
}

impl<'a> TempMove<'a>
{
    fn new() -> Self
    {
        TempMove {
            piece: None,
            old_pos: None
        }
    }

    fn with(mut self, piece: Option<Piece<'a>>, square: Option<Square>) -> Self
    {
        self.piece = piece;
        self.old_pos = square;
        self
    }

    fn set(&mut self, piece: Option<Piece<'a>>, square: Option<Square>)
    {
        self.piece = piece;
        self.old_pos = square;
    }

    fn is_some(&self) -> bool
    {
        self.piece.is_some()
    }

    fn as_mut(&mut self) -> Option<&mut Piece<'a>>
    {
        self.piece.as_mut()
    }
     
    fn as_ref(&self) -> Option<&Piece<'a>>
    {
        self.piece.as_ref()
    }
    fn square(&self) -> Option<&Square>
    {
        self.old_pos.as_ref()
    }

    fn take_square(&mut self) -> Option<Square>
    {
        self.old_pos.take()
    }
}

pub struct Game<'a>
{
    pub hold_mouse: bool,
    pressed_key: bool,
    temp_move: TempMove<'a>,
    recorder: Recorder<'a> 
}

impl<'a> Game<'a>
{
    pub fn new(res: &'a Resources<KEY>, window: &RenderWindow) -> Self
    {
        Game {
            hold_mouse: false,
            pressed_key: false,
            temp_move: TempMove::new(),
            recorder: Recorder::new(res, window)
        }
    }
    

    pub fn display(&self, window: &mut RenderWindow)
    {
        self.recorder.display(window);
        if self.temp_move.is_some()
        {
            window.draw( &self.temp_move.as_ref().unwrap().sprite );
        }
    }

    pub fn update(&mut self, window: &mut RenderWindow)
    {
        use sfml::window::mouse;
        if self.temp_move.is_some()
        {
            if self.hold_mouse
            {
                self.move_piece(window); 
            }
            else
            {
                let mut piece = self.temp_move.piece.take().unwrap();
                if self.evaluate_move(&mut piece, window)
                {
                    let square = utility::get_square(window);
                    let mov = self.construct_move(&piece, square.clone());
                    self.recorder.record( mov );
                    self.recorder.place( piece, square );
                }
                else
                {
                    let square = self.temp_move.take_square().unwrap();
                    self.recorder.place( piece, square );
                }
                
            }

        }
        else if mouse::Button::Left.is_pressed()
        {
            let square = utility::get_square(window);
            self.temp_move.set( self.recorder.get_board().remove(&square), Some(square) );
        }
        use sfml::window::Key;
        if Key::Left.is_pressed() && !self.pressed_key
        {
            self.recorder.undo(); 
            self.pressed_key = true;
        }
        else if !Key::Left.is_pressed() 
        {
            self.pressed_key = false;
        }
    }

    fn construct_move(&self, piece: &Piece<'a>, to: Square) -> Move
    {
        utility::construct_move(&piece, 
                                self.recorder.board(),
                                to, // to
                                self.temp_move.square().unwrap().clone() //from
                                )
    }
    fn move_piece(&mut self, window: &mut RenderWindow)
    {
        self.temp_move.as_mut().unwrap().sprite.set_position( utility::get_mousemid(window) );
    }

    fn evaluate_move(&mut self, piece: &mut Piece<'a>, window: &mut RenderWindow) -> bool
    {
        use self::futures::prelude::*;
        // mutable incase en passant
        let mut en_passant_square: Option<Square> = None;
        match piece.try_move(
                          &mut self.recorder, 
                          self.temp_move.square().unwrap(), 
                          &utility::get_square(window)).poll()
        {
            Err(_) => return false,
            Ok(Async::Ready(s)) => {en_passant_square = s;} 
            _ => {}
        };

        if let Some(s) = en_passant_square
        {
            self.recorder.get_board().remove(&s);
        }
        true
    }
    
}

pub fn init_recourse(res: &mut Resources<KEY>)
{
    res.add_from_file("src/assets/chess.png", _Index::Board); 

    res.add_from_file("src/assets/pawn_w.png", _Index::Pawn(Color::White));
    res.add_from_file("src/assets/pawn_b.png", _Index::Pawn(Color::Black));
    
    res.add_from_file("src/assets/knight_w.png", _Index::Knight(Color::White));
    res.add_from_file("src/assets/knight_b.png", _Index::Knight(Color::Black));
    
    res.add_from_file("src/assets/bishop_w.png", _Index::Bishop(Color::White));
    res.add_from_file("src/assets/bishop_b.png", _Index::Bishop(Color::Black));

    res.add_from_file("src/assets/rook_w.png", _Index::Rook(Color::White));
    res.add_from_file("src/assets/rook_b.png", _Index::Rook(Color::Black));

    res.add_from_file("src/assets/king_w.png", _Index::King(Color::White));
    res.add_from_file("src/assets/king_b.png", _Index::King(Color::Black));

    res.add_from_file("src/assets/queen_w.png", _Index::Queen(Color::White));
    res.add_from_file("src/assets/queen_b.png", _Index::Queen(Color::Black));
}   

