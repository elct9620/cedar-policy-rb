use cedar_policy::{Context, Request};
use magnus::{function, method, Error, Module, Object, RModule, Ruby};

use crate::entity::EntityUidWrapper;

#[magnus::wrap(class = "CedarPolicy::Request")]
pub struct RequestWrapper(Request);

impl RequestWrapper {
    fn new(
        principal: Option<&EntityUidWrapper>,
        action: Option<&EntityUidWrapper>,
        resource: Option<&EntityUidWrapper>,
    ) -> Self {
        Self(
            Request::new(
                principal.map(|p| p.to_entity_uid()),
                action.map(|a| a.to_entity_uid()),
                resource.map(|r| r.to_entity_uid()),
                Context::empty(),
                None,
            )
            .unwrap(),
        )
    }

    fn principal(&self) -> Option<EntityUidWrapper> {
        self.0
            .principal()
            .map(|p| EntityUidWrapper::wrap(p.clone()))
    }

    fn action(&self) -> Option<EntityUidWrapper> {
        self.0.action().map(|a| EntityUidWrapper::wrap(a.clone()))
    }

    fn resource(&self) -> Option<EntityUidWrapper> {
        self.0.resource().map(|r| EntityUidWrapper::wrap(r.clone()))
    }
}

pub fn init(ruby: &Ruby, module: &RModule) -> Result<(), Error> {
    let class = module.define_class("Request", ruby.class_object())?;
    class.define_singleton_method("new", function!(RequestWrapper::new, 3))?;
    class.define_method("principal", method!(RequestWrapper::principal, 0))?;
    class.define_method("action", method!(RequestWrapper::action, 0))?;
    class.define_method("resource", method!(RequestWrapper::resource, 0))?;

    Ok(())
}
