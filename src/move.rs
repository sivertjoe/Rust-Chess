use new_index::_Index;
use square::Square;
use color::Color;
use utility;


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


    pub fn to(&self) -> &Square
    {
        &self.to
    }

    pub fn from(&self) -> &Square
    {
        &self.from
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
            &_Index::Pawn(ref _v) => 
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
