use crate::WorldObject;
use crate::WorldObjects;
use crate::world_object::Plants;
use crate::world_object::StrawberrieBush;

#[derive(Debug, Clone)]
pub struct World {
   // Precontruct the world with array for hilbert curve
   tile_arr: [Tile; 10000]
}

impl<'world> World {
   pub fn get_tile_from_index (&'world mut self, index: usize) -> &'world mut Tile {
      &mut self.tile_arr[index]
   }
}

impl Default for World {
   fn default() -> Self {
      World {
         tile_arr: array_init::array_init(|_| Tile::default())
      }
   }
}

#[test]
fn tile_integration() {
   let mut world = World::default();
   
   let world_object = WorldObject::new(
      WorldObjects::Plant(
         Plants::StrawberrieBush(
            StrawberrieBush::new()
         )
      )
   );

   world.get_tile_from_index(100).add_object(world_object);

   unimplemented!();
}

#[derive(Debug, Clone)]
pub struct Tile {
   table: Vec<WorldObject>,
   // hbcurve_point: Point,
   
   // Caches -- benchmark these to see if it is actually better.
   cache: WorldObject
}

impl Tile {
   pub fn add_object(&mut self, object: WorldObject) {
      self.table.push(self.cache);
      self.cache = object;
   }
}

impl Default for Tile {
   fn default() -> Self {
       Tile { 
         table: Vec::new(),
         cache: WorldObject {
            stale_age: 0,
            age: 0,
            data: WorldObjects::None
         }
      }
   }
}

