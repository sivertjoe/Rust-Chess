extern crate futures;
use self::futures::future;
use self::futures::future::*;

use new_index::_Index;
use square::Square;
use pieces::Pawn;
use recorder::{Recorder, ChessSet};
use color::Color;

pub fn mov(
        _chess_piece: &mut Pawn,
        rec: &mut Recorder,
        curr_square: &Square,
        new_square: &Square,
        color: Color,
        ) -> FutureResult<Option<Square>, ()>
{
    let n_hops = (curr_square.row as i32 - new_square.row as i32).abs(); 
    if n_hops > 2
    {
        return future::err(());
    }
    match &color
    {
        &Color::White => if new_square.row > curr_square.row { return future::err(()); },
        _ => if new_square.row < curr_square.row { return future::err(()); },
    };

    let board = rec.board();
    let mask_color = match &color 
    {
        &Color::White => -1,
        _ => 1,
    };

    let mut temp_square = Square::new(9, 9);
    let first_move_square = match &color
    {
        &Color::White => 6,
        _ => 1,
    };
    if curr_square.row == first_move_square
    {
        temp_square.set(curr_square.col, curr_square.row as i32 + 2 * mask_color);
        if new_square == &temp_square && board.get(&temp_square).is_none()
        {
            temp_square.inc(0, -1 * mask_color);
            if board.get(&temp_square).is_none()
            {
                return future::ok(None);
            }
        }
    }
    temp_square.set(curr_square.col, curr_square.row as i32 + 1 * mask_color);
    if new_square == &temp_square && board.get(&temp_square).is_none()
    {
        return future::ok(None);
    }
    
    if curr_square.col > 0 
    {
        temp_square.set(curr_square.col as i32 - 1, curr_square.row as i32 + 1 * mask_color);
        if let Some(piece) = rec.board().get(&temp_square)
        {
            if new_square == &temp_square && &piece.color == !&color
            {
                return future::ok(None);
            }
        }
    }
    if curr_square.col < 7
    {
        temp_square.set(curr_square.col as i32 + 1, curr_square.row as i32 + 1 * mask_color);
        if let Some(piece) = rec.board().get(&temp_square)
        {
            if new_square == &temp_square && &piece.color == !&color
            {
                return future::ok(None);
            }
        }
    }

    
    let last_move = rec.get(0);
    if let Some(m) = last_move
    {
        let en_passant_square = match &color { &Color::White => 3, _ => 4 };

        if curr_square.row == en_passant_square
        {
            let orgin = m.from().clone();
            let mut legal_en_passant = orgin.clone();
            legal_en_passant.inc(0, -1 * mask_color);

            if &legal_en_passant == new_square
            {
                let enemy_color = !color.clone();
                match board.get(m.to())
                {
                    Some(p) => if p.get_type() == _Index::Pawn(enemy_color) 
                    {
                        return future::ok(Some(m.to().clone()));
                    }
                        _ => {}
                }
            }
        }

    }
    future::err(())
}
