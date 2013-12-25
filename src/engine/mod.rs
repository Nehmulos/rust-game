extern mod rsfml;

use rsfml::window::{Window, VideoMode, ContextSettings, event, keyboard};
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
        let window = match RenderWindow::new(VideoMode::new_init(800, 600, 24), "bst gaem 4eva", sfDefaultStyle, &ContextSettings::default()) {
            Some(v) => ~v,
            None() => fail!("fugg no window DDD:")
        };
        return Engine {
            textureCache: ~TextureCache::new(),
            mode: ~SpaceMode::new(),
            window: window
        };
    }

    pub fn init(& mut self) {
    }

    pub fn uninit(& mut self) {
        self.window.close();
    }

    pub fn run(& mut self) -> int {
        'main: loop {
	    'event: loop {
                match self.window.poll_event() {
                    event::Closed => break 'main,
                    event::KeyPressed{code, ..} => match code {
                        keyboard::Escape => break 'main,
                        _ => {}
                    },
                    _ => {}
                }
            }
            self.mode.update(10);
            self.mode.draw();
	}
        self.uninit();
        return 0;
    }
}
