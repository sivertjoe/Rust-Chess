extern crate futures;
use self::futures::future;
use self::futures::future::*;
use new_index::_Index;
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
    let mut diff_square = utility::square_diff(curr_square, new_square);
    diff_square.0 = diff_square.0.abs();
    diff_square.1 = diff_square.1.abs();

    if diff_square.0 == 0 && diff_square.1 == 0
    {
        return future::err(());
    }
    if diff_square.0 <= 1 && diff_square.1 <= 1
    {
        if let Some(piece) = rec.board().get(new_square)
        {
            if &piece.borrow().color == !&color
            {
                return future::ok(None);
            }
            return future::err(());
        }
        return future::ok(None);
    }
    // Check for castle
    let y = match &color
    {
        &Color::White => 7,
        _ => 0,
            
    };
    let short = Square::new(6, y);
    let long = Square::new(2, y);
    if new_square == &short || new_square == &long
    {
        let mut rook_pos = short.clone();
        match new_square == &short
        {
            true => rook_pos.col = 7,
            _ => rook_pos.col = 0,
        };
        let king_type = _Index::King(color.clone());
        for m in rec.moves()
        {
            if  m.piece == king_type || m.from == rook_pos // Moved either king, or rook
            {
                return future::err(()); 
            }
        }
        
        return future::ok(Some(rook_pos));

    }

    return future::err(());
}
