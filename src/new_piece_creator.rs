
extern crate futures;
use self::futures::future;
use self::futures::future::*;

use square::Square;

use pieces::{Piece, ChessPiece, Pawn, Knight, Bishop, Rook, King, Queen};
use color::Color;
use resources::Resources;

extern crate sfml;
use sfml::graphics::{RenderWindow, RenderTarget, Transformable};
use sfml::system::Vector2f;

use chess_set::ChessSet;

use new_index::_Index;
use KEY;

pub fn m_create_kings<'a>(set: &mut ChessSet)
{
    p_create_king(set, _Index::King(Color::White));
    p_create_king(set, _Index::King(Color::Black));
}

pub fn p_create_king<'a>(set: &mut ChessSet, text: _Index<Color>)
{
    let color = text.get();
    let square = match &color { &Color::White => Square::new(4, 7), _ => Square::new(4, 0) };
    let piece = _create_piece(King::new(), color)
                    .and_then(|p| set_texture(p, set.resource(), set.scale(), &text)).wait().unwrap();
    
    set.place(piece, square); 
}
pub fn m_create_queens(set: &mut ChessSet)
{
    p_create_queen(set, _Index::Queen(Color::White));
    p_create_queen(set, _Index::Queen(Color::Black));
}

pub fn p_create_queen(set: &mut ChessSet, text: _Index<Color>)
{
    let color = text.get();
    let square = match &color { &Color::White => Square::new(3, 7), _ => Square::new(3, 0) };
    let piece = _create_piece(Queen::new(), color)
                    .and_then(|p| set_texture(p, set.resource(), set.scale(), &text)).wait().unwrap();
    set.place(piece, square); 
}

pub fn m_create_pawns(set: &mut ChessSet)
{
    p_create_pawns(set, Color::White);
    p_create_pawns(set, Color::Black);
}

fn p_create_pawns(set: &mut ChessSet, color: Color)
{
    let (y_square, text) = match &color { &Color::White => (6, _Index::Pawn(Color::White)), 
                                                      _ => (1, _Index::Pawn(Color::Black)) };
    for i in 0..=7
    {
        let piece = _create_piece(Pawn::new(), color.clone())
                        .and_then(|p| 
                                  set_texture(p, set.resource(), set.scale(), &text)
                                  ).wait().unwrap();
        let square = Square::new(i, y_square);
        set.place(piece, square);
    }
}

pub fn m_create_bishop(set: &mut ChessSet)
{
    let texture = (_Index::Bishop(Color::White), _Index::Bishop(Color::Black));
    let x_square = (2, 5);
    let create_func = Bishop::new;
    m_create_officers(set, texture, x_square, &create_func); 
}
pub fn m_create_rooks(set: &mut ChessSet)
{
    let texture = (_Index::Rook(Color::White), _Index::Rook(Color::Black));
    let x_square = (0, 7);
    let create_func = Rook::new;
    m_create_officers(set, texture, x_square, &create_func); 
}

pub fn m_create_knights(set: &mut ChessSet)
{
    let texture = (_Index::Knight(Color::White), _Index::Knight(Color::Black));
    let x_square = (1, 6);
    let create_func = Knight::new;
    m_create_officers(set, texture, x_square, &create_func); 
}

fn m_create_officers(
    set: &mut ChessSet,
    texture: (_Index<Color>, _Index<Color>), 
    x_square: (u8, u8),
    create_func: &Fn() -> Box<ChessPiece>)
{
    p_create_officers(set, &texture.0, x_square.0, &create_func);
    p_create_officers(set, &texture.0, x_square.1, &create_func);

    p_create_officers(set, &texture.1, x_square.0, &create_func);
    p_create_officers(set, &texture.1, x_square.1, &create_func);
}
fn p_create_officers(
    set: &mut ChessSet, 
    text: &_Index<Color>, 
    x_square: u8, 
    create_func: &Fn() -> Box<ChessPiece>
    )
{
    let color = text.get(); 
    let y = match &color { &Color::White => 7, _ => 0 };
    let piece1 = _create_piece(create_func(), color)
                        .and_then(|p| set_texture(p, set.resource(), set.scale(), text)).wait().unwrap();
    
    let square = Square::new(x_square, y);
    set.place(piece1, square);
}

fn _create_piece<'a>(p: Box<ChessPiece>, color: Color) -> FutureResult<Piece<'a>, ()>
{
    ok(Piece::new(p, color))
}


pub fn m_create_piece<'a>(rec: &'a Resources<KEY>, scale: f32, index: &_Index<Color>) -> Piece<'a>
{
    let piece_type = match index
    {
        _Index::Pawn(_) => Pawn::new(),
        _Index::Knight(_) => Knight::new(),
        _Index::Bishop(_) => Bishop::new(),
        _Index::Rook(_) => Rook::new(),
        _Index::Queen(_) => Queen::new(),
        _Index::King(_) => King::new(),
         
        _ => unreachable!()
    };
     
    _create_piece(piece_type, index.get())
                .and_then(|p| set_texture(p, rec, scale, index)).wait().unwrap()
}


pub fn set_texture<'a>(mut piece: Piece<'a>, res: &'a Resources<KEY>, scale: f32, index: &_Index<Color>)
-> FutureResult< Piece<'a>, ()>
{
    let piece_pr_square_ratio = 0.37; // Magic number??
    if let Some(texture) = res.get(index)
    {
        piece.sprite.set_texture(texture, false);
        piece.sprite.set_scale(Vector2f::new(
                        piece_pr_square_ratio * scale, 
                        piece_pr_square_ratio * scale));
    }
    future::ok(piece)
}



use sfml::graphics::Sprite;
pub fn create_board<'a>(res: &'a Resources<KEY>, window: &RenderWindow) -> (Sprite<'a>, f32)
{
    let mut board = Sprite::new();
    let mut board_scale = 1.0;        
    if let Some(board_texture) = res.get(&_Index::Board) 
    {
        /*
            * First initialize the board, this means getting the texture, and setting the
            * appropritate size
            */
        board.set_texture(board_texture, false);
        let window_size = window.size();
        let bounds = board.global_bounds();



        // scale size appropriate to height 
        board_scale = window_size.y as f32 / bounds.height;
        if board_scale != 1.0
        {
            board.set_scale( Vector2f::new(board_scale, board_scale)); 
        }
        // Center the window
        let x_diff = window_size.x as f32 - bounds.width * board_scale;
        if x_diff != 0.0
        {
            let new_pos = Vector2f::new( bounds.left + x_diff / 2.0, bounds.top);
            board.set_position( new_pos ); 
        }
    }
    (board, board_scale)
}
