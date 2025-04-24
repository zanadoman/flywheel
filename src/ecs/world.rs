use super::{Manager, manager::ManagerEvent, system::System};

pub struct World<'a> {
    manager: Manager<'a>,
    systems: Vec<System>,
}

impl World<'_> {
    pub fn run(&mut self) {
        for i in 0..self.systems.len() {
            self.systems[i].run(&mut self.manager);
            while let Some(event) = self.manager.poll_event() {
                match event {
                    ManagerEvent::ArchetypeChanged((entity, archetype)) => {
                        for system in &mut self.systems {
                            system.evaluate(*entity, archetype);
                        }
                    }
                    ManagerEvent::EntityDestroyed(entity) => {
                        for system in &mut self.systems {
                            system.remove(*entity);
                        }
                    }
                }
            }
        }
    }
}
