extern crate uuid;
extern crate piston_window;
extern crate gfx;
extern crate gfx_device_gl;
extern crate gfx_texture;
extern crate rand;

extern crate sprite;
extern crate ai_behavior;

use self::uuid::Uuid;
use self::piston_window::*;
use self::sprite::*;

use self::ai_behavior::{
    Action,
};

use std::collections::HashMap;

use values::*;
use textures::Textures;

// TODO: De-dup with main.rs
const CELL_WIDTH: f64 = 32.0;
const CELL_HEIGHT: f64 = 32.0;
const GRID_HEIGHT: u8 = 13;

pub struct Renderer<I: ImageSize, R> where R: gfx::Resources {
    scene: Scene<I>,
    textures: Textures<R>,
    sprites: HashMap<Block, Uuid>,
}

impl<I: ImageSize, R> Renderer<I, R> where R: gfx::Resources {
    pub fn new(textures: Textures<R>) -> Self {
        Renderer {
            textures: textures,
            sprites: HashMap::new(),
            scene: Scene::new(),
        }
    }
}

pub trait BlockRenderer {
    fn event(&mut self, _event: &PistonWindow) {}
    fn add_block(&mut self,  _block: PositionedBlock) {}
    fn move_block(&mut self, _block: PositionedBlock) {}
    fn drop_block(&mut self, _block: PositionedBlock) {}
    fn explode_block(&mut self, _block: PositionedBlock) {}
}

impl BlockRenderer for Renderer<Texture<gfx_device_gl::Resources>, gfx_device_gl::Resources> {
    fn add_block(&mut self, block: PositionedBlock) {
        let texture = self.textures.get(block.block().to_texture_name());
        let sprite = Sprite::from_texture(texture);

        let id = self.scene.add_child(sprite);
        self.scene.run(id,
            &Action(
                MoveTo(0.00,
                    block.x() as f64 * CELL_WIDTH + CELL_WIDTH / 2.0,
                    (GRID_HEIGHT as i8 - block.y() - 1) as f64 * CELL_HEIGHT + CELL_HEIGHT / 2.0
                )
            )
        );
        self.sprites.insert(block.block(), id);
    }

    fn move_block(&mut self, block: PositionedBlock) {
        let sprite = self.sprites.get(&block.block()).unwrap();

        self.scene.stop_all(*sprite);
        self.scene.run(*sprite,
            &Action(
                MoveTo(0.01,
                    block.x() as f64 * CELL_WIDTH + CELL_WIDTH / 2.0,
                    (GRID_HEIGHT as i8 - block.y() - 1) as f64 * CELL_HEIGHT + CELL_HEIGHT / 2.0
                )
            )
        );
    }

    fn drop_block(&mut self, block: PositionedBlock) {
        let sprite = self.sprites.get(&block.block()).unwrap();

        self.scene.stop_all(*sprite);
        self.scene.run(*sprite,
            &Action(
                MoveTo(0.1,
                    block.x() as f64 * CELL_WIDTH + CELL_WIDTH / 2.0,
                    (GRID_HEIGHT as i8 - block.y() - 1) as f64 * CELL_HEIGHT + CELL_HEIGHT / 2.0
                )
            )
        );
    }

    fn explode_block(&mut self, block: PositionedBlock) {
        use self::rand::*;
        // TODO: Remove sprite once done.
        {
            let sprite = self.sprites.get(&block.block()).unwrap();

            // Remove and add to bring to foreground
            self.scene.remove_child(*sprite);
        }
        {
            self.sprites.remove(&block.block());
            self.add_block(block);
        }

        {
            let sprite = self.sprites.get(&block.block()).unwrap();

            let mut rng = thread_rng();

            let t = rng.gen_range(0.4, 0.7);
            let s = rng.gen_range(1.3, 1.7);

            self.scene.run(*sprite,
                &Action(FadeOut(t))
            );
            self.scene.run(*sprite,
                &Action(ScaleBy(t, s, s))
            );
            self.scene.run(*sprite,
                &Action(RotateBy(t, rng.gen_range(-90.0, 90.0)))
            );
            self.scene.remove_child_when_done(*sprite);
        }

        self.sprites.remove(&block.block());
    }

    fn event(&mut self, event: &PistonWindow) {
        self.scene.event(event);
        event.draw_2d(|c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            self.scene.draw(c.transform, g);
        });
    }
}

