extern crate piston_window;
use std::time::{SystemTime,UNIX_EPOCH};
use std::path::Path;

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
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new("Life", [WINDOW_SIZE; 2])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();

    //initialize variables
    let mut previous_update = UNIX_EPOCH;
    let mut speed: u128 = MILLIS_PER_FRAME;
    let mut cell_size:f64 = SQUARE_SIZE;
    let mut foreground: [f32;4] = WHITE;
    let mut background: [f32;4] = BLACK;
    let mut cursor_x:f64 = 0.0;
    let mut cursor_y:f64 = 0.0;
    let mut gen_counter:i64 = 0;
    let mut paused:bool = false;
    let mut color_invert:bool = false;
    let mut world:life::World;

    match args.len()
    {
        1=>{
            world = World::from_blank_state().unwrap();
        },
        3=>{
            let cmd = &args[1];
            let arg = &args[2];
            if cmd=="-n"
            {
                let configuration_path = String::from(arg);
                world = World::from_configuration(&std::fs::read_to_string(Path::new(&configuration_path)).unwrap(), '.', '*').unwrap();
            }
            else
            {
                world = World::from_blank_state().unwrap();
            }
        }
        _=>{
            world = World::from_blank_state().unwrap();
        }
    }

    while let Some(e) = window.next() {
        if previous_update.elapsed().map(|d| d.as_millis()).unwrap_or(0) > speed {

                if !paused
                {
                    world.step();
                    gen_counter = gen_counter + 1;
                }
                // println!("Step took: {}ms", step_start.elapsed().map(|d| d.as_micros()).unwrap_or(0) as f32 / 1000.0);
                previous_update = SystemTime::now();
            }
            if let Some(button) = e.release_args() {
                match button {
                    Button::Keyboard(key) => {
                         match key {
                             Key::Down=>{
                                 cursor_y = cursor_y - 20.0;
                             }
                             Key::Up=>{
                                 cursor_y = cursor_y + 20.0;
                             }
                             Key::Left=>{
                                 cursor_x = cursor_x + 20.0;
                             }
                             Key::Right=>{
                                 cursor_x = cursor_x - 20.0;
                             }
                             Key::Z => {
                                 cell_size = cell_size/2.0;
                             }
                             Key::X => {
                                 cell_size = cell_size*2.0;
                             }
                             Key::C => {
                                 if color_invert
                                 {
                                     color_invert = false;
                                     foreground = WHITE;
                                     background = BLACK;
                                 }
                                 else {
                                     color_invert = true;
                                     foreground = BLACK;
                                     background = WHITE;
                                 }
                             }
                             Key::F => {
                                 if speed>2
                                 {
                                    speed = speed/2;
                                 }
                                 else
                                 {
                                     speed = 0;
                                 }
                             }
                             Key::S => {
                                 if speed == 0
                                 {
                                     speed = 1;
                                 }
                                 else
                                 {
                                     speed = speed*2;
                                 }
                             }
                             Key::I => {
                                 println!("Generation: {:}", gen_counter);
                                 println!("Speed is {:} ms per frame", speed);
                             }
                             Key::P => {
                                 paused = true;
                                 println!("Game paused");
                             }
                             Key::R => {
                                 paused = false;
                                 println!("Resumed");
                             }
                             _=>{
                                 //ignore other input
                                 //println!("Released keyboard key '{:?}'", key);
                             }
                         }
                     }
                     _=>{
                         //ignore other input
                     }
                }
            };

            window.draw_2d(&e, |context, graphics, _| {
                clear(background, graphics);

                // Translate by 1/2 the window size, to center 0,0 in the middle of the window
                let context = context.trans(GFX_CONTEXT_OFFSET+cursor_x, GFX_CONTEXT_OFFSET+cursor_y);

                for loc in world.current_buffer().keys() {
                    if world.get(loc) {
                        rectangle(foreground, [loc.col as f64 * cell_size, loc.row as f64 * cell_size, cell_size, cell_size], context.transform, graphics);
                    } else {
                        // NOTE: Uncomment to render cells that are dead but have entries in the hash map
                        // rectangle(RED, [loc.col as f64 * SQUARE_SIZE, loc.row as f64 * SQUARE_SIZE, SQUARE_SIZE, SQUARE_SIZE], context.transform, graphics);
                    }
                }
            });
        }
}
