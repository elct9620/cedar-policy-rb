use cedar_policy::ffi::JsonValueWithNoDuplicateKeys;
use cedar_policy::Entities;
use magnus::{
    value::{Lazy, ReprValue},
    Error, Module, RClass, Ruby, TryConvert, Value,
};
use serde_magnus::deserialize;

use crate::CEDAR_POLICY;

static ENTITIES: Lazy<RClass> = Lazy::new(|ruby| {
    ruby.get_inner(&CEDAR_POLICY)
        .define_class("Entities", ruby.class_object())
        .unwrap()
});

pub struct EntitiesWrapper(Entities);

impl From<EntitiesWrapper> for Entities {
    fn from(value: EntitiesWrapper) -> Self {
        value.0
    }
}

impl TryConvert for EntitiesWrapper {
    fn try_convert(value: Value) -> Result<Self, Error> {
        let handle = Ruby::get_with(value);
        match value.respond_to("to_ary", false) {
            Ok(true) => {
                let value: Value = value.funcall_public("to_ary", ())?;
                let value: JsonValueWithNoDuplicateKeys = deserialize(&handle, value)?;
                Ok(Self(
                    Entities::from_json_value(value.into(), None)
                        .map_err(|e| Error::new(handle.exception_arg_error(), e.to_string()))?,
                ))
            }
            Err(e) => Err(Error::new(handle.exception_arg_error(), e.to_string())),
            _ => Err(Error::new(
                handle.exception_arg_error(),
                format!("no implicit conversion of {} into Entities", unsafe {
                    value.classname()
                }),
            ))?,
        }
    }
}

pub fn init(ruby: &Ruby) -> Result<(), Error> {
    Lazy::force(&ENTITIES, ruby);
    Ok(())
}
