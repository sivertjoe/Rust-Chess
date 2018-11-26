extern crate sfml;

use color::Color;

use sfml::graphics::{Sprite, Transformable};
use sfml::system::Vector2f;

pub struct Piece<'a>
{
    pub sprite: Sprite<'a>,
    pub color: Color,
    pub rules : Box<ChessPiece>,
}

impl<'a> Piece<'a>
{
    pub fn new(typ: Box<ChessPiece>, color: Color) -> Piece<'a>
    {
        Piece {
            sprite: Sprite::new(),
            color: color,
            rules: typ,
        }
    }
    pub fn set_position(&mut self, s: (u8, u8), square_size: f32, start: &Vector2f)
    {
        let x = start.x + (s.0 as f32 * square_size as f32);
        let y = start.y + (s.1 as f32 * square_size as f32);

        self.sprite.set_position(Vector2f::new(x, y));
    }

}

pub trait ChessPiece
{
}

pub struct Pawn
{
    first_move: bool,
    just_moved: bool,
}
impl Pawn
{
    pub fn new() -> Box<ChessPiece>
    {
        Box::new( Pawn {
            first_move: true,
            just_moved: true,
        } )
    }

}

impl ChessPiece for Pawn
{

    
}

pub struct Rook;
impl Rook
{
    pub fn new() -> Box<ChessPiece>
    {
        Box::new( Rook {} )
    }
}

impl ChessPiece for Rook {}

pub struct Knight;
impl Knight
{
    pub fn new() -> Box<ChessPiece>
    {
        Box::new( Knight {} )
    }
}
impl ChessPiece for Knight
{
}

pub struct Bishop;
impl Bishop
{
    pub fn new() -> Box<ChessPiece>
    {
        Box::new( Bishop{} )
    }
}
impl ChessPiece for Bishop{}

pub struct King;
impl King
{
    pub fn new() -> Box<ChessPiece>
    {
        Box::new( King {} )
    }
}

impl ChessPiece for King {}

pub struct Queen;
impl Queen
{
    pub fn new() -> Box<ChessPiece>
    {
        Box::new( Queen {} )
    }
}

impl ChessPiece for Queen {}
