use cedar_policy::Entities;
use magnus::{function, Error, Module, Object, RModule, Ruby};

#[magnus::wrap(class = "CedarPolicy::Entities")]
pub struct REntities(Entities);

impl REntities {
    fn new() -> Self {
        Self(Entities::empty())
    }

    fn from_json(json: String) -> Self {
        Self(Entities::from_json_str(&json, None).unwrap())
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
    Ok(())
}
