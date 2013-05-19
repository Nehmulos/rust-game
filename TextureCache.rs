use std::treemap;
    pub struct TextureCache {
        textures: TreeMap<&string, int>
    }

    impl TextureCache {
        pub fn new() -> TextureCache {
            TextureCache {
                textures: TreeMap::new()
            }
        }

        pub fn load(&self, path:&string) {
            if self.textures.contains(path) {
                return self.textures.get(path);
            }
            unsafe {
                let id = gl2::gen_textures(1)[0];
                let image = sdl::img::load(&Path(path));
                tex_image_2d(gl2::GL_TEXTURE_2D,
                    0,
                    gl2::GL_RGB as gl2::GLint, // TODO check sdl img struct
                    image.width as gl2::GLint,
                    image.height as gl2::GLint,
                    0 as gl2::GLint,
                    gl2::GL_RGB,
                    gl2::GL_UNSIGNED_BYTE,
                    image.data); // TODO use real img data
                // TODO may unload img if that's possible in rust
                self.set(path,id);
            }
        }
    }
