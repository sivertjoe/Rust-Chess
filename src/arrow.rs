use sfml::system::{ Vector2f, Vector2u};
use sfml::graphics::{Transformable, Color, CustomShape, CustomShapePoints, RectangleShape, RenderTarget, RenderWindow, Shape};
use square::Square;

use utility;

pub struct Arrow<'a>
{
    rect: RectangleShape<'a>,
    shape: CustomShape<'a>,
}


impl<'a> Arrow<'a>
{
    pub fn new(board_size: Vector2u, from: &Square, to: &Square) -> Self
    {
        let ss = board_size.x / 8;
        let mut point1 = utility::get_boardpos(&board_size, to);
        let from_point = utility::get_boardpos(&board_size, from);
        let diff =  from_point - point1;

        point1.x += ss as f32 / 2.0;
        point1.y += ss as f32 / 2.0;


        let length = (diff.x.powf(2.0) + diff.y.powf(2.0)).sqrt();
        let size = Vector2f::new(length, ss as f32 / 4.0);

        let mut rect = RectangleShape::with_size(size);
        rect.set_fill_color(&sfml::graphics::Color::BLUE);
        rect.set_position(point1);


        let angle = utility::get_angle(to, from);
        
        rect.set_rotation(angle);


        let p1 =  point1;
        let p2 =  Vector2f::new(point1.x - 20., point1.y + 20.);
        let p3 =  Vector2f::new(point1.x + 20., point1.y + 20.);

        let mut shape = CustomShape::new(Box::new(TriangleShape::new(p1, p2, p3)));
        shape.set_fill_color(&sfml::graphics::Color::BLUE);
        shape.set_outline_color(&sfml::graphics::Color::BLUE);
        shape.set_outline_thickness(3.);

        Arrow
        {
            rect: rect,
            shape: shape
        }
    }

    pub fn draw(&self, window: &mut RenderWindow)
    {
        window.draw(&self.rect);
        window.draw(&self.shape);
    }
}


struct TriangleShape(Vector2f, Vector2f, Vector2f);
impl TriangleShape
{
    fn new(v1: Vector2f, v2: Vector2f, v3: Vector2f) -> Self
    {
        TriangleShape ( v1, v2, v3 )
    }
}

impl CustomShapePoints for TriangleShape
{
    fn point_count(&self) -> u32
    {
        3
    }

    fn point(&self, point: u32) -> Vector2f
    {
        match point
        {
            0 => self.0,
            1 => self.1,
            2 => self.2,
            _ => unreachable!()
        }
    }
}
