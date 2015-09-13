extern crate piston_window;
extern crate find_folder;
extern crate gfx;
extern crate gfx_device_gl;
extern crate gfx_texture;

use std::rc::Rc;
use std::collections::HashMap;

use gfx_texture::{Texture,TextureSettings,Flip};

pub struct Textures<R> where R: gfx::Resources {
    textures: HashMap<&'static str, Rc<Texture<R>>>,
}

impl Textures<gfx_device_gl::Resources> {
    pub fn new(window: &piston_window::PistonWindow) -> Self {

        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets").unwrap();

        // TODO: Don't hardcode these.
        let block_sprites = vec![
            "element_blue_square.png",
            "element_red_square.png",
            "element_green_square.png",
            "element_yellow_square.png",
            "element_blue_polygon.png",
            "element_red_polygon.png",
            "element_green_polygon.png",
            "element_yellow_polygon.png",
        ];
        let mut store = HashMap::new();

        for filename in block_sprites {
            let texture = Rc::new(Texture::from_path(
                &mut *window.factory.borrow_mut(),
                assets.join(filename),
                Flip::None,
                &TextureSettings::new()
            ).unwrap());

            store.insert(filename, texture);
        }

        Textures {
            textures: store,
        }
    }
}

impl<R: gfx::Resources> Textures<R> {
    pub fn get(&self, key: &str) -> Rc<Texture<R>> {
        self.textures.get(&key).unwrap().clone()
    }
}

