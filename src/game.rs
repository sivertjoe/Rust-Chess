#![allow(dead_code)]
extern crate sfml;
extern crate futures;


use sfml::graphics::{RenderWindow, RenderTarget,  Transformable};
use sfml::system::{ Vector2u};
use resources::Resources;
use color::Color;
use recorder::{Recorder, ChessSet, Move};

use KEY;
use new_index::*;
use square::Square;
use pieces::Piece;

use highlight::Highlight;
use arrow::Arrow;

use utility;
use temp_move::TempMove;


pub struct Game<'a>
{
    pub hold_mouse: bool,
    pressed_left: bool,
    pressed_right: bool,
    temp_move: TempMove<'a>,
    pub recorder: Recorder<'a>,
    turn: Color,
    input_square: Vec<Square>,


    m1: bool,
    m2: bool,


    highlighed_squares: Vec<Highlight<'a>>,
    arrows: Vec<Arrow<'a>>,
}

impl<'a> Game<'a>
{
    pub fn new(res: &'a Resources<KEY>, window: &RenderWindow) -> Self
    {
        Game {
            hold_mouse: false,
            pressed_left: false,
            pressed_right: false,
            temp_move: TempMove::new(),
            recorder: Recorder::new(res, window),
            turn: Color::White,

            m1: false,
            m2: false,

            input_square: Vec::new(),
            highlighed_squares: Vec::new(),
            arrows: Vec::new(),
        }
    }

    #[inline]
    pub fn push_square(&mut self, square: Square)
    {
        self.input_square.push(square);
    }

    pub fn eval_squares(&mut self)
    {
        let s2 = self.input_square.pop().unwrap(); 
        let s1 = self.input_square.pop().unwrap(); 


        if s1 == s2
        {
            // change color

            let highlight = Highlight::new(self.board_size(), &s1);

            if self.highlighed_squares.remove_item(&highlight).is_none()
            {
                self.highlighed_squares.push(highlight);
            }
        }
        else
        {
            // draw arrow
            if let Some(arrow) = Arrow::new(self.board_size(), &s1, &s2)
            {
                if self.arrows.remove_item(&arrow).is_none()
                {
                    self.arrows.push(arrow);
                }
            }
        }
    }

    #[inline]
    fn board_size(&self) -> Vector2u
    {
        self.recorder.ref_board().board_size()
    }
   

    #[inline]
    pub fn clear_squares(&mut self)
    {
        self.highlighed_squares.clear();
    }

    pub fn clear_arrows(&mut self)
    {
        self.arrows.clear();
    }

    pub fn display(&self, window: &mut RenderWindow)
    {

        self.recorder.ref_board().display_board(window);

        self.highlighed_squares.iter().for_each(|s| // Sprite<'a>, red square
        {
            window.draw(s);
        });

        self.recorder.ref_board().display_pieces(window);


        self.arrows.iter().for_each(|a| // Arrow
        {
            window.draw(a);
        });
        
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
            if mouse::Button::Left.is_pressed()
            {
                self.move_piece(window); 
            }
            else
            {
                self.evaluate_move(window);
            }

        }
        else if mouse::Button::Left.is_pressed()
        {
            let square = utility::get_square(window);
            self.temp_move.set( self.recorder.board_mut().remove(&square), Some(square) );
        }

        self.handle_input(window);
    }

    fn evaluate_move(&mut self, window: &mut RenderWindow)
    {
        let mut piece = self.temp_move.piece.take().unwrap();
        
        if !self.legal_move(&mut piece, window)
        {
            // place piece back on board
            self.place_back(piece);
            return;
        }

            let square = utility::get_square(window);
            let _type = piece.get_type();
            self.handle_king_moves(&square, &_type);

            self.recorder.record( self.construct_move(&piece, square.clone()));
            self.recorder.place( piece, square.clone());

            if !self.check(&self.turn)
            {
                self.turn = !self.turn.clone()
            }
            else
            {
                self.recorder._undo();
                self.handle_king_moves(&square, &_type);
            }
    }

