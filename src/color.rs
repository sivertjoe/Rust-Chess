#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Color
{
    White,
    Black
}

use std::ops::Not;
impl Not for Color
{
    type Output = Self;
    fn not(self) -> Self::Output
    {
        match self
        {
            Color::White => Color::Black,
            _ => Color::White
        }
    }
}

impl<'a> Not for &'a Color
{
    type Output = Self;
    fn not(self) -> Self::Output
    {
        match self
        {
            &Color::White => &Color::Black,
            _ => &Color::White
        }
    }
}

