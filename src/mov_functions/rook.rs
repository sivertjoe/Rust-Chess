extern crate futures;
use self::futures::future;
use self::futures::future::*;

use square::Square;
use utility;
use recorder::Recorder;
use color::Color;

pub fn mov(
        rec: &Recorder,
        curr_square: &Square,
        new_square: &Square,
        color: Color,
        ) -> FutureResult<Option<Square>, ()>
{
    let dsquare = utility::square_diff(&new_square, &curr_square);
        
    if dsquare.0 == 0 && dsquare.1 == 0
    {
        return future::err(());
    }

    if dsquare.0 != 0 && dsquare.1 != 0
    {
        return future::err(());
    }

    let iter = match dsquare.0
    {
        0 => (0, dsquare.1 / dsquare.1.abs()),
        _ => (dsquare.0 / dsquare.0.abs(), 0),
    };
    let mut temp_square = curr_square.clone();
    let check = |temp_square: &Square, n: &(i8, i8)| -> bool 
    { 
        if temp_square.col as i8 + n.0 < 0 || temp_square.col as i8 + n.0 > 7
            || temp_square.row as i8 + n.1 < 0 || temp_square.row as i8 + n.1 > 7
        {
            return true;
        }
        false
    
    };
    if check(&temp_square, &iter)
    {
        return future::err(());
    }
    temp_square.inc(iter.0, iter.1);
    loop
    {
        if let Some(piece) = rec.board().get(&temp_square)
        {
            if &piece.borrow().color == !&color && &temp_square == new_square
            {
                return future::ok(None);
            }
            return future::err(());
        }
        if &temp_square == new_square
        {
            return future::ok(None);
        }
        if check(&temp_square, &iter)
        {
            break;
        }
        temp_square.inc(iter.0, iter.1);
    }   
    
    return future::err(());;
}

