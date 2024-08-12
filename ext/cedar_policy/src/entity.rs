use cedar_policy::Entity;
use magnus::{function, method, Error, Module, Object, RModule, Ruby};

use crate::entity_uid::REntityUid;

#[magnus::wrap(class = "CedarPolicy::Entity")]
pub struct REntity(Entity);

impl REntity {
    fn new(uid: &REntityUid) -> Self {
        Self(Entity::with_uid(uid.into()))
    }

    fn uid(&self) -> REntityUid {
        self.0.uid().into()
    }
}

pub fn init(ruby: &Ruby, module: &RModule) -> Result<(), Error> {
    let class = module.define_class("Entity", ruby.class_object())?;
    class.define_singleton_method("new", function!(REntity::new, 1))?;
    class.define_method("uid", method!(REntity::uid, 0))?;
    Ok(())
}
