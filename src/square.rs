#[derive(PartialEq, Eq, Hash)]
pub struct Square
{
    pub row: u8,
    pub col: u8,
}

#[allow(dead_code)]
impl Square
{
    pub fn new(col: u8, row: u8) -> Square
    {
        Square { col: col, row: row }
    }
}

impl std::fmt::Display for Square
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result
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
            _ => unreachable!()
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








