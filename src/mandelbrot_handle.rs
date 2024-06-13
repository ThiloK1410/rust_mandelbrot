
use macroquad::math::{Vec2};
use macroquad::miniquad::window::screen_size;
use macroquad::prelude::screen_width;
use macroquad::window::screen_height;
use crate::complex_number::CNumber;
use crate::SCALE;


const CONVERGENCE_STEPS: f64 = 100f64;
const CONVERGENCE_RADIUS: f64 = 4f64;

pub struct Mandelbrot {
    upper_bound: CNumber,
    lower_bound: CNumber,
    delta_range: CNumber,
}


impl Mandelbrot {
    pub fn new() -> Self {
        let mut x = Self {
            upper_bound: CNumber::new(screen_height() as f64/(SCALE*2f64), screen_width() as f64/(SCALE*2f64)),
            lower_bound: CNumber::new(-screen_height() as f64/(SCALE*2f64), -screen_width() as f64/(SCALE*2f64)),
            delta_range: CNumber::zero(),
        };
        x.delta_range = x.upper_bound - x.lower_bound;
        x
    }
    pub fn get_texture_buffer(&self) -> Vec<u8> {
        let mut buffer = vec![0u8; (screen_width()*screen_height()*4f32) as usize]; // rgba
        for x in 0..screen_width() as usize {
            for y in 0..screen_height() as usize {
                // define complex number from position in 2d-grid
                let number = self.get_number_from_pos(Vec2::new(x as f32, y as f32));
                // calculate the factor from convergence radius of complex number
                let factor = 1f64 - Self::get_convergence(number);
                // write the according color to buffer
                buffer[(x+y*screen_width() as usize) * 4] = (255f64 * factor) as u8;
                buffer[(x+y*screen_width() as usize) * 4 + 1] = (255f64 * factor) as u8;
                buffer[(x+y*screen_width() as usize) * 4 + 2] = (255f64 * factor) as u8;
                buffer[(x+y*screen_width() as usize) * 4 + 3] = 255u8;

            }
        }
        buffer
    }

    pub fn get_convergence(x: CNumber) -> f64 {
        let mut out = CONVERGENCE_STEPS;
        let mut number = CNumber::new(0f64, 0f64);
        for i in 0..CONVERGENCE_STEPS as usize {
            number = number*number + x;
            if number.abs() > CONVERGENCE_RADIUS {
                out = i as f64;
                break;
            }
        }
        out/CONVERGENCE_STEPS
    }

    pub fn get_number_from_pos(&self, pos: Vec2) -> CNumber {
        let pixel_scale = self.delta_range / Vec2::from(screen_size());
        return self.lower_bound + pixel_scale * pos;
    }

    pub fn set_new_bounds(&mut self, lower_bound:CNumber, upper_bound:CNumber) {
        self.lower_bound = lower_bound;
        self.upper_bound = upper_bound;
        self.delta_range = self.upper_bound - self.lower_bound;
    }
}