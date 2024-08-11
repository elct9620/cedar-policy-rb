use std::str::FromStr;

use cedar_policy::{EntityId, EntityTypeName, EntityUid};
use magnus::{function, method, Error, Module, Object, RModule, Ruby};

#[magnus::wrap(class = "CedarPolicy::EntityUid")]
struct EntityUidWrapper(EntityUid);

impl EntityUidWrapper {
    fn new(ruby: &Ruby, entity_type: String, id: String) -> Result<Self, Error> {
        let id = EntityId::from_str(&id)
            .map_err(|e| Error::new(ruby.exception_arg_error(), e.to_string()))?;
        let entity_type = EntityTypeName::from_str(&entity_type)
            .map_err(|e| Error::new(ruby.exception_arg_error(), e.to_string()))?;

        return Ok(Self(EntityUid::from_type_name_and_id(entity_type, id)));
    }

    fn to_s(&self) -> String {
        self.0.to_string()
    }
}

pub fn init(ruby: &Ruby, module: &RModule) -> Result<(), Error> {
    let class = module.define_class("EntityUid", ruby.class_object())?;
    class.define_singleton_method("new", function!(EntityUidWrapper::new, 2))?;
    class.define_method("to_s", method!(EntityUidWrapper::to_s, 0))?;

    Ok(())
}
