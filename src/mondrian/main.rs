use nannou::prelude::*;
use nannou::rand::rngs::StdRng;
use nannou::rand::{Rng, SeedableRng};
use nannou::rand::distributions::Standard;

fn main() {
    nannou::app(model).update(update).run();
}

const STEP_SIZE: usize = 50;
const SIZE: f32 = 1000.0;
const X_SPLIT_THRESHOLD: f64 = 0.5;
const Y_SPLIT_THRESHOLD: f64 = 0.5;
const COLOR_THRESHOLD: f64 = 0.7;

#[derive(Debug, Clone, Copy)]
enum PaletteColor {
    WHITE,
    BLUE,
    RED,
    YELLOW
}

impl PaletteColor {
    fn hex_color(&self) -> u32 {
        match *self {
            PaletteColor::WHITE => 0xFFFFFF,
            PaletteColor::BLUE => 0x0000FF,
            PaletteColor::RED => 0xFF0000,
            PaletteColor::YELLOW => 0xFFD500
        }
    }

    fn random_color(rn: f64) -> PaletteColor {
        match (rn * 10.) as u8 {
            0..=6 => PaletteColor::WHITE,
            7 => PaletteColor::RED,
            8 => PaletteColor::BLUE,
            9 => PaletteColor::YELLOW,
            _ => PaletteColor::WHITE
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Rect {
    pos: (f32, f32),
    w: f32,
    h: f32,
    color: PaletteColor, 
}

struct Model {
    _window: window::Id,
    rng_seed: u64
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(SIZE as u32, SIZE as u32)
        .view(view)
        .key_pressed(key_pressed)
        .mouse_pressed(mouse_pressed)
        .build()
        .unwrap();
    
    Model {
        _window: _window,
        rng_seed: 42,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {

}


fn split_squares(squares: &Vec<Rect>, locus: (Option<f32>, Option<f32>), seed: u64) -> Vec<Rect> {
    let (x_, y_) = locus;

    let mut rng = StdRng::seed_from_u64(seed);

    let mut new_squares: Vec<Rect> = Vec::new();
    for s in squares.iter().rev() {
        if let Some(x) = x_ {
            let temp: f64 = rng.sample(Standard);
            if x > s.pos.0 - s.w / 2. && x < s.pos.0 + s.w / 2. && temp > X_SPLIT_THRESHOLD {
                new_squares.extend(split_on_x(&*s, x).iter().cloned());
            } else {
                new_squares.push(s.clone());
            }
        }

        if let Some(y) = y_ {
            let temp: f64 = rng.sample(Standard);
            if y > s.pos.1 - s.h / 2. && y < s.pos.1 + s.h / 2. && temp > Y_SPLIT_THRESHOLD {
                new_squares.extend(split_on_y(&*s, y).iter().cloned());
            } else {
                new_squares.push(s.clone());
            }
        }

        if x_.is_none() && y_.is_none() {
            new_squares.push(s.clone());
        }
    }
    return new_squares;
}

fn split_on_x(r: &Rect, locus: f32) -> Vec<Rect> {
    let lhs = r.pos.0 - (r.w / 2.);
    let lhw = locus - lhs;
    let rhw = r.w - lhw;

    vec![
        Rect {
            pos: (lhs + (lhw / 2.), r.pos.1),
            w: locus - lhs,
            h: r.h,
            color: PaletteColor::WHITE,
        },
        Rect {
            pos: (locus + (rhw / 2.), r.pos.1),
            w: rhw,
            h: r.h,
            color: PaletteColor::WHITE,
        }
    ]
}

fn split_on_y(r: &Rect, locus: f32) -> Vec<Rect> {
    let top = r.pos.1 + (r.h / 2.);
    let th = top - locus;
    let bh = r.h - th;
    vec![
        Rect {
            pos: (r.pos.0, top - (th / 2.)),
            w: r.w,
            h: top - locus,
            color: PaletteColor::WHITE,
        },
        Rect {
            pos: (r.pos.0, locus - (bh / 2.)),
            w: r.w,
            h: bh,
            color: PaletteColor::WHITE,
        }
    ]
}

fn view(app: &App, model: &Model, frame: Frame) {
    let mut squares = vec![
         Rect {
             pos: (0.0, 0.0),
             w: SIZE,
             h: SIZE,
             color: PaletteColor::WHITE
         }
    ];

    
    for i in ((-1. * SIZE / 2.) as u64..(SIZE / 2.) as u64).step_by(STEP_SIZE) {
        squares = split_squares(&squares, (Some(i as f32), Some(i as f32)), model.rng_seed);
    }

    
    
    
    let draw = app.draw();
    draw.background().color(WHITE);

    let mut rng = StdRng::seed_from_u64(model.rng_seed);

    for s in squares.iter_mut() {
        let color_temp: f64 = rng.sample(Standard);
        if color_temp > COLOR_THRESHOLD {
            s.color = PaletteColor::random_color(rng.sample(Standard));
        }
    }
    
    for s in squares.iter() {        
        draw.rect()
            .x_y(s.pos.0, s.pos.1)
            .w_h(s.w, s.h)
            .color(nannou::color::rgb_u32(s.color.hex_color()))
            .stroke(BLACK)
            .stroke_weight(15.0);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn mouse_pressed(_app: &App, model: &mut Model, _button: MouseButton) {
    model.rng_seed = (random_f32() * 100000.0) as u64;
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    if key == Key::S {
        app.main_window()
            .capture_frame(app.exe_name().unwrap() + "_seed_" + &model.rng_seed.to_string() + ".png");
    } else if key == Key::N {
        model.rng_seed = (random_f32() * 100000.0) as u64;
    }
}
