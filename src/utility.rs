use square::Square;
use sfml::window::mouse; 
use sfml::system::Vector2f; 
use sfml::graphics::{RenderWindow, RenderTarget};

pub fn get_square(window: &mut RenderWindow) -> Square
{
    let board_pos = mouse::desktop_position() - window.position();

    let square_size = window.size().y as i32 / 8;

    let row = (board_pos.x / square_size) as u8;
    let col = (board_pos.y / square_size) as u8;
   
    
    Square::new(row, col)
}

pub fn get_mousemid(window: &mut RenderWindow) -> Vector2f
{
    let square_size = window.size().y as f32 / 16.0;
    let pos = window.mouse_position();
    Vector2f::new(
        pos.x as f32 - square_size as f32, 
        pos.y as f32 - square_size as f32
        )
}


pub fn get_boardpos(window: &mut RenderWindow, square: &Square) -> Vector2f
{
    let square_size = window.size().y as f32 / 8.0;
    Vector2f::new(
        square.col as f32 * square_size, 
        square.row as f32 * square_size
        )
}
