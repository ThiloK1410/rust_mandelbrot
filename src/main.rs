mod complex_number;
mod mandelbrot_handle;


use macroquad::prelude::*;
use macroquad::prelude::KeyCode::{Escape, };
use macroquad::prelude::MouseButton;
use macroquad::prelude::Texture2D;

use crate::mandelbrot_handle::Mandelbrot;

//fps for physics
const FPS: i32 = 1;
const TIME_PER_FRAME: f32 = 1f32 / FPS as f32;

const SCALE: f64 = 200f64;

fn get_conf() -> Conf {
    Conf {
        window_title: "Mandelbrot".to_string(),
        window_width: 1000,
        window_height: 800,
        high_dpi: false,
        fullscreen: false,
        sample_count: 0,
        window_resizable: false,
        icon: None,
        platform: Default::default(),
    }
}
#[macroquad::main(get_conf())]
async fn main() {
    let mut mandelbrot_handle = Mandelbrot::new();
    let mut lag = 0f32;
    let mut texture = Texture2D::from_rgba8(screen_width() as u16, screen_height() as u16, &mandelbrot_handle.get_texture_buffer());
    let mut zoom_mode: u8 = 0;
    let mut zoom_corner_1= Vec2::ZERO;
    let mut zoom_corner_2= Vec2::ZERO;


    loop {
        lag += get_frame_time();

        if is_key_down(Escape) {break}

        match zoom_mode {
            0 => {
                if is_mouse_button_pressed(MouseButton::Left) {
                    zoom_mode = 1;
                    zoom_corner_1 = Vec2::from(mouse_position())
                }
            }
            1 => {
                if is_mouse_button_pressed(MouseButton::Left) {
                    mandelbrot_handle.set_new_bounds(
                        mandelbrot_handle.get_number_from_pos(zoom_corner_1),
                        mandelbrot_handle.get_number_from_pos(zoom_corner_2)
                    );
                    texture = Texture2D::from_rgba8(screen_width() as u16, screen_height() as u16, &mandelbrot_handle.get_texture_buffer());
                    zoom_mode = 0
                }

            }
            _ => {}
        }

        while lag >= TIME_PER_FRAME {
            lag -= TIME_PER_FRAME;

        }
        clear_background(WHITE);


        draw_texture(&texture, 0f32, 0f32, WHITE);
        if zoom_mode==1 {
            zoom_corner_2 = Vec2::from(mouse_position());
            draw_rectangle_lines(
                zoom_corner_1.x,
                zoom_corner_1.y,
                zoom_corner_2.x-zoom_corner_1.x,
                zoom_corner_2.y-zoom_corner_1.y,
                2f32, BLUE
            );
        }
        next_frame().await;
    }


}

