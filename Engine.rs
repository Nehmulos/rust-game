extern mod sdl;

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
		    sdl::event::QuitEvent => return 0,
		    sdl::event::NoEvent => break,
		    _ => {}
		}
	    }
            self.mode.update(10);
            self.mode.draw();
            sdl::video::swap_buffers();
	}
        Engine::uninit();
        return 0;
    }
}
