extern mod sdl2;
use sdl2;

use self::texturecache::*;
use self::gamemode::*;

mod texturecache;
mod gamemode;

pub struct Engine {
    textureCache:~TextureCache,
    mode:~SpaceMode,
    renderer:~sdl2::render::Renderer
}

impl Engine {
    pub fn new() -> Engine {
        let w = match sdl2::video::Window::new("rust game", sdl2::video::PosCentered, sdl2::video::PosCentered, 800, 480, [sdl2::video::OpenGL]) {
            Ok(window) => window,
            Err(err) => fail!(fmt!("could not craete window %s", err))
        };
        let r = match sdl2::render::Renderer::from_window(w,  sdl2::render::DriverAuto, [sdl2::render::Accelerated]) {
            Ok(renderer) => renderer,
            Err(err) => fail!(fmt!("could not create renderer %s", err))
        };
        return Engine {
            textureCache: ~TextureCache::new(),
            mode: ~SpaceMode::new(),
            renderer: r
        }
    }

    pub fn init(&self) {
        sdl2::init([sdl2::InitVideo]);
        //sdl2::img::init([sdl2::img::InitPNG]);
        //let imageLoadResult = sdl2::img::load(&Path("images/test.png"));
        self.renderer.set_draw_color(sdl2::pixels::RGB(0,0,0));
    }

    pub fn uninit() {
    	sdl2::quit();
    }

    pub fn run(&self) -> int {
        'mainloop: loop {
	    loop {
		match sdl2::event::poll_event() {
		    sdl2::event::QuitEvent(_) => break 'mainloop,
		    sdl2::event::NoEvent => break,
		    _ => {}
		}
	    }
            self.mode.update(10);
            self.mode.draw();
            self.renderer.clear();
            self.renderer.present();
	}
        Engine::uninit();
        return 0;
    }
}
