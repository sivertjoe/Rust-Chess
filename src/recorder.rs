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
        self.moves.push(m);
    }

    fn undo(&mut self)
    {
        if self.moves.last().is_none()
        {
            return;
        }
        let last_move = self.moves.pop().unwrap();

        
        match &last_move.piece
        {
            &_Index::Pawn(_) => 
            {                                                          // col
                if utility::square_diff(last_move.to(), last_move.from()).0  != 0
                {
                    let prev_move = self.moves.last().unwrap();
                    let diff_row = utility::square_diff(prev_move.to(), prev_move.from()).1;
                    if diff_row.abs() == 2
                    {
                        use new_piece_creator::*;
                        let (index, square) = calculate_en_passant(&last_move);
                        let piece = m_create_piece(self.resorces, self.board.scale(), &index);  
                        self._place(piece, square);
                    }
                }
            }
            _ => {},
        };

        let board = self.board.get_board(); 
        let piece = board.remove(last_move.to()).unwrap();

        self._place(piece, last_move.from().clone());
        
        if last_move.capture.is_some()
        {
            use new_piece_creator::*;
            let captured_piece = m_create_piece(self.resorces,
                                                 self.board.scale(), 
                                                 last_move.capture.as_ref().unwrap());  
            self._place(captured_piece, last_move.to().clone());
        }
    }


    fn init(&mut self)
    {
        use new_piece_creator::*;
        m_create_kings(self);
        m_create_queens(self);
        m_create_rooks(self);
        m_create_knights(self);
        m_create_bishop(self);
        m_create_pawns(self); 
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


fn calculate_en_passant(mov: &Move) -> (_Index<Color>, Square)
{
    let color = mov.piece.get();
    let mask_color = match &color
    {
        &Color::White => 1,
        _ => -1,
    };
    let mut square = mov.to().clone();
    square.inc(0, mask_color);
    (_Index::Pawn(!color), square)
}

pub struct Recorder<'a>
{
    moves: Vec<Move>,
    resorces: &'a Resources<KEY>,
    board: Board<'a>,
}

impl<'a> Recorder<'a>
{
    pub fn new(res: &'a Resources<KEY>, window: &RenderWindow) -> Self
    {
        let mut recorder = Recorder {
            moves: Vec::new(),
            resorces: res,
            board: Board::new(res, window),
        };

        recorder.init();
        recorder

    }

    pub fn display(&self, window: &mut RenderWindow)
    {
        self.board.display(window);
    }

    pub fn get_board(&mut self) -> &mut HashMap<Square, Piece<'a>>
    {
        self.board.get_board()
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

}

#[allow(dead_code)]
pub struct Move
{
    piece: _Index<Color>,
    
    to: Square,
    from: Square,

    capture: Option<_Index<Color>>,
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


impl std::fmt::Display for Move
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        let capture = match &self.capture { Some(_) => "x", _ => "" };
        let _type = 
        if capture == "x" 
        {
            self.get_capture()         
        }
        else
        {
            self.get_move()
        };

        let final_square = format!("{}", &self.to);
        write!(fmt, "{}{}{}", _type, capture, final_square)
    }
}
