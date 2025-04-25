use super::{Manager, system::System};

pub struct World {
    manager: Manager,
    systems: Vec<System>,
}

impl World {
    pub fn run(&mut self) {
        for i in 0..self.systems.len() {
            self.systems[i].run(&mut self.manager);
            while let Some(entity) = self.manager.poll_dirty() {
                if let Some(archetype) = self.manager.entity_archetype(entity) {
                    for system in &mut self.systems {
                        system.evaluate(entity, archetype);
                    }
                } else {
                    for system in &mut self.systems {
                        system.remove(entity);
                    }
                }
            }
        }
    }
}
