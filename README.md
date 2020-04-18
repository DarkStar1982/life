# LifeEntropy

![Sample image](https://github.com/brundonsmith/life/raw/master/sample.png)

This is a flavor of the [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life)
based on Brandon Smith's Rust implementation of an unmodified Game of Life.


It uses [Piston](https://www.piston.rs/) to create a window and draw the world
state to it.

## Running the program

Assuming you have `cargo` installed, just execute `cargo run`

To load any of example patterns do as following
```
cargo run -- -n src/configurations/glider.txt
```

To run without `cargo`, using just build output
```
...target/debug/entropylife -n ../../src/configurations/glider.txt
```

## Controls
Following commands are supported:
* Move around using cursor keys
* Zoom in/Zoom out: X/Z
* Pause/Resume: P/R
* Run slower/faster: S/F
* Dump info (generation count and speed): I
* Invert colors: C
