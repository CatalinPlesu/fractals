use macroquad::color::colors::*;
use macroquad::color::Color;
use macroquad::window;
use macroquad::window::*;
use num;
use rand::Rng;

struct Point {
    x: f64,
    y: f64,
}

struct Singleton {
    scale: f64,
    max_iter: usize,
    center: Point,
}

impl Default for Singleton {
    fn default() -> Singleton {
        Singleton {
            scale: 1.,
            max_iter: 55,
            center: Point {x: 0., y: 0.},
        }
    }
}

fn mandlelbrot(c: num::complex::Complex<f64>, singl: &Singleton) -> usize {
    let mut z = num::complex::Complex::<f64>::new(0.0, 0.0);
    let mut i: usize = 0;
    while i < singl.max_iter && z.l1_norm() <= 4f64 {
        z += z.powf(2.) + c;
        i += 1;
    }
    i
}

fn fractal() {

    todo!()
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut singl = Singleton{
        ..Default::default()
    };

    let complex_float = num::complex::Complex::new(0., 0.);


    println!("{}", mandlelbrot(complex_float, &singl));

    loop {
        clear_background(LIGHTGRAY);

        window::next_frame().await
    }
}

fn window_conf() -> window::Conf {
    window::Conf {
        window_title: "GC Lab 2".to_owned(),
        // fullscreen: true,
        ..Default::default()
    }
}
