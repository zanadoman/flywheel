use super::{Entity, entity_data::EntityData};

pub struct EntityManager {
    sparse: Vec<Option<EntityData>>,
    destroyed: Vec<EntityData>,
}

impl EntityManager {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            sparse: Vec::new(),
            destroyed: Vec::new(),
        }
    }

    #[must_use]
    pub fn spawn(&mut self) -> Entity {
        if let Some(entity_data) = self.destroyed.pop() {
            let entity = entity_data.owner();
            self.sparse[entity.id()] = Some(entity_data);
            entity
        } else {
            let entity = Entity::new(self.sparse.len());
            self.sparse.push(Some(EntityData::new(entity)));
            entity
        }
    }

    #[must_use]
    pub fn get(&self, owner: Entity) -> Option<&EntityData> {
        self.sparse.get(owner.id())?.as_ref()
    }

    pub fn bind(&mut self, parent: Entity, child: Entity) {
        if parent == child
            || !self.sparse.get(parent.id()).is_some_and(Option::is_some)
            || !self.sparse.get(child.id()).is_some_and(Option::is_some)
        {
            return;
        }
        if let Some(parent) = self.sparse[child.id()].as_ref().unwrap().parent()
        {
            self.sparse[parent.id()]
                .as_mut()
                .unwrap()
                .remove_child(child);
        }
        let mut grand_parent = parent;
        while let Some(root_parent) =
            self.sparse[grand_parent.id()].as_ref().unwrap().parent()
        {
            if child == root_parent {
                let parent_data = self.sparse[parent.id()].as_mut().unwrap();
                grand_parent = parent_data.parent().unwrap();
                parent_data.set_parent(None).unwrap();
                self.sparse[grand_parent.id()]
                    .as_mut()
                    .unwrap()
                    .remove_child(parent);
                break;
            }
            grand_parent = root_parent;
        }
        self.sparse[child.id()]
            .as_mut()
            .unwrap()
            .set_parent(Some(parent))
            .unwrap();
        self.sparse[parent.id()]
            .as_mut()
            .unwrap()
            .insert_child(child)
            .unwrap();
    }

    pub fn unbind(&mut self, child: Entity) {
        if let Some(Some(entity_data)) = self.sparse.get_mut(child.id())
            && let Some(parent) = entity_data.parent()
        {
            entity_data.set_parent(None).unwrap();
            self.sparse[parent.id()]
                .as_mut()
                .unwrap()
                .remove_child(child);
        }
    }

    pub fn destroy(&mut self, entity: Entity) {
        if let Some(Some(entity_data)) = self.sparse.get(entity.id()) {
            if let Some(parent) = entity_data.parent() {
                self.sparse[parent.id()]
                    .as_mut()
                    .unwrap()
                    .remove_child(entity);
            }
            self.destroy_branch(entity);
        }
    }

    fn destroy_branch(&mut self, entity: Entity) {
        let mut entity_data = self.sparse[entity.id()].take().unwrap();
        for child in entity_data.children() {
            self.destroy_branch(*child);
        }
        entity_data.clear();
        self.destroyed.push(entity_data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bind() {
        let mut entity_manager = EntityManager::new();
        let entity0 = entity_manager.spawn();
        entity_manager.bind(entity0, entity0);
        assert!(entity_manager.get(entity0).unwrap().parent().is_none());
        assert!(entity_manager.get(entity0).unwrap().children().is_empty());

        let mut entity_manager = EntityManager::new();
        let entity0 = entity_manager.spawn();
        let entity1 = entity_manager.spawn();
        entity_manager.bind(entity0, entity1);
        assert!(entity_manager.get(entity0).unwrap().parent().is_none());
        assert_eq!(entity_manager.get(entity0).unwrap().children(), [entity1]);
        assert_eq!(
            entity_manager.get(entity1).unwrap().parent(),
            Some(entity0)
        );
        assert!(entity_manager.get(entity1).unwrap().children().is_empty());
        entity_manager.bind(entity1, entity0);
        assert_eq!(
            entity_manager.get(entity0).unwrap().parent(),
            Some(entity1)
        );
        assert!(entity_manager.get(entity0).unwrap().children().is_empty());
        assert!(entity_manager.get(entity1).unwrap().parent().is_none());
        assert_eq!(entity_manager.get(entity1).unwrap().children(), [entity0]);

        let mut entity_manager = EntityManager::new();
        let entity0 = entity_manager.spawn();
        let entity1 = entity_manager.spawn();
        let entity2 = entity_manager.spawn();
        entity_manager.bind(entity0, entity1);
        entity_manager.bind(entity1, entity2);
        assert!(entity_manager.get(entity0).unwrap().parent().is_none());
        assert_eq!(entity_manager.get(entity0).unwrap().children(), [entity1]);
        assert_eq!(
            entity_manager.get(entity1).unwrap().parent(),
            Some(entity0)
        );
        assert_eq!(entity_manager.get(entity1).unwrap().children(), [entity2]);
        assert_eq!(
            entity_manager.get(entity2).unwrap().parent(),
            Some(entity1)
        );
        assert!(entity_manager.get(entity2).unwrap().children().is_empty());
        entity_manager.bind(entity2, entity0);
        assert_eq!(
            entity_manager.get(entity0).unwrap().parent(),
            Some(entity2)
        );
        assert_eq!(entity_manager.get(entity0).unwrap().children(), [entity1]);
        assert_eq!(
            entity_manager.get(entity1).unwrap().parent(),
            Some(entity0)
        );
        assert!(entity_manager.get(entity1).unwrap().children().is_empty());
        assert!(entity_manager.get(entity2).unwrap().parent().is_none());
        assert_eq!(entity_manager.get(entity2).unwrap().children(), [entity0]);
    }
}
