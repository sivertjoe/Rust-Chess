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
    let mut dsquare = utility::square_diff(&new_square, &curr_square);
    dsquare.0 = dsquare.0.abs();
    dsquare.1 = dsquare.1.abs();

    if dsquare.0 == 1 && dsquare.1 == 2 || dsquare.0 == 2 && dsquare.1 == 1
    {
        if let Some(piece) = rec.board().get(new_square)
        {
            return match piece.borrow().color == color
            {
                true => future::err(()),
                _ => future::ok(None),
            }
        }
        return future::ok(None);
    }
    future::err(())
}
