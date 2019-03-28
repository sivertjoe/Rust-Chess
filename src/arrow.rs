use sfml::system::{ Vector2f, Vector2u};
use sfml::graphics::{Transformable, CustomShape, CustomShapePoints, RectangleShape, Shape};
use square::Square;
use sfml::graphics::Color;

use utility;
use angle;

pub struct Arrow<'a>
{
    rect: RectangleShape<'a>,
    shape: CustomShape<'a>,
}

impl<'a> Arrow<'a>
{
    pub fn new(board_size: Vector2u, from: &Square, to: &Square) -> Option<Self>
    {
        let color = sfml::graphics::Color::rgba(249, 212, 35, 230);
        let option_angle = angle::get_angle(to, from);
        if option_angle.is_none()
        {
            return None;
        }
        let angle = option_angle.unwrap();


        Some(Arrow {
            rect: create_rect(from, to, angle, &board_size, &color),
            shape: create_triangle(to, angle, &board_size, &color)
        })
    }
}
fn create_triangle<'s>(to: &Square, angle: f32, board_size: &Vector2u, color: &Color) -> CustomShape<'s>
{
    let ss = board_size.x / 8;
    let mut point1 = utility::get_boardpos(board_size, to);
    point1.x += ss as f32 / 2.0;
    point1.y += ss as f32 / 2.0;
    
    
    let tsize = ss as f32 / 6.0;
    
    let p1 =  Vector2f::new(0.0, 0.0);
    let p2 =  Vector2f::new(-tsize, tsize);
    let p3 =  Vector2f::new(tsize, tsize);

    let mut shape = CustomShape::new(Box::new(TriangleShape::new(p1, p2, p3)));
    shape.set_fill_color(color);
    shape.set_outline_color(color);
    shape.set_outline_thickness(ss as f32 / 12.0);

    shape.set_position(point1);
    shape.rotate(angle - 90.0);
    shape
}

fn create_rect<'s>(from: &Square, to: &Square, angle: f32, board_size: &Vector2u, color: &Color) -> RectangleShape<'s>
{
    let mut point1 = utility::get_boardpos(board_size, to);
    let mut from_point = utility::get_boardpos(board_size, from);
    let diff =  from_point - point1;
    
    let ss = board_size.x / 8;

    // Allign with center
    point1.x += ss as f32 / 2.0;
    point1.y += ss as f32 / 2.0;

    // Allign with center
    from_point.x += ss as f32 / 2.0;
    from_point.y += ss as f32 / 2.0;


    let length = (diff.x.powf(2.0) + diff.y.powf(2.0)).sqrt() - ss as f32 / 8.0;
    let size = Vector2f::new(length, ss as f32 / 6.0);

    let mut rect = RectangleShape::with_size(size);
    rect.set_fill_color(&color);
    rect.set_position(from_point);

    // Set origin around center of square
    rect.set_origin( Vector2f::new(0.0, rect.size().y / 2.0) );

    rect.rotate(angle + 180.0);

    return rect;
}

impl<'s> sfml::graphics::Drawable for Arrow<'s>
{
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut sfml::graphics::RenderTarget,
        states: sfml::graphics::RenderStates<'texture, 'shader, 'shader_texture>,
        )
    {
        target.draw(&self.shape);
        target.draw_rectangle_shape(&self.rect, states);
    }
}

impl<'s> std::cmp::PartialEq for Arrow<'s>
{
    fn eq(&self, rhs: &Arrow) -> bool
    {
        self.shape.position() == rhs.shape.position() 
            && self.rect.position() == rhs.rect.position()
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
