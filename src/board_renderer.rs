extern crate uuid;
extern crate piston_window;
extern crate gfx;
extern crate gfx_device_gl;
extern crate gfx_texture;
extern crate rand;
extern crate sprite;
extern crate ai_behavior;

use values::*;
use textures::*;
use board::*;
use wrapper_types::*;

use self::uuid::Uuid;
use self::piston_window::*;
use self::sprite::*;

use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;

use self::ai_behavior::{
    Action,
    Sequence,
    Wait,
};

pub struct BoardRenderer<I: ImageSize, R: gfx::Resources> {
    textures: Rc<Textures<R>>,
    sprites: HashMap<Block, Uuid>,
    position: PixelPosition,
    scene: Scene<I>,
    dimensions: Dimension,
    cell_dimensions: Dimension,
    break_wait: f64,
}

#[derive(Copy,Clone)]
struct BlockSprite {
    sprite: Uuid,
}

#[derive(Copy,Clone,Debug)]
pub struct RenderState {
    dropping: u32,
    break_wait: f64,
}

macro_rules! delayed_animation {
    ($delay:expr, $body:expr) => {
        Sequence(vec!(Wait($delay), Action($body)))
    }
}

impl RenderState {
    pub fn is_settled(&self) -> bool {
        self.dropping == 0 && self.break_wait <= 0.0
    }

    pub fn new() -> Self {
        RenderState {
            dropping: 0,
            break_wait: 0.0,
        }
    }
}

impl BoardRenderer<Texture<gfx_device_gl::Resources>, gfx_device_gl::Resources> {
    pub fn new(textures: Rc<Textures<gfx_device_gl::Resources>>,
               position: PixelPosition,
               dimensions: Dimension) -> Self {

        // Use an arbitrary block to get the cell dimensions. Assumes they are
        // all the same.
        let texture = textures.get(Block::new(Color::Blue, false).to_texture_name());
        let cell_dimensions = Dimension::from_tuple(texture.get_size());

        BoardRenderer {
            textures: textures,
            sprites: HashMap::new(),
            scene: Scene::new(),
            position: position,
            dimensions: dimensions,
            cell_dimensions: cell_dimensions,
            break_wait: 0.0,
        }
    }

    fn cell_w(&self) -> f64 { self.cell_dimensions.w() as f64 }
    fn cell_h(&self) -> f64 { self.cell_dimensions.h() as f64 }
    fn grid_h(&self) -> f64 { self.dimensions.h() as f64 }
    fn x_gutter(&self) -> f64 { self.cell_w() * 1.5 }
    fn grid_margin(&self) -> PixelPosition {
        PixelPosition::new(self.cell_w() * 1.5, 0.0)
    }

    // Scale grid coordinates to pixel coordinates for a block.
    fn scale(&self, block: &PositionedBlock) -> PixelPosition {
        let h = self.cell_h() * self.grid_h() as f64;

        PixelPosition::new(
            block.x() as f64 * self.cell_w(),
            h - (block.y() as f64 * self.cell_h())
        )
    }

    // Returns a sprite id for a given block. Creates a sprite as necessary.
    fn sprite_for(&mut self, block: &PositionedBlock) -> Uuid {
        let exists = { self.sprites.contains_key(&block.block()) };
        if !exists {
            let sprite = Sprite::from_texture(self.textures.get(block.to_texture_name()));
            let sprite_id = self.scene.add_child(sprite);
            self.sprites.insert(block.block(), sprite_id);
            self.update_block(sprite_id, block)
        }

        *self.sprites.get(&block.block()).unwrap()
    }

    // Update a block sprite's position and texture to match the grid, using
    // the default grid margin.
    fn update_block(&mut self, sprite_id: Uuid, block: &PositionedBlock) {
        let margin = self.grid_margin();
        let pos = self.scale(block).add(margin);
        self.update_block_to_pos(sprite_id, block, pos);
    }

    fn update_block_to_pos(&mut self,
                           sprite_id: Uuid,
                           block: &PositionedBlock,
                           pos: PixelPosition) {

        let sprite = { self.scene.child_mut(sprite_id).unwrap() };

        sprite.set_position(pos.x(), pos.y());
        sprite.set_texture(self.textures.get(block.to_texture_name()));
    }

