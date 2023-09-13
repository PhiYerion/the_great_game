
#[derive(Debug, Copy, Clone)]
pub struct GenericTree {
   wood: u8
}

#[derive(Debug, Copy, Clone)]
pub struct AppleTree {
    apples: u8
}

#[derive(Debug, Copy, Clone)]
pub struct StrawberrieBush {
    berries: u8
}

impl StrawberrieBush {
   pub fn new() -> Self {
      StrawberrieBush {
         berries: 0
      }
   }
}

#[derive(Debug, Copy, Clone)]
pub enum Plants {
   GenericTree(GenericTree),
   AppleTree(AppleTree),
   StrawberrieBush(StrawberrieBush),
}

#[derive(Debug, Copy, Clone)]
pub enum WorldObjects {
   Plant(Plants),
   None
}

#[derive(Debug, Copy, Clone)]
pub struct WorldObject {
   pub stale_age: u16,               // Age since last tick on object
   pub age: u16,                     // Absolute age of object
   pub data: WorldObjects,
}

impl WorldObject {
   pub fn new(data: WorldObjects) -> Self {
      WorldObject { 
         stale_age: 0, 
         age: 0,
         data
      }
   }
}
