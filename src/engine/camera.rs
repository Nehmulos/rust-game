use rsfml::graphics::RenderWindow;

pub struct Camera {
    zoom:int,
    window:@ mut RenderWindow
}