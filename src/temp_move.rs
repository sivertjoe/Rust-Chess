use pieces::Piece;
use square::Square;

use std::rc::Rc;
use std::cell::RefCell;

pub struct TempMove<'a>
{
    pub piece: Option<Rc<RefCell<Piece<'a>>>>,
    pub old_pos: Option<Square>

}

impl<'a> TempMove<'a>
{
   pub fn new() -> Self
    {
        TempMove {
            piece: None,
            old_pos: None
        }
    }


    pub fn set(&mut self, piece: Option<Rc<RefCell<Piece<'a>>>>, square: Option<Square>)
    {
        self.piece = piece;
        self.old_pos = square;
    }

    pub fn is_some(&self) -> bool
    {
        self.piece.is_some()
    }

    pub fn as_mut(&mut self) -> Option<&mut Rc<RefCell<Piece<'a>>>>
    {
        self.piece.as_mut()
    }
     
    pub fn as_ref(&self) -> Option<&Rc<RefCell<Piece<'a>>>>
    {
        self.piece.as_ref()
    }
    pub fn square(&self) -> Option<&Square>
    {
        self.old_pos.as_ref()
    }

    pub fn take_square(&mut self) -> Option<Square>
    {
        self.old_pos.take()
    }
}
