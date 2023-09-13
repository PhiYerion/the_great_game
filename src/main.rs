use hilbert::*;

use crate::world_object::{WorldObjects, WorldObject, StrawberrieBush, AppleTree};
mod world_object;

#[derive(Debug, Clone)]
struct World {
   // Precontruct the world with array for hilbert curve
   tile_arr: [Tile; 10000]
}

impl World {
   fn new() -> Self {
      let mut tile_arr: [Tile; 10000] = Default::default();
      for tile in tile_arr {
         tile = Tile::new()
      }
      World {
         tile_arr
      }
   }
}

#[derive(Debug, Clone)]
struct Tile {
   pointer_table: Vec<WorldObject>,
   // hbcurve_point: Point,
   
   // Caches -- benchmark these to see if it is actually better.
   object_pointer_cache: [WorldObject; 4],
   object_cache: WorldObject
}

impl Tile {
   fn new() -> Self {
      Tile { 
         pointer_table: vec![], 
         object_pointer_cache: [WorldObject::new(WorldObjects::None); 4],
         object_cache: WorldObject::new(WorldObjects::None)
      }
   }
}

fn main() {
   print!("test"); 
   let wo = WorldObject::new(
      WorldObjects::Plant(
         world_object::Plants::StrawberrieBush(
            StrawberrieBush::new()
         )));
}
