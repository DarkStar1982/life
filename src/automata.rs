use std::collections::hash_map::{HashMap};

/* RULES SECTION - SHOULD BE MADE RECONFIGURABLE*/
static NEWBORNS: [bool; 9] = [false, false, false, true, false, false, false, false, false];
static SURVIVES: [bool; 9] = [false, false, true,  true, false, false, false, false, false];

/* DECLARATIONS SECTION */

//location data structure
#[derive(PartialEq,Eq,Hash,Clone,Copy)]
pub struct Loc {
  pub row: i64,
  pub col: i64,
}

//Life-like automata
pub struct LifeWorld {
  buffer_1: HashMap<Loc,bool>,
  buffer_2: HashMap<Loc,bool>,
  using_buffer_1: bool,
}

//Langton's Ant automata
pub struct AntWorld {
  buffer_1: HashMap<Loc,bool>,
  machine: Loc,
  angle: i32,
}

//common trait to both types of automata
pub trait IWorld {
    fn new() -> Self;
    //fn from_blank_state() ->Result<Self,String>;
    fn get(&self, loc: &Loc) -> bool;
    fn set(&mut self, loc: &Loc, alive: bool);
    fn current_buffer(&self) -> &HashMap<Loc,bool>;
    fn step(&mut self);

}

/* IMPLEMENTATION SECTION */

impl Loc {
  pub fn new(row: i64, col: i64) -> Self {
    return Self {
      row,
      col,
    }
  }

  pub fn neighbors(&self) -> [Loc;8] {
    return [
      Loc::new(self.row + 1, self.col + 1),
      Loc::new(self.row + 1, self.col - 1),
      Loc::new(self.row - 1, self.col + 1),
      Loc::new(self.row - 1, self.col - 1),
      Loc::new(self.row + 1, self.col    ),
      Loc::new(self.row    , self.col + 1),
      Loc::new(self.row - 1, self.col    ),
      Loc::new(self.row    , self.col - 1),
    ]
  }
}

impl IWorld for LifeWorld {
    fn new() -> LifeWorld {
      Self {
        buffer_1: HashMap::new(),
        buffer_2: HashMap::new(),
        using_buffer_1: true,
      }
    }

    fn get(&self, loc: &Loc) -> bool {
      return is_alive(self.current_buffer(), loc)
    }

    /**
     * Set aliveness status of a location in the world.
     */
    fn set(&mut self, loc: &Loc, alive: bool) {
      let next_buffer = self.next_buffer();

      // If this location is already in the HashMap, set its value. Otherwise,
      // add it as a new entry to the HashMap.
      match next_buffer.get_mut(loc) {
        Some(val) => *val = alive,
        None => { next_buffer.insert(*loc, alive); }
      };

      if alive {
        // If this location is now alive, we need to add any of its neighbors not
        // already in the HashMap, to it.
        for neighbor in loc.neighbors().iter() {
          if next_buffer.get(neighbor).is_none() {
            next_buffer.insert(*neighbor, false);
          }
        }
      }
    }

    fn current_buffer(&self) -> &HashMap<Loc,bool>
    {
      if self.using_buffer_1 {
        return &self.buffer_1
      } else {
        return &self.buffer_2
      }
    }

    /**
     * One "tick" of the world.
     */
    fn step(&mut self)
    {
      let keys: Vec<Loc> = self.current_buffer().keys().map(|&loc| loc).collect();

      for loc in keys.iter() {
        let alive: bool = self.get(&loc);
        let neighbors: [Loc;8] = loc.neighbors();
        let alive_neighbors: usize = neighbors.iter()
          .map(|neighbor| is_alive(self.current_buffer(), neighbor))
          .filter(|alive| *alive)
          .count();

        // If this cell is dead and doesn't have any alive neighbors, we don't
        // need to check on the next cycle for whether or not it might become
        // alive, so we can omit it altogether from the next HashMap.
        if alive_neighbors > 0 {
          self.set(&loc, self.new_status(alive, alive_neighbors));
        }
      }

      // Toggle buffers
      self.using_buffer_1 = !self.using_buffer_1;

      // Clear the old buffer
      self.next_buffer().clear();
    }
}

/* AntWorld Trait */
impl IWorld for AntWorld {
    fn new() -> AntWorld {
      Self {
        buffer_1: HashMap::new(),
        machine: Loc::new(0,0),
        angle: 90,
      }
    }

    fn get(&self, loc: &Loc) -> bool {
      return is_alive(&self.buffer_1, loc)
    }

    /**
     * Set aliveness status of a location in the world.
     */
    fn set(&mut self, loc: &Loc, alive: bool) {
      let next_buffer = &mut self.buffer_1;

      // If this location is already in the HashMap, set its value. Otherwise,
      // add it as a new entry to the HashMap.
      match next_buffer.get_mut(loc) {
        Some(val) => *val = alive,
        None => { next_buffer.insert(*loc, alive); }
      };
    }

    fn current_buffer(&self) -> &HashMap<Loc,bool>
    {
        return &self.buffer_1
    }

    /**
     * One "tick" of the world.
     */
    fn step(&mut self)
    {
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

impl LifeWorld {
  pub fn from_blank_state()->Result<Self, String>
  {
    let world = Self::new();
    return Ok(world);
  }

  /**
   * Initialize from a configuration string. Assumes string is a grid of
   * periods and asterisks (rows separated by line breaks), where asterisks
   * are "alive" cells and periods are dead cells.
   */
  pub fn from_configuration(data: &str, dead_char: char, alive_char: char) -> Result<Self,String> {
    let mut world = Self::new();

    let mut row = 0;
    let mut col = 0;

    for c in data.chars() {
      if c == dead_char {
        world.set(&Loc { row, col }, false);
        col += 1;
      } else if c == alive_char {
        world.set(&Loc { row, col }, true);
        col += 1;
      } else if c == '\n' {
        row += 1;
        col = 0;
      } else if c == '\r' {
      } else {
        return Err(format!("Invalid char '{}' at {}, {}", c, row, col));
      }
    }

    return Ok(world);
  }

  fn next_buffer(&mut self) -> &mut HashMap<Loc,bool> {
    if self.using_buffer_1 {
      return &mut self.buffer_2
    } else {
      return &mut self.buffer_1
    }
  }

 fn new_status(&self, alive: bool, alive_neighbors: usize) -> bool {
      if alive && SURVIVES[alive_neighbors]
      {
          return true
      }
      else if !alive && NEWBORNS[alive_neighbors] {
          return true
      }
      else
      {
          return false
      }
  }

}

impl AntWorld {
    pub fn from_blank_state()->Result<Self, String>
    {
     let world = Self::new();
     return Ok(world);
    }
}

/**
 * Whether or not the supplied location is alive, based on the supplied buffer.
 */
fn is_alive(buffer: &HashMap<Loc,bool>, loc: &Loc) -> bool {
  *buffer.get(loc).unwrap_or(&false)
}

//Langton's ant moves
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
