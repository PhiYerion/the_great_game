use hilbert::*;
use array_init;

use crate::world_object::{WorldObjects, WorldObject, StrawberrieBush, AppleTree};
mod world_object;
mod world;
use crate::world::World;

fn main() {
   print!("test"); 
   let wo = WorldObject::new(
      WorldObjects::Plant(
         world_object::Plants::StrawberrieBush(
            StrawberrieBush::new()
         )));
   let mut w = World::default();
   w.get_tile_from_index(1).add_object(wo)
}
