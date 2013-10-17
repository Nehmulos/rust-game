extern mod sdl2;
use std::hashmap::HashMap;
use std::path::PosixPath;

pub struct TextureCache {
    textures: HashMap<~str, @sdl2::surface::Surface>
}

impl TextureCache {
    pub fn new() -> TextureCache {
        TextureCache {
            textures: HashMap::new()
        }
    }

    pub fn load(& mut self, path:~str) -> @sdl2::surface::Surface {
        let p = std::path::Path::new(path);
        let s:@sdl2::surface::Surface = sdl2::surface::Surface::from_bmp(p);
        self.textures.insert(path, s);
        return s;
    }
}
