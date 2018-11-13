extern crate sfml;
use sfml::graphics::{Sprite};

use std::collections::HashMap;


use pieces::Piece;
use square::Square;

#[allow(dead_code)]
pub struct Board<'a>
{
    pub squares: HashMap<Square, Piece<'a>>,
    pub board  : Sprite<'a>,
}

#[allow(dead_code)]
impl<'a> Board<'a>
{
    pub fn new(map: HashMap<Square, Piece<'a>>, b: Sprite<'a>) -> Board<'a>
    {
        Board { 
            squares: map, 
            board  : b}
    }


}


