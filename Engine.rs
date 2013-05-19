mod TextureCache;
    pub struct Engine {
        textureCache:~TextureCache::TextureCache
    }

    impl Engine {
        pub fn new() -> Engine {
            return Engine {
                textureCache: ~TextureCache::TextureCache::new()
            }
        } 
    }
