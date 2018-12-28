#![allow(dead_code)]
use sfml::graphics::{RenderTarget, RenderWindow};
use sfml::system::{Vector2f, Vector2u};
use sfml::window::mouse;
use square::Square;
use color::Color;
use new_index::_Index;

pub fn get_square(window: &mut RenderWindow) -> Square 
{
    let board_pos = mouse::desktop_position() - window.position();

    let square_size = window.size().y as i32 / 8;

    let row = (board_pos.x / square_size) as u8;
    let col = (board_pos.y / square_size) as u8;

    Square::new(row, col)
}
pub fn _square_from_vec(v: &Vector2f, window_size: &Vector2u) -> Square
{
    let board_pos = v;
    let square_size = window_size.y as i32 / 8;

    let row = (board_pos.x / square_size as f32) as u8;
    let col = (board_pos.y / square_size as f32) as u8;

    Square::new(row, col)
}

pub fn get_square_from_vec(v: &Vector2f, window: &mut RenderWindow) -> Square 
{
    let board_pos = v;
    let square_size = window.size().y as i32 / 8;

    let row = (board_pos.x / square_size as f32) as u8;
    let col = (board_pos.y / square_size as f32) as u8;

    Square::new(row, col)
}

pub fn get_mousemid(window: &mut RenderWindow) -> Vector2f 
{
    let square_size = window.size().y as f32 / 16.0;
    let pos = window.mouse_position();
    Vector2f::new(
        pos.x as f32 - square_size as f32,
        pos.y as f32 - square_size as f32,
    )
}

pub fn get_boardpos(size: &Vector2u, square: &Square) -> Vector2f 
{
    let square_size = size.y as f32 / 8.0;
    Vector2f::new(
        square.col as f32 * square_size,
        square.row as f32 * square_size,
    )
}

pub fn get_mousesquare(window: &mut RenderWindow) -> Square 
{
    let mousepos = get_mousemid(window);
    get_square_from_vec(&Vector2f::new(mousepos.x as f32, mousepos.y as f32), window)
}
use recorder::Move;
use pieces::Piece;
use std::collections::HashMap;

pub fn construct_move<'a>(
    piece: &Piece<'a>, 
    board: &HashMap<Square, Piece<'a>>, 
    new: Square, 
    old: Square
    ) -> Move
{
    let capture = board.get(&new).map_or(None, |p| Some(p.get_type()) );
    Move::new(piece.get_type(), new, old, capture)
}



pub fn square_diff(s1: &Square, s2: &Square) -> (i8, i8)
{
    (s1.col as i8 - s2.col as i8, s1.row as i8 - s2.row as i8)
}

pub fn calculate_en_passant(mov: &Move) -> (_Index<Color>, Square)
{
    let color = mov.piece().get();
    let mask_color = match &color
    {
        &Color::White => 1,
        _ => -1,
    };
    let mut square = mov.to().clone();
    square.inc(0, mask_color);
    (_Index::Pawn(!color), square)
}