    #[inline]
    fn handle_king_moves(&mut self, square: &Square, _type: &_Index<Color>)
    {
        match _type
        {
            &_Index::King(_) =>
            {
                self.update_kingpos(&square);
            }
            _ => {},
        };
    }
    #[inline]
    fn update_kingpos(&mut self, square: &Square)
    {
        self.recorder._board().update_king(
            &self.turn, 
            &square)
    }

    #[inline]
    fn place_back(&mut self, piece: Piece<'a>)
    {
        let square = self.temp_move.take_square().unwrap();
        self.recorder.place( piece, square );
    }
    fn handle_input(&mut self, window: &mut RenderWindow)
    {
        use sfml::window::Key;
        if Key::Left.is_pressed() && !self.pressed_left
        {
            self.recorder.undo(); 
            self.turn = !self.turn.clone();
            self.pressed_left = true;
        }
        else if !Key::Left.is_pressed() 
        {
            self.pressed_left = false;
        }

        if Key::Right.is_pressed() && !self.pressed_right
        {
            self.recorder.redo(); 
            self.turn = !self.turn.clone();
            self.pressed_right = true;
        }
        else if !Key::Right.is_pressed() 
        {
            self.pressed_right = false;
        }

        self.handle_mouse_input(window); 
    }

    fn handle_mouse_input(&mut self, window: &mut RenderWindow)
    {
        use sfml::window::mouse;
        if mouse::Button::Right.is_pressed() && !self.m1
        {
            self.push_square( utility::get_square(window) );
            self.m1 = true;
        }
        if !mouse::Button::Right.is_pressed() && self.m1
        {
            self.push_square( utility::get_square(window) );
            self.eval_squares();
            self.m1 = false;
        }

        if mouse::Button::Left.is_pressed() && !self.m2
        {
            self.clear_arrows();
            self.clear_squares();
            self.m2 = true;
        }
        if !mouse::Button::Left.is_pressed() && self.m2
        {
            self.m2 = false;
        }

    }

    #[inline]
    fn construct_move(&self, piece: &Piece<'a>, to: Square) -> Move
    {
        utility::construct_move(
            &piece, 
            self.recorder.board(),
            to,
            self.temp_move.square().unwrap().clone() //from
            )
    }

    #[inline]
    fn move_piece(&mut self, window: &mut RenderWindow)
    {
        self.temp_move.as_mut().unwrap().sprite.set_position( utility::get_mousemid(window) );
    }

    fn legal_move(&mut self, piece: &mut Piece<'a>, window: &mut RenderWindow) -> bool
    {
        use self::futures::prelude::*;
        if piece.color() != &self.turn
        {
            return false;
        }
        let mut special_square: Option<Square> = None;
        
        let res = piece.try_move(
                            &self.recorder,
                            self.temp_move.square().unwrap(),
                            &utility::get_square(window)).poll();

        match res 
        {
            Err(_) => return false,
            Ok(Async::Ready(s)) => {special_square = s;} 
            _ => {}
        };

        if let Some(s) = special_square
        {
            self.handle_en_passant_castle(s)
        }

        true
    }

    fn handle_en_passant_castle(&mut self, s: Square)
    {
        let p = self.recorder.board_mut().remove(&s).unwrap();
        match p.get_type()
        {
            _Index::Pawn(_) => {}, // Remove it
            _Index::Rook(_) => // Castle
            {
                let square_col = match s.col
                {
                    7 => 5,
                    0 => 3,
                    _ => unreachable!()
                };
                let square = Square::new(square_col, s.row);
                self.recorder.place(p, square);
            }, 
            _ => unreachable!()
        }
    }

    fn check(&self, color: &Color) -> bool
    {
        let ns = self.recorder.ref_board().get_king(color);
        self.recorder.board().iter().find(|(s, p)|
        {
            use self::futures::prelude::*;
            if &p.color == color { return false; } 
            match p.try_move(&self.recorder, s, ns).poll()
            {
                Ok(Async::Ready(_)) => true, 
                _ => false,
            }

        }).is_some()
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

