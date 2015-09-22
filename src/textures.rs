extern crate piston_window;
extern crate find_folder;
extern crate gfx;
extern crate gfx_device_gl;
extern crate gfx_texture;

use std::rc::Rc;
use std::collections::HashMap;
use std::fs;

use gfx_texture::{Texture,TextureSettings,Flip};

pub struct Textures<R> where R: gfx::Resources {
    textures: HashMap<String, Rc<Texture<R>>>,
}

impl Textures<gfx_device_gl::Resources> {
    pub fn new(window: &piston_window::PistonWindow) -> Self {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets/gen").ok()
            .expect("No assets/ directory found");

        let paths = fs::read_dir(assets).ok()
            .expect("Could not list contents of assets dir");

        let mut store = HashMap::new();

        // TODO: Do this off-thread or otherwise as part of a loading routine.
        // Currently shows an ugly white screen while it loads.
        for filename in paths {
            let filename = filename.unwrap();
            if let Some(ext) = filename.path().extension() {
                if ext == "png" {
                    let texture = Texture::from_path(
                            &mut *window.factory.borrow_mut(),
                            filename.path(),
                            Flip::None,
                            &TextureSettings::new()
                        )
                        .ok()
                        .expect(&format!("Could not load texture: {}", filename.path().display()));

                    store.insert(filename.path().file_name().unwrap().to_str().unwrap().to_string(), Rc::new(texture));
                }
            }
        }

        Textures {
            textures: store,
        }
    }
}

impl<R: gfx::Resources> Textures<R> {
    pub fn get(&self, key: String) -> Rc<Texture<R>> {
        self.textures.get(&key)
            .expect(&format!("No texture exists for {}", key))
            .clone()
    }
}

