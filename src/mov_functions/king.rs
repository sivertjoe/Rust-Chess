extern crate futures;
use self::futures::future;
use self::futures::future::*;

use square::Square;
use utility;
use recorder::Recorder;
use color::Color;
pub fn mov<'a>(
    rec: &Recorder,
    curr_square: &Square,
    new_square: &Square,
    color: Color,
) -> FutureResult<Option<Square>, ()> 
{
    let diff_square = utility::square_diff(curr_square, new_square);
    if diff_square.0 == 0 && diff_square.1 == 0
    {
        return future::err(());
    }
    if diff_square.0 <= 1 && diff_square.1 <= 1
    {
        if let Some(piece) = rec.board().get(new_square)
        {
            if &piece.color == !&color
            {
                return future::ok(None);
            }
            return future::err(());
        }
        return future::ok(None);
    }
    return future::err(());
}
