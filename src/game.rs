extern crate sfml;
use sfml::graphics::{RenderWindow, RenderTarget, Transformable};
use sfml::window::mouse;
use piece_creator::*;
use board::Board;
use square::Square;
use pieces::{Piece};
use color::Color;
use recourses::Recourses;

use std::collections::HashMap;
use index::Index;

use utility;
pub struct Game<'a>
{
    pub hold_mouse: bool,
    temp_piece: Option<Piece<'a>>,
    board: Board<'a>,
    scale: f32, 
}

impl<'a> Game<'a>
{
    pub fn new(res: &'a Recourses<Index>, window: &RenderWindow) -> Game<'a>
    {
        let (board, scale) = create_board(res, window);

        Game {
            hold_mouse: false,
            temp_piece: None,
            scale: scale,
            board: Board::new(create_piece_set(res, window, scale), board) }
    }
    

    pub fn display(&self, window: &mut RenderWindow)
    {
        window.draw(&self.board.board);
        self.board.squares
            .values()
            .for_each(|piece| window.draw(&piece.sprite ));
        if let Some(ref piece) = self.temp_piece
        {
            window.draw(&piece.sprite);
        }
    }

    pub fn update(&mut self, window: &mut RenderWindow)
    {
        if self.temp_piece.is_some()
        {
            if !self.hold_mouse
            {
                if self.is_legal_move()
                {
                    let square = utility::get_square(window);
                    let mut piece = self.temp_piece.take().unwrap();
                    piece.sprite.set_position(utility::get_boardpos(window, &square));
                    self.board.squares.insert(square, piece);
                }
            }
            else
            {
              let piece = self.temp_piece.as_mut().unwrap(); 
              piece.sprite.set_position( utility::get_mousemid(window) );
            }
        }
        else if mouse::Button::Left.is_pressed()
        {
            let square = utility::get_square(window);
            self.temp_piece = self.board.squares.remove(&square);
        }
        self.display(window);
    }

    fn is_legal_move(&mut self) -> bool
    {
        true
    }


}
extern crate futures;
use self::futures::future::*;

fn create_piece_set<'a>(res: &'a Recourses<Index>, window: &RenderWindow, scale: f32) -> HashMap<Square, Piece<'a>>
{
        create_pawns(HashMap::new(), res, Color::White, scale)
        .and_then(|map| create_pawns(map, res, Color::Black, scale))
        
        .and_then(|map| create_knights(map, res, Color::White, scale))
        .and_then(|map| create_knights(map, res, Color::Black, scale))

        .and_then(|map| create_bishops(map, res, Color::White, scale))
        .and_then(|map| create_bishops(map, res, Color::Black, scale))

        .and_then(|map| create_rooks(map, res, Color::White, scale))
        .and_then(|map| create_rooks(map, res, Color::Black, scale))
        .and_then(|map| create_king(map, res, Color::White, scale))
        .and_then(|map| create_king(map, res, Color::Black, scale))
        .and_then(|map| create_queen(map, res, Color::White, scale))
        .and_then(|map| create_queen(map, res, Color::Black, scale))
        .and_then(|map| set_position(map, window)) 
        .wait().unwrap() 
}

pub fn init_recourse(res: &mut Recourses<Index>)
{
    res.add_from_file("src/assets/chess.png", Index::Board); 

    res.add_from_file("src/assets/pawn_w.png", Index::WhitePawn);
    res.add_from_file("src/assets/pawn_b.png", Index::BlackPawn);
    
    res.add_from_file("src/assets/knight_w.png", Index::WhiteKnight);
    res.add_from_file("src/assets/knight_b.png", Index::BlackKnight);
    
    res.add_from_file("src/assets/bishop_w.png", Index::WhiteBishop);
    res.add_from_file("src/assets/bishop_b.png", Index::BlackBishop);

    res.add_from_file("src/assets/rook_w.png", Index::WhiteRook);
    res.add_from_file("src/assets/rook_b.png", Index::BlackRook);

    res.add_from_file("src/assets/king_w.png", Index::WhiteKing);
    res.add_from_file("src/assets/king_b.png", Index::BlackKing);

    res.add_from_file("src/assets/queen_w.png", Index::WhiteQueen);
    res.add_from_file("src/assets/queen_b.png", Index::BlackQueen);
}   

