//use TextureCache;
    struct Engine {
        textureCache:&TextureCache
    }

    impl Engine {
        pub fn new() -> &Engine {
            Engine {
                textureEngine: TextureCache.new()
            }
        } 
    }
