extern crate tempdir;

use std::vec::Vec;
use std::collections::HashMap;
use std::fmt::Display;
use std::process::Command;
use std::path::PathBuf;

use tempdir::TempDir;

/// Generate all the block and breaker assets.
///
/// Source destination should contain a white glyph exclamation.png to be
/// colorized and imprinted on breakers.
///
/// Requires imagemagick `convert` tool to be availabe on the path.
///
/// Usage: generate_tiles src dest
fn main() {
    // http://www.colourlovers.com/palette/1418121/Primal_Dragon
    let mut colors = HashMap::new();
    colors.insert("red",    (186, 30,   35));
    colors.insert("yellow", (224, 185,  26));
    colors.insert("green",  (134, 179,  23));
    colors.insert("blue",   (28,  143, 144));
    colors.insert("grey",   (198, 198, 198));

    let width = 32;
    let height = 32;
    let border_width = 3;

    let tmpdir = TempDir::new("generate-tiles")
        .ok()
        .expect("Could not create temp dir");
    let tmp = tmpdir.path();

    let src = std::env::args().nth(1)
        .expect("Must pass input directory as first argument");
    let src = PathBuf::from(src);

    let out = std::env::args().nth(2)
        .expect("Must pass output directory as second argument");
    let out = PathBuf::from(out);

    let circle_bumpmap = tmp.join("circle_bumpmap.png");
    let square_bumpmap = tmp.join("square_bumpmap.png");

    convert(gen_circle_bumpmap(width, height, border_width), &circle_bumpmap);
    convert(gen_square_bumpmap(width * 3, height * 3, border_width), &square_bumpmap);

    let mut positions = HashMap::new();
    positions.insert("tl", (0, 0));
    positions.insert("t",  (1, 0));
    positions.insert("tr", (2, 0));
    positions.insert("l",  (0, 1));
    positions.insert("m",  (1, 1));
    positions.insert("r",  (2, 1));
    positions.insert("bl", (0, 2));
    positions.insert("b",  (1, 2));
    positions.insert("br", (2, 2));

    for (name, rgb) in &colors {
        let large = tmp.join(format!("{}_large.png", name));
        let darker = (rgb.0 / 2, rgb.1 / 2, rgb.2 / 2);

        // Make a large bevelled block, then slice it to get the 8 required
        // sides, the middle block (with no bevel), and a normal block.
        let mut tile = Vec::new();

        tile.extend(size(d(width * 3, height * 3)));
        tile.push(bg(c3(*rgb)));

        tile.extend(overlay(&square_bumpmap));
        tile.extend(border(c3(darker), &p(width * 3 - 1, height * 3 - 1)));

        convert(tile, &large);

        for (suffix, p) in &positions {
            let (x, y) = *p;
            let cmd = apply(&large, crop(width, height, x*width, y*height));
            convert(cmd, &out.join(format!("{}_{}.png", name, suffix)));
        }

        // Normal block
        let hw = width / 2;
        let hh = height / 2;
        let tl = tmp.join(format!("{}_0.png", name));
        let tr = tmp.join(format!("{}_1.png", name));
        let bl = tmp.join(format!("{}_2.png", name));
        let br = tmp.join(format!("{}_3.png", name));
        let l  = tmp.join(format!("{}_4.png", name));
        let r  = tmp.join(format!("{}_5.png", name));

        convert(apply(&large, crop(hw, hw, 0,  0)), &tl);
        convert(apply(&large, crop(hw, hw, width * 3 - hw,  0)), &tr);
        convert(apply(&large, crop(hw, hw, 0,  height * 3 - hh)), &bl);
        convert(apply(&large, crop(hw, hw, width * 3 - hw,  height * 3 - hh)), &br);

        convert(concat(&tl, &bl, true), &l);
        convert(concat(&tr, &br, true), &r);
        convert(concat(&l, &r, false), &out.join(format!("{}.png", name)));

        // Breakers
        let exclamation = tmp.join(format!("{}_exclamation.png", name));
        let color_circle = tmp.join(format!("{}_circle.png", name));

        let mut cmd = Vec::new();
        cmd.extend(size(d(width, height)));
        cmd.push(bg(c3(darker)));
        cmd.extend(cutout(&src.join("exclamation.png")));
        convert(cmd, &exclamation);

        let center = p((width - 1) as f32 / 2.0, (height - 1) as f32 / 2.0);
        let edge = p(width / 2, height - 1);

        let mut cmd = Vec::new();
        cmd.extend(size(d(width, height)));
        cmd.push(bg("none".to_string()));
        cmd.extend(fill(c3(*rgb)));
        cmd.extend(circle(&center, &edge));
        cmd.extend(over(&exclamation));
        convert(cmd, &color_circle);

        let mut cmd = Vec::new();
        cmd.push(color_circle.to_str().unwrap().to_string());
        cmd.extend(overlay(&circle_bumpmap));
        cmd.extend(cutout(&color_circle));
        cmd.extend(circle_border(&center, &edge, c3(darker)));
        convert(cmd, &out.join(format!("{}_breaker.png", name)));
    }

    let background = out.join("grey.png");

    // Counters
    for (name, rgb) in &colors {
        if *name == "grey" {
            continue;
        }

        for n in 1..4 {
            let number = src.join(format!("{}.png", n));
            let color_number = tmp.join(format!("{}.png", n));

            // Color the number
            let mut cmd = Vec::new();
            cmd.extend(size(d(width, height)));
            cmd.push(bg(c3(*rgb)));
            cmd.extend(cutout(&number));
            convert(cmd, &color_number);

            // Place number on background
            let mut cmd = Vec::new();
            cmd.push(background.to_str().unwrap().to_string());
            cmd.extend(over(&color_number));
            convert(cmd, &out.join(format!("{}_{}.png", name, n)));
        }
    }
}

