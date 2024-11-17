use crate::shared::Point;

pub trait Terminal {
    fn clear(&mut self);
    fn write_at(&mut self, point: Point, text: char);
}

pub trait Drawable {
    fn draw(&self);
}

pub trait Updatable {
    fn update(&mut self);
}
