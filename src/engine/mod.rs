extern mod rsfml;
//use sdl2;

use self::texturecache::TextureCache;
use self::gamemode::{SpaceMode};

mod texturecache;
mod gamemode;

pub struct Engine {
    textureCache:~TextureCache,
    mode:~SpaceMode,
}

impl Engine {
    pub fn new() -> Engine {
        return Engine {
            textureCache: ~TextureCache::new(),
            mode: ~SpaceMode::new(),
        }
    }

    pub fn init(&self) {
    }

    pub fn uninit() {
    }

    pub fn run(&self) -> int {
        'main: loop {
	    'event: loop {
	        break 'event;
            }
            self.mode.update(10);
            self.mode.draw();
	}
        Engine::uninit();
        return 0;
    }
}
