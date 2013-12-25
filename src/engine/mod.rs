extern mod rsfml;

use rsfml::window::{Window, VideoMode, ContextSettings, event, keyboard};
use rsfml::graphics::{RenderWindow, sfDefaultStyle, Color};

use self::texturecache::TextureCache;
use self::gamemode::{SpaceMode};
use self::entity::{Drawable, Updateable};

mod texturecache;
mod gamemode;
mod entity;

pub struct Engine {
    textureCache:~TextureCache,
    mode:~SpaceMode,
    window:@ mut RenderWindow,
    entities:Entities
}

struct Entities {
    drawables: ~[~Drawable],
    updateables: ~[~Updateable]
}

impl Entities {
    pub fn new() -> Entities {
        return Entities {
            drawables: ~[],
            updateables: ~[]
        }
    }
}

impl Engine {
    pub fn new() -> Engine {
        let window = match RenderWindow::new(VideoMode::new_init(800, 600, 24), "bst gaem 4eva", sfDefaultStyle, &ContextSettings::default()) {
            Some(v) => @ mut v,
            None() => fail!("fugg no window DDD:")
        };
        window.set_vertical_sync_enabled(true);

        return Engine {
            textureCache: ~TextureCache::new(),
            mode: ~SpaceMode::new(),
            window: window,
            entities: Entities::new()
        };
    }

    pub fn init(& mut self) {
    }

    pub fn uninit(& mut self) {
        self.window.close();
    }

    pub fn run(& mut self) -> int {
        let clearColor = Color::new_RGB(0,0,0);
        'main: loop {
	    'event: loop {
                match self.window.poll_event() {
                    event::Closed => break 'main,
                    event::KeyPressed{code, ..} => match code {
                        keyboard::Escape => break 'main,
                        _ => {}
                    },
                    event::NoEvent => break 'event,
                    _ => {}
                }
            }
            self.window.clear(&clearColor);
            self.mode.update(10);
            self.mode.draw();
            self.window.display();
	}
        self.uninit();
        return 0;
    }
}
