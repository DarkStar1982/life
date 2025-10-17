extern crate piston_window;
extern crate clap;

mod automata;

use std::time::{SystemTime,UNIX_EPOCH};
use std::path::Path;
use piston_window::*;
use automata::{IWorld, LifeWorld, AntWorld};
use clap::{Command, Arg};


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

fn dfa_rules(rule_str: String) -> Option<[[bool;9];2]>
{
    let mut result_birth = [false, false, false, false, false, false, false, false, false];
    let mut result_lives = [false, false, false, false, false, false, false, false, false];
    let mut state:i32 = 0;

    //empty rulestring
    if rule_str==""
    {
        result_birth[3] = true;
        result_lives[2] = true;
        result_lives[3] = true;
        return Some([result_birth, result_lives])
    }

    //...otherwise parse rulestring
    for char in rule_str.chars()
    {
        match char
        {
            'b'|'B'=> {
                match state
                {
                    0 => { state = 1 }
                    2 => { state = 1 }
                    _ => { return None }
                }
            }
            's'|'S'=> {
                match state
                {
                    0 => { state = 2 }
                    1 => { state = 2 }
                    _ => { return None}
                }
            }
            '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9'=>{
                let index:u32 = char.to_digit(10).unwrap();
                match state
                {
                    1=> { result_birth[index as usize]= true }
                    2=> { result_lives[index as usize]= true }
                    _=> { return None }
                }
            }
            _ => { return None }
        }
    }
    return Some([result_birth, result_lives])
}

fn main() 
{

    //command-line arguments
    let matches = Command::new("Cellular Automata Engine")
                          .version("0.1")
                          .author("Dennis")
                          .about("Runs various cellular automata")
                          .arg(Arg::new("input")
                               .short('i')
                               .long("input")
                               .value_name("FILE")
                               .help("Runs from a Life 1.05 input file"))
                          .arg(Arg::new("mode")
                               .short('m')
                               .long("mode")
                               .value_name("MODE")
                               .help("selects input file"))
                          .arg(Arg::new("rule")
                               .short('r')
                               .long("rule")
                               .value_name("RULE")
                               .help("selects automata rule"))
                          .arg(Arg::new("rle_file")
                                .short('e')
                                .long("rle")
                                .value_name("RLE_FILE")
                                .help("Runs from a Life RLE input file"))
                          .get_matches(); 

    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new("EntropyLife", [WINDOW_SIZE; 2])
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

    //input type
    let mut from_rle:bool = false;
    let mut ww = 10;

    //read command line parameters
    let binding = "".to_string();
    let binding2 = "".to_string();
    let binding3 = "".to_string();
    let mut filepath = matches.get_one::<String>("input").unwrap_or(&binding);
    let mode = matches.get_one::<String>("mode").unwrap_or(&binding2);
    let rules = matches.get_one::<String>("rule").unwrap_or(&binding3);
    let processed_rules = dfa_rules(rules.to_string()).unwrap();
    if filepath == ""
    {
        filepath = matches.get_one::<String>("rle_file").unwrap_or(&binding);
        from_rle = true;
    };


    match mode.as_str() {
        "l" | "life" => {
            if filepath!=""
                {
                    if from_rle
                        { xworld = Box::new(LifeWorld::from_rle_file(&std::fs::read_to_string(Path::new(&filepath)).unwrap()).unwrap()) }
                    else
                        { xworld = Box::new(LifeWorld::from_configuration(&std::fs::read_to_string(Path::new(&filepath)).unwrap(), '.', '*',  processed_rules).unwrap()) }
                }
            else
                { xworld = Box::new(LifeWorld::new()) }
        }
        "a" | "ant" => { xworld = Box::new(AntWorld::new()) }
        _ => { return }
    }

    while let Some(e) = window.next() {
        if (win_cfg.speed == 0) | (previous_update.elapsed().map(|d| d.as_millis()).unwrap_or(0) > win_cfg.speed) {
                if !win_cfg.paused
                {
                    if win_cfg.speed == 0
                    {
                        xworld.step();
                        gen_counter = gen_counter + 1;
                    }
                    else {
                        xworld.step();
                        gen_counter = gen_counter + 1;
                    }
                }
                previous_update = SystemTime::now();
            }
            if let Some(button) = e.release_args() {
                match button {
                    Button::Keyboard(key) => {
                         match key {
                             //scroll
                             Key::Down  =>  { win_cfg.cursor_y = win_cfg.cursor_y - 20.0 }
                             Key::Up    =>  { win_cfg.cursor_y = win_cfg.cursor_y + 20.0 }
                             Key::Left  =>  { win_cfg.cursor_x = win_cfg.cursor_x + 20.0 }
                             Key::Right =>  { win_cfg.cursor_x = win_cfg.cursor_x - 20.0 }

                             //zoom
                             Key::Z =>  { win_cfg.cell_size = win_cfg.cell_size/2.0 }
                             Key::X =>  { win_cfg.cell_size = win_cfg.cell_size*2.0 }

                             //color switch
                             Key::C =>  {
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

                             //simulation speed controls
                             Key::F =>  {
                                 if win_cfg.speed>2
                                    { win_cfg.speed = win_cfg.speed/2 }
                                 else
                                    { win_cfg.speed = 0 }
                             }
                             Key::S =>  {
                                 if win_cfg.speed == 0
                                    { win_cfg.speed = 1 }
                                 else
                                    { win_cfg.speed = win_cfg.speed*2 }
                             }
                             // Warp fast forward
                             Key::W => {
                                for i in 0..ww
                                {
                                    xworld.step();
                                }
                                gen_counter = gen_counter + ww;
                             }
                             // Set Warp 10x faster
                             Key::V => {
                                if ww<10000 {ww = ww*10;}
                             }
                            // Set Warp 10x slower
                             Key::U => {
                                if ww>10 {ww = ww/10;}
                             }
                             // pause-resume actions
                             Key::P =>  {
                                 win_cfg.paused = true;
                                 println!("Game paused");
                             }
                             Key::R =>  {
                                 win_cfg.paused = false;
                                 println!("Resumed");
                             }

                             // info dump
                             Key::I =>  {
                                 println!("Generation: {:}", gen_counter);
                                 println!("Speed is {:} ms per frame", win_cfg.speed);
                                 println!("Warp speed is {:}", ww);
                                 println!("Cell population is {:}", xworld.pop_count())
                             }

                             _=> { /* ignore other key inputs */ }
                         }
                     }
                     _=> {/* ignore all other inputs */ }
                }
            };

            window.draw_2d(&e, |context, graphics, _| {
                clear(background, graphics);

                // Translate by 1/2 the window size, to center 0,0 in the middle of the window
                let context = context.trans(GFX_CONTEXT_OFFSET+win_cfg.cursor_x, GFX_CONTEXT_OFFSET+win_cfg.cursor_y);
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
