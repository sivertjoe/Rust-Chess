
extern crate futures;
use self::futures::future;
use self::futures::future::*;


use std::collections::HashMap;
use square::Square;

use pieces::{Piece, ChessPiece, Pawn, Knight, Bishop, Rook, King, Queen};
use color::Color;
use index::Index;
use recourses::Recourses;

extern crate sfml;
use sfml::graphics::{RenderWindow, RenderTarget, Transformable};
use sfml::system::Vector2f;


fn _create_piece<'a>(p: Box<ChessPiece>, color: Color) -> FutureResult<Piece<'a>, ()>
{
    ok(Piece::new(p, color))
}


pub fn place_on_board<'a>(mut map: HashMap<Square, Piece<'a>>, piece: Piece<'a>, s: Square)
-> Result<Loop< HashMap<Square, Piece<'a>>, HashMap<Square, Piece<'a>> >, ()>
{
    map.insert(s, piece);
    match map.len() % 8
    {
        0 => Ok(Loop::Break(map)),
        _ => Ok(Loop::Continue(map)),
    }
}



pub fn create_pawns<'a>(map: HashMap<Square, Piece<'a>>, 
                        res: &'a Recourses<Index>, 
                        color: Color, 
                        scale: f32) 
-> FutureResult< HashMap<Square, Piece<'a>>, ()>
{
    let index = match &color { &Color::White => Index::WhitePawn, _ => Index::BlackPawn };
    future::ok(
        loop_fn(map, |map|
        {
            _create_piece(Pawn::new(), color.clone())
            .and_then(|p| set_texture(p, res, scale, &index))
            .and_then(|p| 
            {
                let y_square = match color { Color::White => 6, _ => 1};
                let x_square = (map.len() % 8) as u8;
                let s = Square::new(x_square, y_square);
                place_on_board(map, p, s)
            })        
        }).wait().unwrap()
        )
}




pub fn create_knights<'a>(map: HashMap<Square, Piece<'a>>, 
                          res: &'a Recourses<Index>, 
                          color: Color, 
                          scale: f32) 
-> FutureResult<HashMap<Square, Piece<'a>>, ()>
{
    let (y_pos, texture) = match &color 
    { 
        &Color::White => (7, Index::WhiteKnight), 
        _ => (0, Index::BlackKnight) 
    };
    
    create_officers(
        map,
        res,
        color,
        texture,
        scale,
        (Square::new(1, y_pos), Square::new(6, y_pos)),
        &_create_piece,
        &Knight::new
        )
    
}


pub fn create_rooks<'a>(map: HashMap<Square, Piece<'a>>, 
                       res: &'a Recourses<Index>, 
                       color: Color, 
                       scale: f32) 
-> FutureResult<HashMap<Square, Piece<'a>>, ()>
{
    let (y_pos, texture) = match &color
    {
        &Color::White => (7, Index::WhiteRook),
        _ => (0, Index::BlackRook)
    };
    create_officers(
        map,
        res,
        color,  
        texture,
        scale,
        (Square::new(0, y_pos), Square::new(7, y_pos)),
        &_create_piece,
        &Rook::new
        )
}



pub fn create_bishops<'a>(map: HashMap<Square, Piece<'a>>, 
                          res: &'a Recourses<Index>, 
                          color: Color, 
                          scale: f32) 
-> FutureResult<HashMap<Square, Piece<'a>>, ()>
{
    let (y_pos, texture) = match &color
    {
        &Color::White => (7, Index::WhiteBishop),
        _ => (0, Index::BlackBishop),
    };
    create_officers(
        map,
        res,
        color,
        texture,
        scale,
        (Square::new(2, y_pos), Square::new(5, y_pos)),
        &_create_piece,
        &Bishop::new
        )
}



fn create_officers<'a>(mut map: HashMap<Square, Piece<'a>>,
                      res: &'a Recourses<Index>,
                      color: Color,
                      texture: Index,
                      scale: f32,
                      pos: (Square, Square),
                      create_func: &Fn(Box<ChessPiece>, Color) -> FutureResult<Piece<'a>, ()>,
                      create_piece: &Fn() -> Box<ChessPiece>)
-> FutureResult<HashMap<Square, Piece<'a>>, ()>
{

    let piece1 = create_func(create_piece(), color.clone())
                .and_then(|p| set_texture(p, res, scale, &texture)).wait().unwrap();


    let piece2 = create_func(create_piece(), color)
                .and_then(|p| set_texture(p, res, scale, &texture)).wait().unwrap();

    map.insert(pos.0, piece1);
    map.insert(pos.1, piece2);
    future::ok(map)
}



