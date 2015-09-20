extern crate lodepng;

use lodepng::*;

use std::path::PathBuf;

const UP:    u8 = 0b00000001;
const DOWN:  u8 = 0b00000010;
const RIGHT: u8 = 0b00000100;
const LEFT:  u8 = 0b00001000;

/// Given a tile with borders on all sides, automatically create a 9-image
/// tileset that can be used to construct larger tiles.
fn main() {
    let border = 8;

    let colors = vec!("green", "red", "yellow", "blue", "purple");

    for color in colors {
        let source = format!("assets/{}.png", color);
        image_fill(&source, &format!("assets/{}_tl.png", color), border, DOWN + RIGHT);
        image_fill(&source, &format!("assets/{}_l.png", color), border, DOWN + RIGHT + UP);
        image_fill(&source, &format!("assets/{}_bl.png", color), border, UP + RIGHT);
        image_fill(&source, &format!("assets/{}_b.png", color), border, UP + RIGHT + LEFT);
        image_fill(&source, &format!("assets/{}_br.png", color), border, UP + LEFT);
        image_fill(&source, &format!("assets/{}_r.png", color), border, UP + DOWN + LEFT);
        image_fill(&source, &format!("assets/{}_tr.png", color), border, DOWN + LEFT);
        image_fill(&source, &format!("assets/{}_t.png", color), border, DOWN + RIGHT + LEFT);
        image_fill(&source, &format!("assets/{}_m.png", color), border, UP + RIGHT + DOWN + LEFT);
    }
}

fn image_fill(path: &String, dest: &String, border: usize, directions: u8) {
    let image = lodepng::decode32_file(path).unwrap();

    let mut pixels = image.buffer.as_cslice();

    if directions & DOWN > 0 {
        let template_y = image.height - border - 1;
        for y in (template_y+1)..image.height {
            for x in 0..image.width {
                let template_index = template_y * image.width + x;
                let index = y * image.width + x;

                pixels[index] = pixels[template_index];
            }
        }
    }

    if directions & UP > 0 {
        let template_y = border;
        for y in 0..border {
            for x in 0..image.width {
                let template_index = template_y * image.width + x;
                let index = y * image.width + x;

                pixels[index] = pixels[template_index];
            }
        }
    }

    if directions & RIGHT > 0 {
        let template_x = image.width - border - 1;
        for y in 0..image.height {
            let template_y = y;
            for x in template_x..image.width {
                let template_index = template_y * image.width + template_x;
                let index = y * image.width + x;

                pixels[index] = pixels[template_index];
            }
        }
    }

    if directions & LEFT > 0 {
        let template_x = border;
        for y in 0..image.height {
            let template_y = y;
            for x in 0..border {
                let template_index = template_y * image.width + template_x;
                let index = y * image.width + x;

                pixels[index] = pixels[template_index];
            }
        }
    }

    lodepng::encode32_file(dest, image.buffer.as_cslice().as_ref(), image.width, image.height);
}

