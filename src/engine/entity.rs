use self::camera::Camera;

mod camera;

pub trait Drawable {
    fn draw(&Camera);
}

pub trait Updateable {
    fn update(dt:int);
}