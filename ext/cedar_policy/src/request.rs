use cedar_policy::{Context, Request};
use magnus::{function, method, Error, Module, Object, RModule, Ruby};
use std::convert::Into;

use crate::entity_uid::REntityUid;

#[magnus::wrap(class = "CedarPolicy::Request")]
pub struct RRequest(Request);

impl RRequest {
    fn new(
        principal: Option<&REntityUid>,
        action: Option<&REntityUid>,
        resource: Option<&REntityUid>,
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

    fn principal(&self) -> Option<REntityUid> {
        self.0.principal().map(&Into::into)
    }

    fn action(&self) -> Option<REntityUid> {
        self.0.action().map(&Into::into)
    }

    fn resource(&self) -> Option<REntityUid> {
        self.0.resource().map(&Into::into)
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
