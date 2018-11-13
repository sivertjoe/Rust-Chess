extern crate sfml;
use sfml::system::Vector2f;
use sfml::graphics::{RenderWindow, RenderTarget, Transformable};

use piece_creator::*;
use board::Board;
use square::Square;
use pieces::{Piece, Pawn, Knight, Bishop, Rook};
use color::Color;
use recourses::Recourses;

use sfml::graphics::Sprite;
use std::collections::HashMap;
use index::Index;
pub struct Game<'a>
{
    board: Board<'a>,
    scale: f32, 
}

impl<'a> Game<'a>
{
    pub fn new(res: &'a Recourses<Index>, window: &RenderWindow) -> Game<'a>
    {
        let (board, scale) = create_board(res, window);

        Game {
            scale: scale,
            board: Board::new(create_piece_set(res, window, scale), board) }
    }
    

    pub fn display(&self, window: &mut RenderWindow)
    {
        window.draw(&self.board.board);
        self.board.squares
            .values()
            .for_each(|piece| window.draw(&piece.sprite ));
    }

    pub fn update(&self, window: &mut RenderWindow)
    {
        self.display(window);
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

        .and_then(|map| create_rook(map, res, Color::White, scale))
        .and_then(|map| create_rook(map, res, Color::Black, scale))
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
}   

