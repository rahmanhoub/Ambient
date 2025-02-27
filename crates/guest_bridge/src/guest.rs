pub use ambient_api as api;
pub use ambient_api::components::core as components;

use std::future::Future;
pub fn run_async(_world: &ecs::World, future: impl Future<Output = ()> + Send + 'static) {
    ambient_api::prelude::run_async(async {
        future.await;
        api::prelude::EventOk
    });
}
pub async fn sleep(seconds: f32) {
    ambient_api::prelude::sleep(seconds).await;
}

pub mod ecs {
    use ambient_api::ecs::SupportedValueGet;
    pub use ambient_api::{
        ecs::{Component, SupportedValueSet as ComponentValue, UntypedComponent},
        prelude::{Entity, EntityId},
    };

    #[derive(Clone, Copy)]
    pub struct World;
    impl World {
        pub fn spawn(&self, entity: Entity) -> EntityId {
            ambient_api::entity::spawn(&entity)
        }
        pub fn despawn(&self, entity_id: EntityId) -> bool {
            ambient_api::entity::despawn(entity_id)
        }
        pub fn set<T: ComponentValue>(&self, entity_id: EntityId, component: Component<T>, value: T) -> Result<(), ECSError> {
            // TODO: set_component needs to return errors
            ambient_api::entity::set_component(entity_id, component, value);
            Ok(())
        }
        pub fn add_component<T: ComponentValue>(&self, entity_id: EntityId, component: Component<T>, value: T) -> Result<(), ECSError> {
            // TODO: add_component needs to return errors
            ambient_api::entity::add_component(entity_id, component, value);
            Ok(())
        }
        pub fn add_components(&self, entity_id: EntityId, components: Entity) -> Result<(), ECSError> {
            // TODO: add_components needs to return errors
            ambient_api::entity::add_components(entity_id, components);
            Ok(())
        }
        pub fn get<T: ComponentValue + SupportedValueGet>(&self, entity_id: EntityId, component: Component<T>) -> Result<T, ECSError> {
            ambient_api::entity::get_component(entity_id, component).ok_or_else(|| ECSError::EntityDoesntHaveComponent)
        }
        // TODO: This should actually return &T
        pub fn get_ref<T: ComponentValue + SupportedValueGet>(&self, entity_id: EntityId, component: Component<T>) -> Result<T, ECSError> {
            self.get(entity_id, component)
        }
        pub fn has_component<T: SupportedValueGet>(&self, entity_id: EntityId, component: Component<T>) -> bool {
            ambient_api::entity::has_component(entity_id, component)
        }
        pub fn resource<T: ComponentValue + SupportedValueGet>(&self, component: Component<T>) -> T {
            ambient_api::entity::get_component(ambient_api::entity::resources(), component).unwrap()
        }
    }
    #[derive(Debug)]
    pub enum ECSError {
        EntityDoesntHaveComponent,
        NoSuchEntity,
    }

    pub struct ComponentDesc(Box<dyn UntypedComponent>);
    impl ComponentDesc {
        pub fn index(&self) -> u32 {
            self.0.index()
        }
    }
    impl<T: 'static> From<Component<T>> for ComponentDesc {
        fn from(value: Component<T>) -> Self {
            Self(Box::new(value))
        }
    }
}

pub mod window {
    use ambient_window_types::CursorIcon;

    pub fn set_cursor(_world: &crate::ecs::World, _cursor: CursorIcon) {
        // TODO: Once we have client side scripting this needs to be hooked up to that
    }
    pub fn get_clipboard() -> Option<String> {
        None
    }
}
