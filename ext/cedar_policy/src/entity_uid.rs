use cedar_policy::EntityUid;
use cedar_policy_core::jsonvalue::JsonValueWithNoDuplicateKeys;
use magnus::{value::ReprValue, Error, Ruby, TryConvert, Value};
use serde_magnus::deserialize;

pub struct EntityUidWrapper(EntityUid);

impl From<EntityUidWrapper> for EntityUid {
    fn from(value: EntityUidWrapper) -> EntityUid {
        value.0
    }
}

impl TryConvert for EntityUidWrapper {
    fn try_convert(value: Value) -> Result<Self, magnus::Error> {
        let handle = Ruby::get_with(value);
        match value.respond_to("to_hash", false) {
            Ok(true) => {
                let value: Value = value.funcall_public("to_hash", ())?;
                let value: JsonValueWithNoDuplicateKeys = deserialize(value)?;
                Ok(Self(EntityUid::from_json(value.into()).map_err(|e| {
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
}
