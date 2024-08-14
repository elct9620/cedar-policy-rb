use cedar_policy::{Context, EntityUid, Request};
use cedar_policy_core::jsonvalue::JsonValueWithNoDuplicateKeys;
use magnus::{function, method, value::ReprValue, Error, Module, Object, RModule, Ruby, Value};
use serde_magnus::deserialize;
use std::convert::Into;

#[magnus::wrap(class = "CedarPolicy::Request")]
pub struct RRequest(Request);

impl RRequest {
    fn new(
        ruby: &Ruby,
        principal: Option<Value>,
        action: Option<Value>,
        resource: Option<Value>,
    ) -> Result<Self, Error> {
        Ok(Self(
            Request::new(
                parse_entity_uid(principal)?,
                parse_entity_uid(action)?,
                parse_entity_uid(resource)?,
                Context::empty(),
                None,
            )
            .map_err(|e| Error::new(ruby.exception_runtime_error(), e.to_string()))?,
        ))
    }

    fn principal(&self) -> Option<String> {
        self.0.principal().map(ToString::to_string)
    }

    fn action(&self) -> Option<String> {
        self.0.action().map(ToString::to_string)
    }

    fn resource(&self) -> Option<String> {
        self.0.resource().map(ToString::to_string)
    }
}

impl<'a> From<&'a RRequest> for &'a Request {
    fn from(request: &'a RRequest) -> Self {
        &request.0
    }
}

fn parse_entity_uid(value: Option<Value>) -> Result<Option<EntityUid>, Error> {
    match value {
        Some(value) => {
            let handle = Ruby::get_with(value);
            match value.respond_to("to_hash", false) {
                Ok(true) => {
                    let value: Value = value.funcall_public("to_hash", ())?;
                    let value: JsonValueWithNoDuplicateKeys = deserialize(value)?;
                    Ok(Some(EntityUid::from_json(value.into()).map_err(|e| {
                        Error::new(handle.exception_runtime_error(), e.to_string())
                    })?))
                }
                Err(e) => Err(Error::new(handle.exception_runtime_error(), e.to_string())),
                _ => Err(Error::new(
                    handle.exception_arg_error(),
                    format!("no implicit conversion of {} into EntityUid", unsafe {
                        value.classname()
                    }),
                ))?,
            }
        }
        None => Ok(None),
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
