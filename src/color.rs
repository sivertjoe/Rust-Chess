#[derive(Clone)]
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
