use hilbert::*;
mod world_object;

#[derive(Debug, Clone)]
struct World {
   // Precontruct the world with array for hilbert curve
   tile_arr: [Tile; 10000]
}

impl World {
   fn gen_world(self: &mut Self) {
   
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
      Tile { pointer_table: vec![], object_pointer_cache: [WorldObject::new(); 4], object_cache: WorldObject::new() }
   }
}

fn main() {
   print!("test"); 
   WorldObject::new(Wor)
}
