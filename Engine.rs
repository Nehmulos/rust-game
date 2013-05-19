//use TextureCache::TextureCache;
    struct Engine {
        textureCache:&TextureCache::TextureCache
    }

    impl Engine {
        pub fn new() -> &Engine {
            Engine {
                textureEngine: TextureCache.new()
            }
        } 
    }
