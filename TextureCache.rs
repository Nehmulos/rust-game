use std::treemap;
use opengles::gl2;

use sdl;
    pub struct TextureCache {
        textures: treemap::TreeMap<~str, u32>
    }

    impl TextureCache {
        pub fn new() -> TextureCache {
            TextureCache {
                textures: treemap::TreeMap::new()
            }
        }

        pub fn load(& mut self, path:~str) -> u32 {
            if self.textures.contains_key(&path) {
                return *self.textures.find(&path).get();
            }
            unsafe {
                let id = gl2::gen_textures(1)[0];
                let image = sdl::img::load(&Path(path)).unwrap();
                image.with_lock( |pixels| {
                    let immutablePixels : &[u8] = pixels; // TODO how to inline copy
                    gl2::tex_image_2d(gl2::TEXTURE_2D,
                                      0,
                                      gl2::RGB as gl2::GLint, // TODO check sdl img struct
                                      image.get_width() as gl2::GLint,
                                      image.get_height() as gl2::GLint,
                                      0 as gl2::GLint,
                                      gl2::RGB,
                                      gl2::UNSIGNED_BYTE,
                                      Some(immutablePixels));
                    pixels;
                });
                // TODO may unload img if that's possible in rust
                self.textures.insert(~"test",1);
                self.textures.insert(path,id);
                return id;
            }
        }
    }
