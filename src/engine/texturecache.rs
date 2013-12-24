extern mod rsfml;
use std::hashmap::HashMap;
pub use rsfml::graphics::texture::Texture;

pub struct TextureCache {
    textures: HashMap<~str, @Texture>
}

impl TextureCache {
    pub fn new() -> TextureCache {
        TextureCache {
            textures: HashMap::new()
        }
    }

    pub fn load(& mut self, path:~str) -> @Texture {
        let s:@Texture = match(Texture::new_from_file(path)) {
            Some(t) => @t,
            None => fail!("fugg no texture :DDD") // TODO use failsafe texture
        };
        self.textures.insert(path, s);
        return s;
    }
}