pub fn set_texture<'a>(mut piece: Piece<'a>, res: &'a Recourses<Index>, scale: f32, index: &Index)
-> FutureResult< Piece<'a>, ()>
{
    let piece_pr_square_ratio = 0.37;
    if let Some(texture) = res.get(index)
    {
        piece.sprite.set_texture(texture, false);
        piece.sprite.set_scale(
            Vector2f::new(piece_pr_square_ratio * scale, piece_pr_square_ratio * scale));
    }
    future::ok(piece)
}



pub fn _set_position(piece: &mut Piece, pos: &Square, window: &RenderWindow) -> FutureResult<bool, ()>
{
    let bounds = window.size();
    let startx = ((bounds.x - bounds.y) as f32) / 2.0;
    let pos = (pos.col, pos.row);
    let square_size = bounds.y as f32 / 8.0; 
    piece.set_position(pos, square_size, &Vector2f::new(startx, 0.0));
    future::ok(true)
}


pub fn set_position<'a>(mut map: HashMap<Square, Piece<'a>>, window: &RenderWindow) 
->FutureResult<HashMap<Square, Piece<'a>>, ()>
{ 
    loop_fn(map.iter_mut(), |mut iter|
    {
        let (square, piece) = iter.next().unwrap();
        _set_position(piece, square, window)
        .and_then(|_|
        {
            match iter.size_hint()
            {
                (0, Some(0)) => Ok(Loop::Break(iter)),
                _ => Ok(Loop::Continue(iter))
            }
        
        })
    }).wait().unwrap();
    future::ok(map)
}




pub fn create_queen<'a>(map: HashMap<Square, Piece<'a>>, res: &'a Recourses<Index>, color: Color, scale: f32) 
-> FutureResult<HashMap<Square, Piece<'a>>, ()>
{
    let (y_square, texture) = match &color
    {
        &Color::White => (7, Index::WhiteQueen),
        _ => (0, Index::BlackQueen)
    };
    create_single_piece(map, res, color, scale, Square::new(3, y_square), 
                        texture, &_create_piece, &Queen::new)
}


fn create_single_piece<'a>(
                        mut map: HashMap<Square, Piece<'a>>, 
                        res: & 'a Recourses<Index>, 
                        color: Color, 
                        scale: f32,
                        square: Square,
                        texture: Index,
                        create_func: &Fn(Box<ChessPiece>, Color) -> FutureResult<Piece<'a>, ()>,
                        create_piece:&Fn() -> Box<ChessPiece>)

-> FutureResult<HashMap<Square, Piece<'a>>, ()>
{
    let piece = create_func(create_piece(), color)
                    .and_then(|p| set_texture(p, res, scale, &texture)).wait().unwrap();
    map.insert(square, piece);
    ok(map)
}




pub fn create_king<'a>(map: HashMap<Square, Piece<'a>>, res: &'a Recourses<Index>, color: Color, scale: f32) 
-> FutureResult<HashMap<Square, Piece<'a>>, ()>
{
    let (y_square, texture) = match &color
    {
        &Color::White => (7, Index::WhiteKing),
        _ => (0, Index::BlackKing)
    };
    create_single_piece(map, res, color, scale, Square::new(4, y_square), 
                        texture, &_create_piece, &King::new)
}



use sfml::graphics::Sprite;
pub fn create_board<'a>(res: &'a Recourses<Index>, window: &RenderWindow) 
-> (Sprite<'a>, f32)
{
    let mut board = Sprite::new();
    let mut board_scale = 1.0;        
    if let Some(board_texture) = res.get(&Index::Board) 
    {
        /*
            * First initialize the board, this means getting the texture, and setting the
            * appropritate size
            */
        board.set_texture(board_texture, false);
        let window_size = window.size();
        let bounds = board.global_bounds();



        // scale size appropriate to height 
        let scale = window_size.y as f32 / bounds.height;
        if scale != 1.0
        {
            board.set_scale( Vector2f::new(scale, scale)); 
            board_scale = scale;
        }
        // Center the window
        let x_diff = window_size.x as f32 - bounds.width * scale;
        if x_diff != 0.0
        {
            let new_pos = Vector2f::new( bounds.left + x_diff / 2.0, bounds.top);
            board.set_position( new_pos ); 
        }
    }
    (board, board_scale)
}
