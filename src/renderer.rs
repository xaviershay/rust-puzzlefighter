extern crate uuid;
extern crate piston_window;
extern crate gfx;
extern crate gfx_device_gl;
extern crate gfx_texture;
extern crate rand;

extern crate sprite;
extern crate ai_behavior;

use std::rc::Rc;

use self::uuid::Uuid;
use self::piston_window::*;
use self::sprite::*;

use self::ai_behavior::{
    Action,
    Sequence,
    Wait,
};

use std::collections::HashMap;

use values::*;
use textures::Textures;

pub struct Renderer<I: ImageSize, R> where R: gfx::Resources {
    scene: Scene<I>,
    textures: Rc<Textures<R>>,
    sprites: HashMap<Block, Uuid>,
    position: PixelPosition,
    dimensions: Dimension,
    cell_dimensions: Dimension,
}

pub struct GlRenderSettings<R: gfx::Resources> {
    textures: Rc<Textures<R>>,
}

macro_rules! delayed_animation {
    ($delay:expr, $body:expr) => {
        Sequence(vec!(Wait($delay), Action($body)))
    }
}

impl<R> GlRenderSettings<R> where R: gfx::Resources {
    pub fn new(textures: Textures<R>) -> Self {
        GlRenderSettings {
            textures: Rc::new(textures),
        }
    }
}


impl<I: ImageSize, R> Renderer<I, R> where R: gfx::Resources {
    pub fn new(textures: Rc<Textures<R>>, position: PixelPosition, dimensions: Dimension, cell_dimensions: Dimension) -> Self {
        Renderer {
            textures: textures,
            sprites: HashMap::new(),
            scene: Scene::new(),
            position: position,
            dimensions: dimensions,
            cell_dimensions: cell_dimensions,
        }
    }

    fn scale_x(&self, x: i8) -> f64 {
        let cell_w = self.cell_dimensions.w() as f64;

        x as f64 * cell_w + cell_w / 2.0
    }

    fn scale_y(&self, y: i8) -> f64 {
        let cell_h = self.cell_dimensions.h() as f64;
        let grid_h = self.dimensions.h();

        (grid_h as i8 - y - 1) as f64 * cell_h + cell_h / 2.0
    }
}

pub trait RenderSettings {
    fn build(&self, _position: PixelPosition, _dimensions: Dimension) -> Box<BlockRenderer>;
}

impl RenderSettings for GlRenderSettings<gfx_device_gl::Resources> {
    fn build(&self, position: PixelPosition, dimensions: Dimension) -> Box<BlockRenderer> {
        let texture = self.textures.get("element_blue_square.png");
        let cell_dimensions = Dimension::from_tuple(texture.get_size());

        Box::new(Renderer::new(self.textures.clone(), position, dimensions, cell_dimensions)) as Box<BlockRenderer>
    }
}

pub trait BlockRenderer {
    fn event(&mut self, _event: &PistonWindow) {}
    fn add_block(&mut self,  _block: PositionedBlock) {}
    fn move_block(&mut self, _block: PositionedBlock) {}
    fn drop_block(&mut self, _block: PositionedBlock) {}
    fn remove_block(&mut self, _block: PositionedBlock) {}
    fn explode_block(&mut self, _block: PositionedBlock, _depth: u8) {}
    fn is_animating(&self, block: PositionedBlock) -> bool;
}

impl BlockRenderer for Renderer<Texture<gfx_device_gl::Resources>, gfx_device_gl::Resources> {
    fn add_block(&mut self, block: PositionedBlock) {
        let texture = self.textures.get(block.block().to_texture_name());
        let mut sprite = Sprite::from_texture(texture);
        sprite.set_position(self.scale_x(block.x()), self.scale_y(block.y()));
        let id = self.scene.add_child(sprite);
        self.sprites.insert(block.block(), id);
    }

    fn move_block(&mut self, block: PositionedBlock) {
        let sprite = self.sprites.get(&block.block()).unwrap();

        self.scene.stop_all(*sprite);
        let action = Action(
            MoveTo(0.01, self.scale_x(block.x()), self.scale_y(block.y()))
        );
        self.scene.run(*sprite, &action);
    }

    fn drop_block(&mut self, block: PositionedBlock) {
        let sprite = self.sprites.get(&block.block()).unwrap();

        // TODO: Scale time by how far the drop is.
        self.scene.stop_all(*sprite);
        let action = Action(Ease(EaseFunction::QuadraticIn, Box::new(
            MoveTo(0.2, self.scale_x(block.x()), self.scale_y(block.y()))
        )));
        self.scene.run(*sprite, &action);
    }

    fn remove_block(&mut self, block: PositionedBlock) {
        {
            let sprite = self.sprites.get(&block.block()).unwrap();

            // Remove and add to bring to foreground
            self.scene.remove_child(*sprite);
        }

        self.sprites.remove(&block.block());
    }

    fn explode_block(&mut self, block: PositionedBlock, depth: u8) {
        use self::rand::*;

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
            let r = rng.gen_range(-90.0, 90.0);
            let delay = depth as f64 * 0.05;

            self.scene.run(*sprite, &delayed_animation!(delay, FadeOut(t)));
            self.scene.run(*sprite, &delayed_animation!(delay, ScaleBy(t, s, s)));
            self.scene.run(*sprite, &delayed_animation!(delay, RotateBy(t, r)));

            self.scene.remove_child_when_done(*sprite);
        }

        self.sprites.remove(&block.block());
    }

    fn is_animating(&self, block: PositionedBlock) -> bool {
        let sprite = self.sprites.get(&block.block()).unwrap();

        self.scene.running_for_child(*sprite) > 0
    }

    fn event(&mut self, event: &PistonWindow) {
        use graphics::*;

        self.scene.event(event);
        event.draw_2d(|c, g| {
            // Center board
            let cam = &c.trans(self.position.x() as f64, self.position.y() as f64);

            // Board bounding box
            let dimensions = [0.0, 0.0,
                self.cell_dimensions.w() as f64 * self.dimensions.w() as f64,
                self.cell_dimensions.h() as f64 * self.dimensions.h() as f64,
            ];
            Rectangle::new([0.2, 0.2, 0.2, 1.0])
                .draw(dimensions, &cam.draw_state, cam.transform, g);

            // Blocks
            self.scene.draw(cam.transform, g);
        });
    }
}

