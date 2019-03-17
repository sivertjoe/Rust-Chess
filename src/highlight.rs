use sfml::graphics::{Shape,    Transformable, RectangleShape};
use sfml::system::{ Vector2f, Vector2u};
use square::Square;

use utility;

pub struct Highlight<'a>
{
    shape: RectangleShape<'a>,
}

impl<'a> Highlight<'a>
{
    pub fn new(board_size: Vector2u, s1: &Square) -> Self 
    {
        let square_size = Vector2f::new( 
                                board_size.x as f32 / 8.0, 
                                board_size.y as f32 / 8.0);
        let mut square = RectangleShape::with_size(square_size);
        square.set_fill_color(&sfml::graphics::Color::RED);
        square.set_position( utility::get_boardpos(&board_size, s1) );

        Highlight {
            shape: square 
        }
    }
}

impl<'s> std::cmp::PartialEq for Highlight<'s>
{
    fn eq(&self, rhs: &Highlight) -> bool
    {
        self.shape.position() == rhs.shape.position()
    }
}

impl<'s> sfml::graphics::Drawable for Highlight<'s>
{
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut sfml::graphics::RenderTarget,
        states: sfml::graphics::RenderStates<'texture, 'shader, 'shader_texture>,
        )
    {
        target.draw_rectangle_shape(&self.shape, states);
    }
}
