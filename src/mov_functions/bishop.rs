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
    if new_square.col == curr_square.col || 
        ((new_square.col + new_square.row) % 2) != ((curr_square.col + curr_square.row) % 2)

    {
        return future::err(());
    }
    
    let mut inc = utility::square_diff(new_square, curr_square);
    if inc.0 < 0
    {
        inc.0 = -1;
    }
    else
    {
        inc.0 = 1;
    }
    if inc.1 < 0
    {
        inc.1 = -1;
    }
    else
    {
        inc.1 = 1;
    }
    let mut temp_square = curr_square.clone();
    temp_square.inc(inc.0, inc.1);
    while &temp_square != new_square
    {
        if rec.board().contains_key(&temp_square)
        {
            return future::err(());
        }
        temp_square.inc(inc.0, inc.1);
    }
    if let Some(piece) = rec.board().get(&new_square)
    {
        return match piece.color == color
        {
            true => future::err(()),
            _ => future::ok(None),

        };
        
    }

   future::ok(None)
}
