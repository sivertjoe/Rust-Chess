#![allow(dead_code)]
#![allow(unused_variables)]
use color::Color;
use square::Square;
use pieces::{Piece};
use board::Board;
use resources::Resources;
extern crate sfml;
use sfml::graphics::{RenderWindow, Transformable};
use sfml::system::Vector2u;
use std::collections::HashMap;


use utility;

use new_index::*;

use KEY; // Defines the key use in resource throughout the program, defined in main.rs

pub trait ChessSet<'a>
{
    fn place(&mut self, Piece<'a>, Square);
    fn place_multiple(&mut self, vec: Vec<Piece<'a>>, s: Vec<Square>);
    
    fn record(&mut self, Move);
    fn get(&self, usize) -> Option<&Move>;
    fn undo(&mut self);
    fn redo(&mut self);

    // Utility
    fn init(&mut self);
    fn resource(&self) -> &'a Resources<KEY>;
    fn scale(&self) -> f32;
    fn board_size(&self) -> Vector2u;
}

impl<'a> ChessSet<'a> for Recorder<'a>
{
    fn place(&mut self, p: Piece<'a>, s: Square)
    {
        self._place(p, s);
    }

    fn place_multiple(&mut self, pieces: Vec<Piece<'a>>, squares: Vec<Square>)
    {
        pieces.into_iter().zip(squares.into_iter()).for_each(|(p, s)| self.place(p, s));
    }

    fn record(&mut self, m: Move)
    {
        if !self.move_buffer.contains(&m)
        {
            self.move_buffer.clear();
        }
        else if self.move_buffer.len() != 0
        {
            self.move_buffer.pop();
        }
        self.moves.push(m);
    }

    fn redo(&mut self)
    {
        if let Some(saved_move) = self.move_buffer.pop()
        {
            let board = self.board_mut();
            let piece = board.remove(saved_move.from()).unwrap();

            self._place(piece, saved_move.to().clone());
        
            match saved_move.piece
            {
                _Index::Pawn(_) => 
                {
                    let diff = utility::square_diff(saved_move.to(), saved_move.from());
                    if diff.0 != 0 && saved_move.capture == None
                    {
                        // En passant
                        let color = saved_move.piece.get();
                        let n = match &color { &Color::White => 1, _ => -1 };
                        let mut en_passant_square = saved_move.to().clone();
                        en_passant_square.set( en_passant_square.col, en_passant_square.row as isize + n);

                        self.board_mut().remove(&en_passant_square);
                    }
                }

                _Index::King(_) =>
                {
                    let diff = utility::square_diff( &saved_move.from, &saved_move.to );
                    if diff.0 == -2 // Short castle
                    {
                        let mut rook_square = Square::new( 7, saved_move.to.row );
                        let rook = self.board_mut().remove(&rook_square).unwrap();
                        rook_square.col = 5;
                        self._place(rook, rook_square);
                    }
                    if diff.0 == 2 // Long
                    {
                        let mut rook_square = Square::new( 0, saved_move.to.row );
                        let rook = self.board_mut().remove(&rook_square).unwrap();
                        rook_square.col = 3;
                        self._place(rook, rook_square);
                    }
                }
                _ => {},
            };
            self.moves.push(saved_move);
        }
    }

    fn undo(&mut self)
    {
        if let Some(last_move) = self._undo()
        {
            self.move_buffer.push(last_move);
        }
    }


    fn init(&mut self)
    {
        use new_piece_creator::*;
        m_create_queens(self);
        m_create_rooks(self);
        m_create_knights(self);
        m_create_bishop(self);
        m_create_pawns(self); 
        m_create_kings(self);

        self.board.set_kings();

    }
    fn resource(&self) -> &'a Resources<KEY>
    {
        &self.resorces
    }
    fn scale(&self) -> f32
    {
        self.board.scale()
    }

    fn board_size(&self) -> Vector2u
    {
        self.board.board_size()
    }
    fn get(&self, offset: usize) -> Option<&Move>
    {
        if self.moves.len() == 0 { return None; }
        self.moves.get(self.moves.len() - 1 -offset)
    }

}

fn handle_en_passant(rec: &mut Recorder, last_move: &Move)
{
    match last_move.piece()
    {
        &_Index::Pawn(_) => 
        {                                                          // col
            if utility::square_diff(last_move.to(), last_move.from()).0  != 0
            {
                let prev_move = rec.moves.last().unwrap();
                let diff_row = utility::square_diff(prev_move.to(), prev_move.from()).1;
                let en_passant_square = match last_move.piece.get()
                {
                    Color::White => 3,
                    _ => 4,
                };
                if diff_row.abs() == 2 && last_move.from().row == en_passant_square
                {
                    use new_piece_creator::*;
                    let (index, square) = utility::calculate_en_passant(&last_move);
                    let piece = m_create_piece(rec.resorces, rec.board.scale(), &index);  
                    rec._place(piece, square);
                }
            }
        }
        _ => {},
    };
}


pub struct Recorder<'a>
{
    moves: Vec<Move>,
    move_buffer: Vec<Move>,

    resorces: &'a Resources<KEY>,
    board: Board<'a>,
}

impl<'a> Recorder<'a>
{
    pub fn new(res: &'a Resources<KEY>, window: &RenderWindow) -> Self
    {
        let mut recorder = Recorder {
            moves: Vec::new(),
            move_buffer: Vec::new(),
            resorces: res,
            board: Board::new(res, window),
        };

        recorder.init();
        recorder

    }

    pub fn n_moves(&self) -> usize
    {
        self.moves.len()
    }

    pub fn move_buffer(&self) -> &Vec<Move>
    {
        &self.move_buffer
    }
    pub fn set_moves(&mut self, vec: Vec<Move>)
    {
        self.moves.clear();
        self.move_buffer.clear(); 
        
        self.move_buffer = vec.into_iter().rev().collect();
    }


    pub fn _board(&mut self) -> &mut Board<'a>
    {
        &mut self.board
    }

    pub fn ref_board(&self) -> &Board<'a>
    {
        &self.board
    }


    pub fn board_mut(&mut self) -> &mut HashMap<Square, Piece<'a>>
    {
        self.board.board_mut()
    }

    #[inline]
    pub fn get_piece(&mut self, square: &Square) -> Option<Piece<'a>>
    {
        self.board.get_piece(square)
    }

    pub fn board(&self) -> &HashMap<Square, Piece<'a>>
    {
        self.board.board()
    }

    fn _place(&mut self, mut p: Piece<'a>, s: Square)
    {
        let pos = utility::get_boardpos(&self.board_size(), &s);
        p.sprite.set_position(pos);
        self.board.place(p, s);
    }

    pub fn _undo(&mut self) -> Option<Move>
    {
        if self.moves.last().is_none()
        {
            return None;
        }
        let last_move = self.moves.pop().unwrap();

        let board = self.board.board_mut(); 
        let piece = board.remove(last_move.to()).unwrap();

        self._place(piece, last_move.from().clone());
        
        if last_move.capture.is_some()
        {
            use new_piece_creator::*;
            let cap = last_move.capture.as_ref().unwrap();
            let captured_piece = m_create_piece(self.resorces, self.board.scale(), cap);

            self._place(captured_piece, last_move.to().clone());
        }
        handle_en_passant(self, &last_move);
        handle_castle(self, &last_move);
        Some(last_move)
    }

    pub fn moves(&self) -> std::slice::Iter<Move>
    {
        self.moves.iter()
    }
    

}
fn handle_castle(rec: &mut Recorder, last_move: &Move)
{
    match last_move.piece
    {
        _Index::King(_) =>
        {
            let diff = utility::square_diff( &last_move.from, &last_move.to );
            if diff.0 == -2 // From short castle
            {
                let mut rook_square = Square::new( 5, last_move.to.row );
                let rook = rec.board_mut().remove(&rook_square).unwrap();
                rook_square.col = 7;
                rec._place(rook, rook_square);
            }
            if diff.0 == 2
            {
                let mut rook_square = Square::new( 3 , last_move.to.row );
                let rook = rec.board_mut().remove(&rook_square).unwrap();
                rook_square.col = 0;
                rec._place(rook, rook_square);
            }
        }
        _ => {}
    }
}
#[allow(dead_code)]
#[derive(PartialEq, Eq, Debug)]
pub struct Move
{
    pub piece: _Index<Color>,
    
    pub to: Square,
    pub from: Square,

    pub capture: Option<_Index<Color>>,
} 


impl Move
{
    pub fn new(
        i: _Index<Color>, 
        to: Square,
        from: Square, 
        capture: Option<_Index<Color>>
        ) -> Self
    {
        Move {
            piece: i,
            to: to,
            from: from,
            capture: capture
        }
    }
    pub fn piece(&self) -> &_Index<Color>
    {
        &self.piece
    }

    pub fn get_type(&self) -> _Index<Color>
    {
        self.piece.clone()
    }

    pub fn to(&self) -> &Square
    {
        &self.to
    }

    pub fn from(&self) -> &Square
    {
        &self.from
    }
    fn get_capture(&self) -> String
    {
        match &self.piece
        {
            &_Index::Queen(_) => "Q".to_string(),
            &_Index::King(_) => "K".to_string(),
            &_Index::Bishop(_) => "B".to_string(),
            &_Index::Knight(_) => "N".to_string(),
            &_Index::Rook(_) => "R".to_string(),
            _ => format!("{}", self.from).remove(0).to_string(), 
        }
    }

    fn get_move(&self) -> String
    {
        match &self.piece
        {
            &_Index::Queen(_) => "Q".to_string(),
            &_Index::King(_) => "K".to_string(),
            &_Index::Bishop(_) => "B".to_string(),
            &_Index::Knight(_) => "N".to_string(),
            &_Index::Rook(_) => "R".to_string(),
            _ => "".to_string(),
        }
    }
}

use std::fmt;
impl fmt::Display for Move
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result
    {
        let text: String = match &self.piece
        {
            &_Index::Pawn(ref v) => 
            {
                let diff = utility::square_diff(&self.to, &self.from);
                if diff.0 != 0
                {
                    // Implying capture
                    let mut letter = format!("{}", self.from);
                    letter.pop();
                    format!("{}x{}", letter, self.to)
                }
                else
                {
                    format!("{}", self.to) 
                }
            }
            
            &_Index::Rook(_) | &_Index::Knight(_) =>
            {
                let takes = match self.capture
                {
                    Some(_) => "x",
                    _ => ""
                };

                let letter = self.get_move();
                format!("{}{}{}{}", letter, self.from, takes, self.to)
            }


            _ => 
            {
                let letter = self.get_move();
                let takes = match self.capture
                {
                    Some(_) => "x",
                    _ => "",
                };
                format!("{}{}{}", letter, takes, self.to)
            }
        };
        write!(fmt, "{}", text)
    }
}