fn size(arg: String) -> Vec<String> {
    vec!("-size".to_string(), arg)
}

fn bg(c: String) -> String { format!("xc:{}", c) }
fn fill(c: String) -> Vec<String> { vec!("-fill".to_string(), c) }
fn shade(d: String) -> Vec<String> { vec!("-shade".to_string(), d) }
fn antialias(toggle: bool) -> String {
    if toggle {
        "+antialias"
    } else {
        "-antialias"
    }.to_string()
}

fn circle(c: &String, b: &String) -> Vec<String> {
    vec!("-draw".to_string(), format!("circle {} {}", c, b))
}

fn rect(tl: &String, tr: &String) -> Vec<String> {
    vec!("-draw".to_string(), format!("rectangle {} {}", tl, tr))
}

fn overlay(path: &PathBuf) -> Vec<String> {
    vec!(
        "-compose".to_string(),
        "Overlay".to_string(),
        path.to_str().unwrap().to_string(),
        "-composite".to_string(),
    )
}

fn over(path: &PathBuf) -> Vec<String> {
    vec!(
        "-compose".to_string(),
        "Over".to_string(),
        path.to_str().unwrap().to_string(),
        "-composite".to_string(),
    )
}

fn crop<T: Display>(w: T, h: T, x: T, y: T) -> Vec<String> {
    vec!(
        "-crop".to_string(),
        format!("{}x{}+{}+{}", w, h, x, y),
    )
}

fn cutout(path: &PathBuf) -> Vec<String> {
    vec!(
        "-compose".to_string(),
        "Dst_In".to_string(),
        path.to_str().unwrap().to_string(),
        "-alpha".to_string(),
        "Set".to_string(),
        "-composite".to_string()
    )
}

fn apply(path: &PathBuf, op: Vec<String>) -> Vec<String> {
    let mut cmd = vec!(path.to_str().unwrap().to_string());
    cmd.extend(op);
    cmd
}

fn concat(a: &PathBuf, b: &PathBuf, below: bool) -> Vec<String> {
    let append = if below {
        "-append"
    } else {
        "+append"
    };

    vec!(
        a.to_str().unwrap().to_string(),
        b.to_str().unwrap().to_string(),
        append.to_string(),
    )
}

fn border(c: String, br: &String) -> Vec<String> {
    let mut result = vec!(
        "-stroke".to_string(),
        c.to_string(),
        "-strokewidth".to_string(),
        "1".to_string(),
    );
    result.extend(fill("none".to_string()));
    result.extend(rect(&d(0,0), &br));
    result
}

fn circle_border(center: &String, edge: &String, color: String) -> Vec<String> {
    let mut result = vec!(
        "-stroke".to_string(),
        color.to_string(),
        "-strokewidth".to_string(),
        "1".to_string(),
    );
    result.extend(fill("none".to_string()));
    result.extend(circle(&center, &edge));
    result
}

fn d<T: Display>(x: T, y: T) -> String {
    format!("{}x{}", x, y)
}

fn p<T: Display>(x: T, y: T) -> String {
    format!("{},{}", x, y)
}

fn c<T: Display>(r: T, g: T, b: T) -> String {
    format!("rgb({},{},{})", r, g, b)
}

fn c3<T: Display>(tuple: (T, T, T)) -> String {
    format!("rgb({},{},{})", tuple.0, tuple.1, tuple.2)
}

fn convert(args: Vec<String>, out: &PathBuf) {
    let mut cmd = Command::new("convert");
    let cmd = cmd
        .args(&args)
        .arg(out);

    println!("{:?}", cmd);

    cmd.status().unwrap_or_else(|e| {
        panic!("failed to execute process: {}", e)
    });
}

fn gen_circle_bumpmap(width: u32, height: u32, border: u32) -> Vec<String> {
    let mut cmd = Vec::new();
    cmd.extend(size(d(width, height)));
    cmd.push(bg(c(0, 0, 0)));
    cmd.extend(fill(c(0, 0, 0)));
    cmd.push(antialias(false));

    let center = p((width - 1) as f64 / 2.0, (height - 1) as f64 / 2.0);
    for i in 0..border+1 {
        let grey = 255 / border * i;

        cmd.extend(fill(c(grey, grey, grey)));
        cmd.extend(
            circle(&center, &p(width / 2, 31 - i))
        );
    }
    cmd.extend(shade(d(120, 30)));
    cmd
}

fn gen_square_bumpmap(width: u32, height: u32, border: u32) -> Vec<String> {
    let mut cmd = Vec::new();
    cmd.extend(size(d(width, height)));
    cmd.push(bg(c(0, 0, 0)));
    cmd.extend(fill(c(0, 0, 0)));
    cmd.push(antialias(false));

    for i in 0..border+1 {
        let tl = p(i, i);
        let br = p(width - 1 - i, height - 1 - i);

        let grey = 255 / border * i;

        cmd.extend(fill(c(grey, grey, grey)));
        cmd.extend(rect(&tl, &br));
    }
    cmd.extend(shade(d(120, 30)));
    cmd
}

