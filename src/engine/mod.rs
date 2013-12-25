extern mod rsfml;

use rsfml::window::{Window, VideoMode, ContextSettings};
use rsfml::graphics::{RenderWindow, sfDefaultStyle};

use self::texturecache::TextureCache;
use self::gamemode::{SpaceMode};

mod texturecache;
mod gamemode;

pub struct Engine {
    textureCache:~TextureCache,
    mode:~SpaceMode,
    window:~RenderWindow,
}

impl Engine {
    pub fn new() -> Engine {
        let window = match(RenderWindow::new(VideoMode::new_init(800, 600, 24), "bst gaem 4eva", sfDefaultStyle, &ContextSettings::default())) {
            Some(v) => ~v,
            None() => fail!("fugg no window DDD:")
        };
        return Engine {
            textureCache: ~TextureCache::new(),
            mode: ~SpaceMode::new(),
            window: window
        };
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
