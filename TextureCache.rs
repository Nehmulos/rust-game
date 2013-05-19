use std::treemap;
use opengles::gl2;
use sdl;
use core::hashmap;
    pub struct TextureCache {
        textures: treemap::TreeMap<&str, int>
    }

    impl TextureCache {
        pub fn new() -> TextureCache {
            TextureCache {
                textures: treemap::TreeMap::new()
            }
        }

        pub fn load(&self, path:&str) {
            if self.textures.contains(path) {
                return self.textures.get(path);
            }
            unsafe {
                let id = gl2::gen_textures(1)[0];
                let image = sdl::img::load(&Path(path));
                gl2::tex_image_2d(gl2::TEXTURE_2D,
                    0,
                    gl2::RGB as gl2::GLint, // TODO check sdl img struct
                    image.width as gl2::GLint,
                    image.height as gl2::GLint,
                    0 as gl2::GLint,
                    gl2::RGB,
                    gl2::UNSIGNED_BYTE,
                    image.data); // TODO use real img data
                // TODO may unload img if that's possible in rust
                self.set(path,id);
            }
        }
    }
