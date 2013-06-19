use sdl;
//use opengles::gl2;

mod Engine;

pub fn main() {
    print("helloWorld");
    do sdl::start {
	sdl::init([sdl::InitVideo]);
	sdl::img::init([sdl::img::InitPNG]);
        sdl::wm::set_caption("rust-sdl demo - video", "rust-sdl");
	
	let imageLoadResult = sdl::img::load(&Path("images/test.png"));
	//let image : ~sdl::video::Surface = imageLoadResult.unwrap();

	
	let mut screen = match sdl::video::set_video_mode(
	    800, 600, 32, [sdl::video::HWSurface], [sdl::video::DoubleBuf, sdl::video::OpenGL])
        {
	    Ok(screen) => screen,
	    Err(err) => fail!(fmt!("failed to set video mode: %s", err))
	};
/*
    gl2::clear_color(1.0f32,0f32,0f32,1.0f32);

    // fucking mac support
    //gl2::clear_depth(1.0f);
        
    gl2::viewport(0, 0, 640, 480);

    gl2::matrix_mode(gl2::GL_PROJECTION);
    gl2::glLoadIdentity();
        
    gl2::glOrtho(0, 640, 480, 0, 1, -1);
       
    gl2::glMatrixMode(GL_MODELVIEW);
   
    gl2::enable(gl2::TEXTURE_2D);
        
    //gl2::glLoadIdentity();
    */
	screen.flip();
        let mut engine = Engine::Engine::new();
        print(fmt!("%d",engine.textureCache.load(~"images/rust-logo.png") as int));

        engine.run();
    }
}
