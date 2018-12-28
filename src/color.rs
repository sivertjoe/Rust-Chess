#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Color
{
    White,
    Black
}

impl std::ops::Not for Color
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
impl std::ops::Not for &Color
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

