use cedar_policy::{Context, Request};
use magnus::{function, method, Error, Module, Object, RModule, Ruby, Value};
use std::convert::Into;

use crate::entity_uid::{to_euid_value, EntityUidWrapper};

#[magnus::wrap(class = "CedarPolicy::Request")]
pub struct RRequest(Request);

impl RRequest {
    fn new(
        ruby: &Ruby,
        principal: Option<EntityUidWrapper>,
        action: Option<EntityUidWrapper>,
        resource: Option<EntityUidWrapper>,
    ) -> Result<Self, Error> {
        Ok(Self(
            Request::new(
                principal.map(|p| p.into()),
                action.map(|a| a.into()),
                resource.map(|r| r.into()),
                Context::empty(),
                None,
            )
            .map_err(|e| Error::new(ruby.exception_runtime_error(), e.to_string()))?,
        ))
    }

    fn principal(&self) -> Option<Value> {
        self.0.principal().map(&to_euid_value)
    }

    fn action(&self) -> Option<Value> {
        self.0.action().map(&to_euid_value)
    }

    fn resource(&self) -> Option<Value> {
        self.0.resource().map(&to_euid_value)
    }
}

impl<'a> From<&'a RRequest> for &'a Request {
    fn from(request: &'a RRequest) -> Self {
        &request.0
    }
}

pub fn init(ruby: &Ruby, module: &RModule) -> Result<(), Error> {
    let class = module.define_class("Request", ruby.class_object())?;
    class.define_singleton_method("new", function!(RRequest::new, 3))?;
    class.define_method("principal", method!(RRequest::principal, 0))?;
    class.define_method("action", method!(RRequest::action, 0))?;
    class.define_method("resource", method!(RRequest::resource, 0))?;

    Ok(())
}
