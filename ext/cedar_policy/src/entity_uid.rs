use std::str::FromStr;

use cedar_policy::{EntityId, EntityTypeName, EntityUid};
use magnus::{function, method, Error, Module, Object, RModule, Ruby};

#[magnus::wrap(class = "CedarPolicy::EntityUid")]
pub struct EntityUidWrapper(EntityUid);

impl EntityUidWrapper {
    fn new(ruby: &Ruby, entity_type: String, id: String) -> Result<Self, Error> {
        let id = EntityId::from_str(&id)
            .map_err(|e| Error::new(ruby.exception_arg_error(), e.to_string()))?;
        let entity_type = EntityTypeName::from_str(&entity_type)
            .map_err(|e| Error::new(ruby.exception_arg_error(), e.to_string()))?;

        return Ok(Self(EntityUid::from_type_name_and_id(entity_type, id)));
    }

    fn eq(&self, other: &EntityUidWrapper) -> bool {
        self.0.eq(&other.0)
    }

    fn to_s(&self) -> String {
        self.0.to_string()
    }
}

impl From<EntityUid> for EntityUidWrapper {
    fn from(uid: EntityUid) -> Self {
        Self(uid)
    }
}

impl From<&EntityUid> for EntityUidWrapper {
    fn from(uid: &EntityUid) -> Self {
        Self(uid.clone())
    }
}

impl From<&EntityUidWrapper> for EntityUid {
    fn from(wrapper: &EntityUidWrapper) -> Self {
        wrapper.0.clone()
    }
}

pub fn init(ruby: &Ruby, module: &RModule) -> Result<(), Error> {
    let class = module.define_class("EntityUid", ruby.class_object())?;
    class.define_singleton_method("new", function!(EntityUidWrapper::new, 2))?;
    class.define_method("==", method!(EntityUidWrapper::eq, 1))?;
    class.define_method("to_s", method!(EntityUidWrapper::to_s, 0))?;
    class.define_method("inspect", method!(EntityUidWrapper::to_s, 0))?;

    Ok(())
}
