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









