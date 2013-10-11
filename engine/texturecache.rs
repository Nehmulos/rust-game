use std::hashmap::HashMap;
pub struct TextureCache {
    textures: HashMap<~str, u32>
}

impl TextureCache {
    pub fn new() -> TextureCache {
        TextureCache {
            textures: HashMap::new()
        }
    }

    pub fn load(& mut self, path:~str) -> u32 {
        return -1;
    }
}
