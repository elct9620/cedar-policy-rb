use cedar_policy::Context;
use cedar_policy_core::jsonvalue::JsonValueWithNoDuplicateKeys;
use magnus::{
    value::{Lazy, ReprValue},
    Error, Module, RClass, Ruby, TryConvert, Value,
};
use serde_magnus::deserialize;

use crate::CEDAR_POLICY;

static CONTEXT: Lazy<RClass> = Lazy::new(|ruby| {
    ruby.get_inner(&CEDAR_POLICY)
        .define_class("Context", ruby.class_object())
        .unwrap()
});

pub struct ContextWrapper(Context);

impl From<ContextWrapper> for Context {
    fn from(value: ContextWrapper) -> Self {
        value.0
    }
}

impl TryConvert for ContextWrapper {
    fn try_convert(value: Value) -> Result<Self, Error> {
        let handle = Ruby::get_with(value);
        match value.respond_to("to_hash", false) {
            Ok(true) => {
                let value: Value = value.funcall_public("to_hash", ())?;
                let value: JsonValueWithNoDuplicateKeys = deserialize(value)?;
                Ok(Self(Context::from_json_value(value.into(), None).map_err(
                    |e| Error::new(handle.exception_runtime_error(), e.to_string()),
                )?))
            }
            Err(e) => Err(Error::new(handle.exception_runtime_error(), e.to_string())),
            _ => Err(Error::new(
                handle.exception_arg_error(),
                format!("no implicit conversion of {} into Context", unsafe {
                    value.classname()
                }),
            ))?,
        }
    }
}

pub fn init(ruby: &Ruby) -> Result<(), Error> {
    Lazy::force(&CONTEXT, ruby);

    Ok(())
}
