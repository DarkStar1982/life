use std::time::{SystemTime,UNIX_EPOCH};
use std::path::Path;

extern crate piston_window;
use piston_window::*;

mod life;
use life::{World};

const BLACK: [f32;4] = [0.0, 0.0, 0.0, 1.0];
const WHITE: [f32;4] = [1.0; 4];
// const RED: [f32;4] =   [1.0, 0.0, 0.0, 1.0];
const SQUARE_SIZE: f64 = 5.0;
const WINDOW_SIZE: u32 = 1024;
const GFX_CONTEXT_OFFSET: f64 = (WINDOW_SIZE / 2) as f64;
const MILLIS_PER_FRAME: u128 = 10;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: life CONFIGURATION")
    } else {
        let opengl = OpenGL::V3_2;
        let mut window: PistonWindow = WindowSettings::new("Life", [WINDOW_SIZE; 2])
            .exit_on_esc(true)
            .graphics_api(opengl)
            .build()
            .unwrap();

        let configuration_path = String::from("./src/configurations/") + &args[1] + ".txt";
        let mut world = World::from_configuration(&std::fs::read_to_string(Path::new(&configuration_path)).unwrap(), '.', '*').unwrap();

        let mut previous_update = UNIX_EPOCH;

        while let Some(e) = window.next() {
            if previous_update.elapsed().map(|d| d.as_millis()).unwrap_or(0) > MILLIS_PER_FRAME {
                // NOTE: Uncomment for timing info
                // let step_start = SystemTime::now();
                world.step();
                // println!("Step took: {}ms", step_start.elapsed().map(|d| d.as_micros()).unwrap_or(0) as f32 / 1000.0);
                previous_update = SystemTime::now();
            }

            window.draw_2d(&e, |context, graphics, _| {
                clear(BLACK, graphics);

                // Translate by 1/2 the window size, to center 0,0 in the middle of the window
                let context = context.trans(GFX_CONTEXT_OFFSET, GFX_CONTEXT_OFFSET);

                for loc in world.current_buffer().keys() {
                    if world.get(loc) {
                        rectangle(WHITE, [loc.col as f64 * SQUARE_SIZE, loc.row as f64 * SQUARE_SIZE, SQUARE_SIZE, SQUARE_SIZE], context.transform, graphics);
                    } else {
                        // NOTE: Uncomment to render cells that are dead but have entries in the hash map
                        // rectangle(RED, [loc.col as f64 * SQUARE_SIZE, loc.row as f64 * SQUARE_SIZE, SQUARE_SIZE, SQUARE_SIZE], context.transform, graphics);
                    }
                }
            });
        }
    }
}