    pub fn render(&mut self, event: &GameWindow, board: &mut Board) -> Option<RenderState> {
        let mut result = None;

        self.scene.event(event);

        event.update(|args| {
            self.break_wait -= args.dt;
            let mut render_state = RenderState::new();
            let mut seen = HashSet::new();

            if let Some(piece) = board.current_piece() {
                for block in piece.blocks().into_iter() {
                    let sprite_id = self.sprite_for(&block);
                    seen.insert(block.block());

                    self.update_block(sprite_id, &block);
                }
            }

            if let Some(piece) = board.next_piece() {
                for block in piece.blocks().into_iter() {
                    let sprite_id = self.sprite_for(&block);
                    seen.insert(block.block());

                    // Top left
                    let pos = PixelPosition::new(
                        block.x() as f64,
                        (2 - block.y()) as f64 * self.cell_h());

                    self.update_block_to_pos(sprite_id, block, pos);
                }
            }

            for event in board.consume_events() {
                match event {
                    BlockEvent::Drop(from, to) => {
                        let sprite_id = self.sprite_for(&to);
                        self.update_block(sprite_id, &from);

                        // Animated drop to new position
                        // TODO: Scale duration by height, apply gravity
                        let new_pos = self.scale(&to).add(self.grid_margin());

                        self.scene.stop_all(sprite_id);
                        let action = Action(Ease(EaseFunction::QuadraticIn, Box::new(
                            MoveTo(0.2, new_pos.x(), new_pos.y())
                        )));
                        self.scene.run(sprite_id, &action);
                    },
                    BlockEvent::Explode(block, depth) => {
                        use self::rand::*;

                        {
                            let sprite_id = self.sprite_for(&block);

                            // Remove and add to bring to foreground
                            self.scene.remove_child(sprite_id);
                            self.sprites.remove(&block.block());
                        }

                        {
                            let sprite_id = self.sprite_for(&block);

                            self.update_block(sprite_id, &block);

                            let mut rng = thread_rng();

                            let t = rng.gen_range(0.4, 0.7);
                            let s = rng.gen_range(1.3, 1.7);
                            let r = rng.gen_range(-90.0, 90.0);
                            let delay = depth as f64 * 0.05;

                            self.scene.run(sprite_id, &delayed_animation!(delay, FadeOut(t)));
                            self.scene.run(sprite_id, &delayed_animation!(delay, ScaleBy(t, s, s)));
                            self.scene.run(sprite_id, &delayed_animation!(delay, RotateBy(t, r)));

                            self.break_wait = self.break_wait.max(delay);
                        }
                    }
                }
            }

            // Position each non-animated block on the board. Needs to run
            // after event processing, since it relies on current animation
            // state.
            for block in board.grid().blocks() {
                let sprite_id = self.sprite_for(&block);

                seen.insert(block.block());

                if self.scene.running_for_child(sprite_id).unwrap() == 0 {
                    self.update_block(sprite_id, &block);
                } else {
                    render_state.dropping += 1;
                }
            }

            let mut existing = HashSet::new();

            for (k, _) in self.sprites.iter() {
                existing.insert(*k);
            }
            for removed in existing.difference(&seen) {
                let done = {
                    let sprite_id = self.sprites.get(removed).unwrap();
                    self.scene.running_for_child(*sprite_id).unwrap() == 0
                };

                if done {
                    let sprite_id = self.sprites.remove(removed).unwrap();
                    self.scene.remove_child(sprite_id);
                }
            }
            render_state.break_wait = self.break_wait;
            result = Some(render_state)
        });

        event.draw_2d(|c, g| {
            let cam = &c.trans(self.position.x() as f64 - self.cell_dimensions.w() as f64 / 2.0,
                               self.position.y() as f64 - self.cell_dimensions.h() as f64 / 2.0);
                               
            // Board bounding box
            {
                let dimensions = [0.0, 0.0,
                    self.cell_dimensions.w() as f64 * self.dimensions.w() as f64,
                    self.cell_dimensions.h() as f64 * self.dimensions.h() as f64,
                ];
                let cam = &cam.trans(
                    self.x_gutter() + self.cell_dimensions.w() as f64 / -2.0,
                    self.cell_dimensions.h() as f64 / 2.0);
                Rectangle::new([1.0, 1.0, 1.0, 0.2])
                    .draw(dimensions, &cam.draw_state, cam.transform, g);
            }

            self.scene.draw(cam.transform, g);
        });
        result
    }
}
