use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;

/// This struct caches textures so that they don't have to be loaded from disk
/// every time that something requests one.
pub struct TextureManager<'texc, Ctx> {
    loaded_textures: HashMap<String, Rc<Texture<'texc>>>,
    texture_creator: &'texc TextureCreator<Ctx>
}

impl<'texc, Ctx> TextureManager<'texc, Ctx> {
    pub fn new(texture_creator: &'texc TextureCreator<Ctx>) -> Self {
        Self {loaded_textures: HashMap::default(), texture_creator}
    }

    /// Get a texture from
    pub fn get_texture<T>(&mut self, filename: &Path) -> Rc<Texture<'texc>> {
        let filename_str = filename.to_str().unwrap().to_string();
        if let Some(res) = self.loaded_textures.get(&filename_str) {
            return res.clone();
        }
        let res = Rc::new(self.texture_creator.load_texture(filename).unwrap());
        self.loaded_textures.insert(filename_str, res.clone());
        res
    }
}
