extern crate sfml;
use sfml::system::Vector2u;

use pieces::Piece;
use square::Square;
use r#move::Move;
use KEY;
use resources::Resources;

pub trait ChessSet<'a>
{
    fn place(&mut self, Piece<'a>, Square);
    fn place_multiple(&mut self, vec: Vec<Piece<'a>>, s: Vec<Square>);
    
    fn record(&mut self, Move);
    fn get(&self, usize) -> Option<&Move>;
    fn undo(&mut self);
    fn redo(&mut self);

    // Utility
    fn init(&mut self);
    fn resource(&self) -> &'a Resources<KEY>;
    fn scale(&self) -> f32;
    fn board_size(&self) -> Vector2u;
}
