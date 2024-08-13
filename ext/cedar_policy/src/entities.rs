use cedar_policy::Entities;
use magnus::{
    function, method, scan_args::scan_args, value::ReprValue, Error, Module, Object, RModule, Ruby,
    Value,
};

use crate::{entity::REntity, entity_uid::REntityUid, error::ENTITIES_ERROR};

#[magnus::wrap(class = "CedarPolicy::Entities")]
pub struct REntities(Entities);

impl REntities {
    fn new(args: &[Value]) -> Result<Self, Error> {
        let args = scan_args::<(), _, (), (), (), ()>(args)?;
        let (entities,): (Option<Value>,) = args.optional;

        match entities {
            Some(entities) => Self::from_value(entities),
            None => Ok(Self(Entities::empty())),
        }
    }

    fn from_value(value: Value) -> Result<Self, Error> {
        let handle = Ruby::get_with(value);
        match value.is_kind_of(handle.class_string()) {
            true => Self::from_json(&handle, value.to_string()),
            _ => match value.respond_to("to_json", false)? {
                true => Self::from_json(&handle, value.funcall_public("to_json", ())?),
                _ => Err(Error::new(
                    handle.get_inner(&ENTITIES_ERROR),
                    "Invalid entities".to_string(),
                )),
            },
        }
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
    class.define_singleton_method("new", function!(REntities::new, -1))?;
    class.define_method("get", method!(REntities::get, 1))?;
    Ok(())
}
