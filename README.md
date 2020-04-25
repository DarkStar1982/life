# LifeEntropy

![Sample image](https://github.com/brundonsmith/life/raw/master/sample.png)

This is an engine for various cellular automata, such as [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) based on Brandon Smith's original Game of Life implementation in Rust.


It uses [Piston](https://www.piston.rs/) to create a window and draw the world
state to it.

## Running the program

Assuming you have `cargo` installed, just execute `cargo run`

To load any of Life example patterns do as following
```
cargo run -- m l -i src/configurations/glider.txt
```

To run without `cargo`, using just build output
```
...target/debug/entropylife -m l -i ../../src/configurations/glider.txt
```

When run with ""-m a" input arguments it runs Langton's Ant algorithm
When run with ""-m l" input arguments it runs Conway's Game of Life algorithm


## Controls
Following commands are supported:
* Move around using cursor keys
* Zoom in/Zoom out: X/Z
* Pause/Resume: P/R
* Run slower/faster: S/F
* Dump info (generation count and speed): I
* Invert colors: C
