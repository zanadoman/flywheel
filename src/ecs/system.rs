use super::{Entity, Manager, archetype::Archetype};

pub trait SystemCallback = Fn(&mut Manager, &[Entity]);

// pub(super) struct SystemBuilder<'a> {
//     manager: &'a mut Manager<'a>,
//     archetype: Archetype,
//     antitype: Archetype,
//     callback: Box<dyn SystemCallback>,
// }
//
// impl SystemBuilder<'_> {
//     fn with<T: 'static>(mut self) -> Self {
//         self.archetype
//             .add(self.manager.component_id_or_register::<T>());
//         self
//     }
//
//     fn without<T: 'static>(mut self) -> Self {
//         self.antitype
//             .add(self.manager.component_id_or_register::<T>());
//         self
//     }
//
//     fn build(self) -> System {
//         System {
//             archetype: self.archetype,
//             antitype: self.antitype,
//             sparse: Vec::new(),
//             owners: Vec::new(),
//             dense: Vec::new(),
//             callback: self.callback,
//         }
//     }
// }

pub(super) struct System {
    archetype: Archetype,
    antitype: Archetype,
    sparse: Vec<Option<usize>>,
    dense: Vec<Entity>,
    callback: Box<dyn SystemCallback>,
}

impl System {
    #[must_use]
    pub fn new<F: SystemCallback + 'static>(
        archetype: Archetype,
        antitype: Archetype,
        callback: F,
    ) -> Self {
        Self {
            archetype,
            antitype,
            sparse: Vec::new(),
            dense: Vec::new(),
            callback: Box::new(callback),
        }
    }

    pub fn evaluate(&mut self, entity: Entity, archetype: &Archetype) {
        if let Some(Some(index)) = self.sparse.get(entity.id()) {
            if !self.archetype.is_subset_of(archetype)
                || self.antitype.has_common_with(archetype)
            {
                self.remove_unchecked(*index);
            }
        } else if self.archetype.is_subset_of(archetype)
            && !self.antitype.has_common_with(archetype)
        {
            if self.sparse.len() <= entity.id() {
                self.sparse.resize(entity.id() + 1, None);
            }
            if self.sparse[entity.id()].is_none() {
                self.sparse[entity.id()] = Some(self.dense.len());
                self.dense.push(entity);
            }
        }
    }

    pub fn remove(&mut self, entity: Entity) {
        if let Some(Some(index)) = self.sparse.get(entity.id()) {
            self.remove_unchecked(*index);
        }
    }

    pub fn run(&self, manager: &mut Manager) {
        (self.callback)(manager, &self.dense);
    }

    fn remove_unchecked(&mut self, index: usize) {
        let last_index = self.dense.len() - 1;
        if index != last_index {
            self.dense.swap(index, last_index);
            let swapped = self.dense[index].id();
            self.sparse[swapped] = Some(index);
        }
        self.dense.pop();
        self.sparse[index] = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ENTITY0: Entity = Entity::new(0);
    const ENTITY1: Entity = Entity::new(1);
    const ENTITY2: Entity = Entity::new(2);
    const ENTITY3: Entity = Entity::new(3);

    #[must_use]
    fn system_archetype() -> Archetype {
        let mut archetype = Archetype::new();
        archetype.add(0);
        archetype.add(1);
        archetype
    }

    #[must_use]
    fn system_antitype() -> Archetype {
        let mut archetype = Archetype::new();
        archetype.add(2);
        archetype
    }

    #[must_use]
    fn empty_archetype() -> Archetype {
        Archetype::new()
    }

    #[must_use]
    fn conflicting_archetype() -> Archetype {
        let mut archetype = Archetype::new();
        archetype.add(0);
        archetype.add(1);
        archetype.add(2);
        archetype
    }

    #[must_use]
    fn matching_archetype() -> Archetype {
        let mut archetype = Archetype::new();
        archetype.add(0);
        archetype.add(1);
        archetype
    }

    #[must_use]
    fn matching_supertype() -> Archetype {
        let mut archetype = Archetype::new();
        archetype.add(0);
        archetype.add(1);
        archetype.add(3);
        archetype
    }

    #[must_use]
    fn setup<F: SystemCallback + 'static>(callback: F) -> System {
        let mut system =
            System::new(system_archetype(), system_antitype(), callback);
        system.evaluate(ENTITY0, &empty_archetype());
        system.evaluate(ENTITY1, &conflicting_archetype());
        system.evaluate(ENTITY2, &matching_archetype());
        system.evaluate(ENTITY3, &matching_supertype());
        system
    }

    #[test]
    fn evaluate_remove_run() {
        setup(|_, entities| {
            assert_eq!(entities.len(), 2);
            assert!(!entities.contains(&ENTITY0));
            assert!(!entities.contains(&ENTITY1));
            assert!(entities.contains(&ENTITY2));
            assert!(entities.contains(&ENTITY3));
        })
        .run(&mut Manager::new());
        let mut system = setup(|_, entities| {
            assert_eq!(entities.len(), 1);
            assert!(!entities.contains(&ENTITY0));
            assert!(!entities.contains(&ENTITY1));
            assert!(!entities.contains(&ENTITY2));
            assert!(entities.contains(&ENTITY3));
        });
        system.evaluate(ENTITY2, &empty_archetype());
        system.run(&mut Manager::new());
        let mut system = setup(|_, entities| {
            assert_eq!(entities.len(), 1);
            assert!(!entities.contains(&ENTITY0));
            assert!(!entities.contains(&ENTITY1));
            assert!(entities.contains(&ENTITY2));
            assert!(!entities.contains(&ENTITY3));
        });
        system.evaluate(ENTITY3, &conflicting_archetype());
        system.run(&mut Manager::new());
        let mut system = setup(|_, entities| assert_eq!(entities.len(), 0));
        system.remove(ENTITY2);
        system.remove(ENTITY3);
        system.run(&mut Manager::new());
    }
}
