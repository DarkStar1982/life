# LifeEntropy

![Sample image](https://github.com/brundonsmith/life/raw/master/sample.png)

This is an flavor of the [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life)
based on Brandon Smith's Rust implementation of an unmodified Game of Life.

The major difference are two additional parameters:
* Entropy (each alive cell has a half/life where it becomes dead, regardless of its neighbors' state
* Energy (some dead cells may become alive at each generation, regardless of their neighbors' state )

It uses [Piston](https://www.piston.rs/) to create a window and draw the world
state to it.

## Running the program

Assuming you have `cargo` installed, just execute `cargo run`

```
cargo run
```

## Controls
Following commands are supported:
* Move around using cursor keys
* Zoom in/Zoom out: X/Z
* Pause/Resume: P/R
* Run slower/faster: S/F
* Dump info (generation count and speed): I
