use std::str::FromStr;

use cedar_policy::{EntityId, EntityTypeName, EntityUid};
use magnus::{function, method, Error, Module, Object, RModule, Ruby};

#[magnus::wrap(class = "CedarPolicy::EntityUid")]
pub struct REntityUid(EntityUid);

impl REntityUid {
    fn new(ruby: &Ruby, entity_type: String, id: String) -> Result<Self, Error> {
        let id = EntityId::from_str(&id)
            .map_err(|e| Error::new(ruby.exception_arg_error(), e.to_string()))?;
        let entity_type = EntityTypeName::from_str(&entity_type)
            .map_err(|e| Error::new(ruby.exception_arg_error(), e.to_string()))?;

        return Ok(Self(EntityUid::from_type_name_and_id(entity_type, id)));
    }

    fn eq(&self, other: &REntityUid) -> bool {
        self.0.eq(&other.0)
    }
}

impl From<EntityUid> for REntityUid {
    fn from(uid: EntityUid) -> Self {
        Self(uid)
    }
}

impl From<&EntityUid> for REntityUid {
    fn from(uid: &EntityUid) -> Self {
        Self(uid.clone())
    }
}

impl From<&REntityUid> for EntityUid {
    fn from(wrapper: &REntityUid) -> Self {
        wrapper.0.clone()
    }
}

impl ToString for REntityUid {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

pub fn init(ruby: &Ruby, module: &RModule) -> Result<(), Error> {
    let class = module.define_class("EntityUid", ruby.class_object())?;
    class.define_singleton_method("new", function!(REntityUid::new, 2))?;
    class.define_method("==", method!(REntityUid::eq, 1))?;
    class.define_method("to_s", method!(REntityUid::to_string, 0))?;
    class.define_method("to_str", method!(REntityUid::to_string, 0))?;
    class.define_method("inspect", method!(REntityUid::to_string, 0))?;

    Ok(())
}
