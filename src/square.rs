extern crate num;
use self::num::ToPrimitive;


unsafe impl Sync for Square{}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Square
{
    pub row: u8,
    pub col: u8,
}


// col is the file, 0 means a files etc.
// row is number 
// col: 5, row: 4 = e4
#[allow(dead_code)]
impl Square
{
    pub fn new<T, U>(col: T, row: U) -> Square
        where T: ToPrimitive,
              U: ToPrimitive
    {
        Square { 
            col: col.to_u8().expect("Error while setting col"), 
            row: row.to_u8().expect("Error while setting row")
        }
    }
    pub fn set<T, V>(&mut self, col: T, row: V) 
        where T: ToPrimitive,
              V: ToPrimitive
    {
        self.col = col.to_u8().expect("Error while setting square (1)");
        self.row = row.to_u8().expect("Error while setting square (2)");
    }

    pub fn inc<T, V>(&mut self, col: T, row: V) 
        where T: ToPrimitive,
              V: ToPrimitive
    {
        self.col =( self.col as i32 + col.to_i32().expect("!1!") ) as u8;
        self.row =( self.row as i32 + row.to_i32().expect("!2!") ) as u8;
    }


    pub fn from_nontation<'a, S: Into<&'a str>>(notation: S) -> Square
    {
        let notation = notation.into().as_bytes();
        let col = notation[0] - 97;
        let row = 8 - (notation[1] - 48);

        Self::new(col, row)
    }
}

use std::fmt;
impl fmt::Display for Square
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result
    {
        let letter: char = match self.col
        {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            3 => 'd',
            4 => 'e',
            5 => 'f',
            6 => 'g',
            7 => 'h',
            _ => unreachable!(),
        };

        let pos = 8 - self.row;

        write!(fmt, "{}{}", letter, pos)
    }
}


#[test]
fn correct_square_display()
{
    let h1 = Square::new(7, 7);
    let e4 = Square::new(4, 4);
    let d8 = Square::new(3, 0);

    assert_eq!("h1".to_owned(), format!("{}", h1));
    assert_eq!("e4".to_owned(), format!("{}", e4));
    assert_eq!("d8".to_owned(), format!("{}", d8));
}

#[test]
fn from_str()
{
    let square = format!("{}", Square::new(5, 5));
    assert_eq!("f3".to_owned(), square);
}






