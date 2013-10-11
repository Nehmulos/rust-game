extern mod sdl;

mod Engine;

fn main() {
    print("helloWorld");
    
    sdl::init([sdl::InitVideo]);
    sdl::img::init([sdl::img::InitPNG]);
    sdl::wm::set_caption("rust-sdl demo - video", "rust-sdl");
    
    let imageLoadResult = sdl::img::load(&Path("images/test.png"));
    
    let mut screen = match sdl::video::set_video_mode(
	800, 600, 32, [sdl::video::HWSurface], [sdl::video::DoubleBuf, sdl::video::OpenGL])
        {
	Ok(screen) => screen,
	Err(err) => fail!(fmt!("failed to set video mode: %s", err))
    };

    screen.flip();
    let mut engine = Engine::Engine::new();
    print(fmt!("%d",engine.textureCache.load(~"images/rust-logo.png") as int));

    engine.run();
}
