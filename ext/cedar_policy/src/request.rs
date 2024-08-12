use cedar_policy::{Context, Request};
use magnus::{function, method, Error, Module, Object, RModule, Ruby};
use std::convert::Into;

use crate::entity_uid::EntityUidWrapper;

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
                principal.map(&Into::into),
                action.map(&Into::into),
                resource.map(&Into::into),
                Context::empty(),
                None,
            )
            .unwrap(),
        )
    }

    fn principal(&self) -> Option<EntityUidWrapper> {
        self.0.principal().map(&Into::into)
    }

    fn action(&self) -> Option<EntityUidWrapper> {
        self.0.action().map(&Into::into)
    }

    fn resource(&self) -> Option<EntityUidWrapper> {
        self.0.resource().map(&Into::into)
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
