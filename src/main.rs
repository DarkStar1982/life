extern crate piston_window;

mod automata;

use std::time::{SystemTime,UNIX_EPOCH};
use std::path::Path;
use piston_window::*;
use automata::{IWorld, LifeWorld, AntWorld};

//global constants
const BLACK: [f32;4] = [0.0, 0.0, 0.0, 1.0];
const WHITE: [f32;4] = [1.0; 4];
const SQUARE_SIZE: f64 = 5.0;
const WINDOW_SIZE: u32 = 1024;
const GFX_CONTEXT_OFFSET: f64 = (WINDOW_SIZE / 2) as f64;
const MILLIS_PER_FRAME: u128 = 10;

//config struct
struct Config {
    speed:u128,
    cell_size:f64,
    cursor_x:f64,
    cursor_y:f64,
    paused: bool,
    color_invert:bool
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new("Life", [WINDOW_SIZE; 2])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();

    //world
    let mut xworld:Box<dyn IWorld>;

    //screen configs
    let mut win_cfg = Config {
        speed: MILLIS_PER_FRAME,
        cell_size: SQUARE_SIZE,
        cursor_x: 0.0,
        cursor_y: 0.0,
        paused: false,
        color_invert: false
    };

    //screen colors
    let mut foreground: [f32;4] = WHITE;
    let mut background: [f32;4] = BLACK;

    //internal counters
    let mut previous_update = UNIX_EPOCH;
    let mut gen_counter:i64 = 0;

    match args.len()
    {
        1=>{
            xworld = Box::new(AntWorld::new());
        },
        3=>{
            let cmd = &args[1];
            let arg = &args[2];
            if cmd=="-n"
            {
                let configuration_path = String::from(arg);
                xworld = Box::new(LifeWorld::from_configuration(&std::fs::read_to_string(Path::new(&configuration_path)).unwrap(), '.', '*').unwrap());
            }
            else
            {
                xworld = Box::new(LifeWorld::new())
            }
        }
        _=>{
            xworld = Box::new(AntWorld::new());
        }
    }


    while let Some(e) = window.next() {
        if (win_cfg.speed == 0) | (previous_update.elapsed().map(|d| d.as_millis()).unwrap_or(0) > win_cfg.speed) {

                if !win_cfg.paused
                {
                    xworld.step();
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
                                 win_cfg.cursor_y = win_cfg.cursor_y - 20.0;
                             }
                             Key::Up=>{
                                 win_cfg.cursor_y = win_cfg.cursor_y + 20.0;
                             }
                             Key::Left=>{
                                 win_cfg.cursor_x = win_cfg.cursor_x + 20.0;
                             }
                             Key::Right=>{
                                 win_cfg.cursor_x = win_cfg.cursor_x - 20.0;
                             }
                             Key::Z => {
                                 win_cfg.cell_size = win_cfg.cell_size/2.0;
                             }
                             Key::X => {
                                 win_cfg.cell_size = win_cfg.cell_size*2.0;
                             }
                             Key::C => {
                                 if win_cfg.color_invert
                                 {
                                     win_cfg.color_invert = false;
                                     foreground = WHITE;
                                     background = BLACK;
                                 }
                                 else {
                                     win_cfg.color_invert = true;
                                     foreground = BLACK;
                                     background = WHITE;
                                 }
                             }
                             Key::F => {
                                 if win_cfg.speed>2
                                 {
                                    win_cfg.speed = win_cfg.speed/2;
                                 }
                                 else
                                 {
                                     win_cfg.speed = 0;
                                 }
                             }
                             Key::S => {
                                 if win_cfg.speed == 0
                                 {
                                     win_cfg.speed = 1;
                                 }
                                 else
                                 {
                                     win_cfg.speed = win_cfg.speed*2;
                                 }
                             }
                             Key::I => {
                                 println!("Generation: {:}", gen_counter);
                                 println!("Speed is {:} ms per frame", win_cfg.speed);
                             }
                             Key::P => {
                                 win_cfg.paused = true;
                                 println!("Game paused");
                             }
                             Key::R => {
                                 win_cfg.paused = false;
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
                //iterate through cells
                for loc in xworld.current_buffer().keys() {
                    if xworld.get(loc) {
                        rectangle(foreground, [loc.col as f64 * win_cfg.cell_size, loc.row as f64 * win_cfg.cell_size, win_cfg.cell_size, win_cfg.cell_size], context.transform, graphics);
                    } else {

                    }
                }
            });
        }
}
