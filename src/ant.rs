/* Langton's ant algorthim implementation */
use std::collections::hash_map::{HashMap};

#[derive(PartialEq,Eq,Hash,Clone,Copy)]
pub struct Loc {
  pub row: i64,
  pub col: i64,
}

impl Loc {
  pub fn new(row: i64, col: i64) -> Self {
    return Self {
      row,
      col,
    }
  }
}

pub struct World {
  buffer_1: HashMap<Loc,bool>,
  machine: Loc,
  angle: i32,
}

impl World {

  pub fn new() -> World {
    Self {
      buffer_1: HashMap::new(),
      machine: Loc::new(0,0),
      angle: 90,
    }
  }

  /**
   * Initialize from a configuration string. Assumes string is a grid of
   * periods and asterisks (rows separated by line breaks), where asterisks
   * are "alive" cells and periods are dead cells.
   */
  pub fn from_blank_state()->Result<Self,String>
  {
      let world = Self::new();
      return Ok(world);
  }

  pub fn current_buffer(&self) -> &HashMap<Loc,bool>
  {
      return &self.buffer_1
  }

  /**
   * Get aliveness status of a location in the world.
   */
  pub fn get(&self, loc: &Loc) -> bool {
    return is_alive(&self.buffer_1, loc)
  }

  /**
   * Set aliveness status of a location in the world.
   */
  pub fn set(&mut self, loc: &Loc, alive: bool) {
    let next_buffer = &mut self.buffer_1;

    // If this location is already in the HashMap, set its value. Otherwise,
    // add it as a new entry to the HashMap.
    match next_buffer.get_mut(loc) {
      Some(val) => *val = alive,
      None => { next_buffer.insert(*loc, alive); }
    };
  }

  /**
   * One "tick" of the world.
   */
  pub fn step(&mut self) {
    let is_alive: bool = self.get(&self.machine);
    let loc = self.machine;
    if is_alive {
        self.angle = (self.angle - 90) % 360;
        if self.angle<0
        {
            self.angle = self.angle + 360;
        }
        self.set(&loc, false)
    }
    else {
        self.angle = (self.angle + 90) % 360;
        self.set(&loc, true);
    }
    let offset = moves(self.angle);
    self.machine = Loc::new(loc.row+offset.0, loc.col + offset.1);
  }
}

/**
 * Whether or not the supplied location is alive, based on the supplied buffer.
 */
fn is_alive(buffer: &HashMap<Loc,bool>, loc: &Loc) -> bool {
  *buffer.get(loc).unwrap_or(&false)
}

fn moves (angle: i32)->(i64,i64)
{
    match angle
    {
        0   =>return (0, 1),
        90  =>return (1, 0),
        180 =>return (0,-1),
        270 =>return (-1,0),
        _   =>return (0, 0),
    }
}
