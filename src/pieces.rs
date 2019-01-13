#![allow(dead_code)]
#![allow(unused_variables)]
extern crate sfml;

extern crate futures;
use self::futures::future;
use self::futures::future::*;
use color::Color;

use sfml::graphics::{Sprite, Transformable};
use sfml::system::Vector2f;
use recorder::*;

use new_index::*;

unsafe impl<'a> Sync for Piece<'a>{}

pub struct Piece<'a> 
{
    pub sprite: Sprite<'a>,
    pub color: Color,
    pub rule: Box<ChessPiece>,
}

impl<'a> Piece<'a> {
    pub fn new(typ: Box<ChessPiece>, color: Color) -> Piece<'a> 
    {
        Piece {
            sprite: Sprite::new(),
            color: color,
            rule: typ,
        }
    }
    pub fn set_position(&mut self, s: (u8, u8), square_size: f32, start: &Vector2f) 
    {
        let x = start.x + (s.0 as f32 * square_size as f32);
        let y = start.y + (s.1 as f32 * square_size as f32);

        self.sprite.set_position(Vector2f::new(x, y));
    }

    pub fn color(&self) -> &Color 
    {
        &self.color
    }

    pub fn try_move(
        &self,
        rec: &Recorder,
        curr_square: &Square,
        new_square: &Square,
    ) -> FutureResult<Option<Square>, ()> 
    {
        return self.rule.mov(rec, curr_square, new_square, self.color.clone());
    }
    
    pub fn get_type(&self) -> _Index<Color>
    {
        self.rule.get_type(&self.color)    
    }
    
}




use square::Square;
pub trait ChessPiece 
{
    fn mov<'a>(
        &self,
        rec: &Recorder,
        curr_square: &Square,
        new_square: &Square,
        color: Color,
    ) -> FutureResult<Option<Square>, ()> 
    {
        future::err(())
    }

    fn get_type(&self, color: &Color) -> _Index<Color>
    {
        unimplemented!()
    }
}

#[derive(PartialEq)]
pub struct Pawn;
impl Pawn 
{
    pub fn new() -> Box<ChessPiece> 
    {
        Box::new(Pawn {} )
    }
}
impl ChessPiece for Pawn 
{
    fn mov<'a>(
        &self,
        rec: &Recorder,
        curr_square: &Square,
        new_square: &Square,
        color: Color,
    ) -> FutureResult<Option<Square>, ()> 
    {
        use mov_functions::pawn;
        pawn::mov(self, rec, curr_square, new_square, color)
    }

    fn get_type(&self, c: &Color) -> _Index<Color>
    {
        _Index::Pawn(c.clone())
    }

}

#[derive(PartialEq)]
pub struct Rook;
impl Rook 
{
    pub fn new() -> Box<ChessPiece> 
    {
        Box::new(Rook {})
    }
}

impl ChessPiece for Rook 
{
    fn get_type(&self, c: &Color) -> _Index<Color>
    {
        _Index::Rook(c.clone())
    }
    fn mov<'a>(
        &self,
        rec: &Recorder,
        curr_square: &Square,
        new_square: &Square,
        color: Color,
    ) -> FutureResult<Option<Square>, ()> 
    {
        use mov_functions::rook;
        rook::mov(rec, curr_square, new_square, color)
    }
}

#[derive(PartialEq)]
pub struct Knight;
impl Knight 
{
    pub fn new() -> Box<ChessPiece> 
    {
        Box::new(Knight {})
    }

}
impl ChessPiece for Knight 
{

    fn get_type(&self, c: &Color) -> _Index<Color>
    {
        _Index::Knight(c.clone())
    }
    fn mov<'a>(
        &self,
        rec: &Recorder,
        curr_square: &Square,
        new_square: &Square,
        color: Color,
    ) -> FutureResult<Option<Square>, ()> 
    {
        use mov_functions::knight;
        knight::mov(rec, curr_square, new_square, color)
    }
}

#[derive(PartialEq)]
pub struct Bishop;
impl Bishop 
{
    pub fn new() -> Box<ChessPiece> 
    {
        Box::new(Bishop {})
    }
}
impl ChessPiece for Bishop 
{

    fn get_type(&self, c: &Color) -> _Index<Color>
    {
        _Index::Bishop(c.clone())
    }
    fn mov<'a>(
        &self,
        rec: &Recorder,
        curr_square: &Square,
        new_square: &Square,
        color: Color,
    ) -> FutureResult<Option<Square>, ()> 
    {
        use mov_functions::bishop;
        bishop::mov(rec, curr_square, new_square, color)
    }
}

#[derive(PartialEq)]
pub struct King;
impl King 
{
    pub fn new() -> Box<ChessPiece> 
    {
        Box::new(King {})
    }
}

impl ChessPiece for King 
{

    fn get_type(&self, c: &Color) -> _Index<Color>
    {
        _Index::King(c.clone())
    }
    fn mov<'a>(
        &self,
        rec: &Recorder,
        curr_square: &Square,
        new_square: &Square,
        color: Color,
    ) -> FutureResult<Option<Square>, ()> 
    {
        use mov_functions::king;
        king::mov(rec, curr_square, new_square, color)
    }
}

#[derive(PartialEq)]
pub struct Queen;
impl Queen {
    pub fn new() -> Box<ChessPiece> 
    {
        Box::new(Queen {})
    }
}

impl ChessPiece for Queen 
{
    fn get_type(&self, c: &Color) -> _Index<Color>
    {
        _Index::Queen(c.clone())
    }
    fn mov<'a>(
        &self,
        rec: &Recorder,
        curr_square: &Square,
        new_square: &Square,
        color: Color,
    ) -> FutureResult<Option<Square>, ()> 
    {
        use mov_functions::queen;
        queen::mov(rec, curr_square, new_square, color)
    }
}
