extern crate sprite;
extern crate piston_window;
extern crate uuid;
extern crate graphics;
extern crate find_folder;
extern crate gfx;
extern crate gfx_texture;

use sprite::{Sprite,Scene};
use self::uuid::Uuid;

use std::collections::HashMap;
use std::rc::Rc;

use graphics::{ Graphics, ImageSize };
use gfx_texture::{Texture};

use piston_window::{PistonWindow,WindowSettings,Flip,TextureSettings};


struct Game;

struct TextureFactory<R> where R: gfx::Resources {
    block_textures: Vec<Rc<Texture<R>>>,
}

impl<R> TextureFactory<R> where R: gfx::Resources  {
    fn new(window: PistonWindow) -> TextureFactory<R> {

        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets").unwrap();

        let block_sprites = vec![
            "element_blue_square.png",
            "element_red_square.png",
            "element_green_square.png",
            "element_yellow_square.png",
        ];
        /*
        let block_textures: Vec<Rc<Texture<R>>> = block_sprites.iter().map(|filename| {
            Rc::new(Texture::from_path(
                &mut *window.factory.borrow_mut(),
                assets.join(filename),
                Flip::None,
                &TextureSettings::new()
            ).unwrap() as Texture<R>)
        }).collect();
        */
        let texture: Texture<R> = gfx_texture::Texture::from_path(
            &mut *window.factory.borrow_mut(),
            assets.join("element_red_square.png"),
            Flip::None, &TextureSettings::new()
        ).unwrap();
        let block_textures = Vec::new();
        block_textures.push(texture);

        TextureFactory {
            block_textures: block_textures,
        }
    }
    fn from_color(&self, color: Color) -> Rc<Texture<R>> {
        match color {
            Red => self.block_textures[0],
            _   => self.block_textures[1],
        }
    }
}

struct Sprites<I: ImageSize, R> where R: gfx::Resources {
    sprites: HashMap<Uuid, Sprite<I>>,
    textures: TextureFactory<R>,
}

struct Renderer<I: ImageSize, R> where R: gfx::Resources {
    scene: Scene<I>,
    textures: TextureFactory<R>,
}

struct Block {
    color: Color,
}

impl<I: ImageSize, R> Renderer<I, R> where R: gfx::Resources {
    fn new(textures: TextureFactory<R>, scene: Scene<I>) -> Renderer<I, R> {
        Renderer {
            textures: textures,
            scene: scene,
        }
    }

    fn load(&mut self, sprite: Sprite<I>) {
        self.scene.add_child(sprite);
    }

    fn add_block(&mut self, block: Block) {
        let sprite = Sprite::from_texture(self.textures.for_color(block.color));

        let id = self.scene.add_child(sprite);
        // TODO: Keep track of block -> id mapping
    }
}


enum Color {
    Blue,
    Red,
    Green,
    Yellow
}

const GRID_HEIGHT: usize = 10;
const GRID_WIDTH: usize = 8;
const CELL_WIDTH: f64 = 32.0;
const CELL_HEIGHT: f64 = 32.0;

fn main() {
    let width = (GRID_WIDTH * CELL_WIDTH as usize) as u32;
    let height = (GRID_HEIGHT * CELL_HEIGHT as usize) as u32;
    let window: PistonWindow =
        WindowSettings::new("Puzzle Fighter Turbo II", (width, height))
        .exit_on_esc(true)
        .build()
        .unwrap();

    let textures = TextureFactory::new(window);

    let mut scene = Scene::new();

    let mut renderer = Renderer::new(textures, scene);
    renderer.add_block(Block { color: Color::Red });

}
