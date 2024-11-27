use cedar_policy::{Request, Schema};
use magnus::{
    function, method,
    scan_args::{get_kwargs, scan_args},
    Error, Module, Object, RModule, Ruby, Value,
};
use std::convert::Into;

use crate::{
    context::ContextWrapper,
    entity_uid::{to_euid_value, EntityUidWrapper},
    error::REQUEST_VALIDATION_ERROR,
    schema::RSchema,
};

#[magnus::wrap(class = "CedarPolicy::Request")]
pub struct RRequest(Request);

impl RRequest {
    fn new(ruby: &Ruby, args: &[Value]) -> Result<Self, Error> {
        let args = scan_args::<_, (), (), (), _, ()>(args)?;
        let (principal, action, resource, context): (
            EntityUidWrapper,
            EntityUidWrapper,
            EntityUidWrapper,
            ContextWrapper,
        ) = args.required;
        let kw_args = get_kwargs::<_, (), (Option<&RSchema>,), ()>(args.keywords, &[], &["schema"])?;
        let (schema,) = kw_args.optional;
        let schema = schema.map(|s| Schema::from(s));

        Ok(Self(
            Request::new(
                principal.into(),
                action.into(),
                resource.into(),
                context.into(),
                schema.as_ref(),
            )
            .map_err(|e| Error::new(ruby.get_inner(&REQUEST_VALIDATION_ERROR), e.to_string()))?,
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
    class.define_singleton_method("new", function!(RRequest::new, -1))?;
    class.define_method("principal", method!(RRequest::principal, 0))?;
    class.define_method("action", method!(RRequest::action, 0))?;
    class.define_method("resource", method!(RRequest::resource, 0))?;

    Ok(())
}
