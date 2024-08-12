use cedar_policy::Entities;
use magnus::{function, method, Error, Module, Object, RModule, Ruby};

use crate::{entity::REntity, entity_uid::REntityUid, error::ENTITIES_ERROR};

#[magnus::wrap(class = "CedarPolicy::Entities")]
pub struct REntities(Entities);

impl REntities {
    fn new() -> Self {
        Self(Entities::empty())
    }

    fn from_json(ruby: &Ruby, json: String) -> Result<Self, Error> {
        let entities = Entities::from_json_str(&json, None)
            .map_err(|error| Error::new(ruby.get_inner(&ENTITIES_ERROR), error.to_string()))?;
        Ok(Self(entities))
    }

    fn get(&self, uid: &REntityUid) -> Option<REntity> {
        self.0.get(&uid.into()).map(|entity| entity.into())
    }
}

impl From<&REntities> for Entities {
    fn from(entities: &REntities) -> Self {
        entities.0.clone()
    }
}

pub fn init(ruby: &Ruby, module: &RModule) -> Result<(), Error> {
    let class = module.define_class("Entities", ruby.class_object())?;
    class.define_singleton_method("new", function!(REntities::new, 0))?;
    class.define_singleton_method("from_json", function!(REntities::from_json, 1))?;
    class.define_method("get", method!(REntities::get, 1))?;
    Ok(())
}
