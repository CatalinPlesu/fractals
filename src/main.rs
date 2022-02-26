use macroquad::color::colors::*;
use macroquad::color::Color;
use macroquad::input::*;
use macroquad::texture::*;
use macroquad::window;
use macroquad::window::*;

use egui;
use num;
// use rand::Rng;

#[derive(Clone, Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl Point<f64> {
    fn to_world(&self, singl: &Singleton) -> Point<f64> {
        let unit = map_screen_to_world(&singl);
        Point::<f64> {
            x: (self.x - singl.offset.1.x as f64 - screen_width() as f64 / 2f64) * unit
                + singl.center.x,
            y: -(self.y - singl.offset.1.y as f64 - screen_height() as f64 / 2f64) * unit
                + singl.center.y,
        }
    }
}

#[derive(Clone, Debug)]
struct Singleton {
    scale: f64,
    max_iter: usize,
    center: Point<f64>,
    offset: (Point<f32>, Point<f32>),
    refresh: bool,
    mouse_click: bool,
}

impl Default for Singleton {
    fn default() -> Singleton {
        Singleton {
            scale: 1.,
            max_iter: 55,
            center: Point { x: 0., y: 0. },
            offset: (Point { x: 0., y: 0. }, Point { x: 0., y: 0. }),
            refresh: false,
            mouse_click: false,
        }
    }
}

fn mandelbrot(c: num::complex::Complex<f64>, singl: &Singleton) -> usize {
    let mut z = num::complex::Complex::<f64>::new(0.0, 0.0);
    let mut i: usize = 0;
    while i < singl.max_iter && z.l1_norm() <= 4f64 {
        z = z.powf(2f64) + c;
        i += 1;
    }
    return i;
}
fn map_screen_to_world(singl: &Singleton) -> f64 {
    let world_unit: f64;
    if screen_width() < screen_height() {
        world_unit = 4f64 / (screen_width() as f64 * singl.scale);
    } else {
        world_unit = 4f64 / (screen_height() as f64 * singl.scale);
    }
    return world_unit;
}

fn fractal(singl: &Singleton) -> Texture2D {
    let mut fractal = Image::gen_image_color(screen_width() as u16, screen_height() as u16, WHITE);

    let unit = map_screen_to_world(singl);

    for x in 0..screen_width() as u32 {
        for y in 0..screen_height() as u32 {
            let point = Point::<f64> {
                x: x as f64,
                y: y as f64,
            }
            .to_world(&singl);
            let c = num::complex::Complex::<f64>::new(point.x, point.y);

            let iter = mandelbrot(c, singl);

            fractal.set_pixel(
                x,
                y,
                Color::new(
                    (3. * iter as f32) / 255.,
                    (singl.max_iter as f32 - iter as f32) / 255.,
                    (singl.max_iter as f32 - iter as f32) / 255.,
                    1.,
                ),
            );
        }
    }

    return Texture2D::from_image(&fractal);
}

fn draw_menus(singl: &mut Singleton) {
    egui_macroquad::ui(|egui_ctx| {
        egui::Window::new("Settings").show(egui_ctx, |ui| {
            ui.add(egui::Slider::new(&mut singl.max_iter, 0..=500).text("Max iterations"));
            if ui.button("Refresh").clicked() {
                singl.refresh = true;
                singl.mouse_click = false;
                singl.offset = (Point { x: 0., y: 0. }, Point { x: 0., y: 0. });
            }
        });

        egui::Window::new("Debugg info").show(egui_ctx, |ui| {
            ui.label(format!("Scale: {}", singl.scale));
            ui.label(format!("Iterations: {}", singl.max_iter));
            ui.label(format!("Center: ({}, {})", singl.center.x, singl.center.y));
            ui.label(format!(
                "Offset: ({}, {}), ({}, {})",
                singl.offset.0.x, singl.offset.0.y, singl.offset.1.x, singl.offset.1.y
            ));
            ui.label(format!("Refresh: {}", singl.refresh));
            ui.label(format!("Mouse click: {}", singl.mouse_click));
            ui.label(format!("Mouse position: {:?}", mouse_position()));
            ui.label(format!(
                "World position: {:?}",
                Point::<f64> {
                    x: mouse_position().0 as f64,
                    y: mouse_position().1 as f64
                }
                .to_world(&singl)
            ));

            if ui.button("Reset").clicked() {
                *singl = Singleton {
                    ..Default::default()
                };
            }
            if ui.button("Center").clicked() {
                singl.offset = (Point { x: 0., y: 0. }, Point { x: 0., y: 0. });
                singl.mouse_click = false;
            }
        });
    });
    egui_macroquad::draw();
}

fn user_input(singl: &mut Singleton) {
    if is_mouse_button_pressed(MouseButton::Left) && !singl.mouse_click {
        let mouse = mouse_position();
        // singl.offset.0.x = mouse.0;
        // singl.offset.0.y = mouse.1;
        singl.mouse_click = true;
        let world_center = Point::<f64> {
            x: mouse.0 as f64,
            y: mouse.1 as f64,
        }
        .to_world(&singl);
        singl.center = world_center;
    }
    if is_mouse_button_released(MouseButton::Left) && singl.mouse_click {
        let mouse = mouse_position();
        // singl.offset.1.x -= singl.offset.0.x - mouse.0;
        // singl.offset.1.y -= singl.offset.0.y - mouse.1;
        singl.mouse_click = false;
    }

    if is_key_down(KeyCode::Enter) {
        singl.refresh = true;
    }

    singl.scale += singl.scale * (mouse_wheel().1 / 10.) as f64;
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut singl = Singleton {
        ..Default::default()
    };

    let mut texture = fractal(&singl);

    loop {
        clear_background(LIGHTGRAY);

        if singl.refresh {
            texture = fractal(&singl);
            singl.refresh = false;
        }

        draw_texture(texture, singl.offset.1.x, singl.offset.1.y, WHITE);

        user_input(&mut singl);
        draw_menus(&mut singl);

        window::next_frame().await
    }
}

fn window_conf() -> window::Conf {
    window::Conf {
        window_title: "GC Lab 2".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}
