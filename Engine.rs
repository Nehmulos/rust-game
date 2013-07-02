use sdl;
use opengles::gl2;

mod TextureCache;
mod GameMode;
pub struct Engine {
    textureCache:~TextureCache::TextureCache,
    mode:~GameMode::SpaceMode
}

impl Engine {
    pub fn new() -> Engine {
        return Engine {
            textureCache: ~TextureCache::TextureCache::new(),
            mode: ~GameMode::SpaceMode::new()
        }
    }

    pub fn init() {
    }

    pub fn uninit() {
    	sdl::quit();
    }

    pub fn run(&self) -> int {
        loop {
	    loop {
		match sdl::event::poll_event() {
		    sdl::event::QuitEvent => return,
		    sdl::event::NoEvent => break,
		    _ => {}
		}
	    }
            gl2::clear(gl2::COLOR_BUFFER_BIT);
            self.mode.update(10);
            self.mode.draw();
            sdl::video::swap_buffers();
	}
        self.uninit();
        return 0;
    }
}
