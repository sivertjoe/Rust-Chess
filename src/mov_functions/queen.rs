extern crate futures;
use self::futures::future;
use self::futures::future::*;
use square::Square;
use recorder::Recorder;
use color::Color;

use self::futures::Async;
pub fn mov<'a>(
    rec: &Recorder,
    curr_square: &Square,
    new_square: &Square,
    color: Color,
) -> FutureResult<Option<Square>, ()> 
{
    use mov_functions::bishop;
    use mov_functions::rook;
    match bishop::mov(rec, curr_square, new_square, color.clone()).poll()
    {
        Ok(Async::Ready(s)) => return future::ok(s),
        _ => {}
    };
    rook::mov(rec, curr_square, new_square, color.clone())
}
