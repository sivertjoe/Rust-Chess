#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum _Index<T>
where T: PartialEq + Eq + std::hash::Hash
{
    Board,
    King(T),
    Queen(T),

    Rook(T),
    Bishop(T),
    Knight(T),
    Pawn(T)
}

impl<T> _Index<T>
    where T: Clone + Eq + PartialEq + std::hash::Hash
{
    pub fn get(&self) -> T 
    {
        use _Index::*;
        match self
        {
            &King(ref v) => v.clone(),
            &Queen(ref v) => v.clone(),
            &Rook(ref v) => v.clone(),
            &Bishop(ref v) => v.clone(),
            &Knight(ref v) => v.clone(),
            &Pawn(ref v) => v.clone(),
            _ => unreachable!()
        }
    }

}
