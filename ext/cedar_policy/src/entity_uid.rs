use cedar_policy::ffi::JsonValueWithNoDuplicateKeys;
use cedar_policy::EntityUid;
use magnus::{
    value::{Lazy, ReprValue},
    Class, Error, IntoValue, Module, RClass, Ruby, TryConvert, Value,
};
use serde_magnus::deserialize;

use crate::{error::PARSE_ERROR, CEDAR_POLICY};

static ENTITY_UID: Lazy<RClass> = Lazy::new(|ruby| {
    ruby.get_inner(&CEDAR_POLICY)
        .define_class("EntityUid", ruby.class_object())
        .unwrap()
});

pub struct EntityUidWrapper(EntityUid);

impl EntityUidWrapper {
    pub fn new(uid: EntityUid) -> Self {
        Self(uid)
    }
}

impl From<EntityUidWrapper> for EntityUid {
    fn from(value: EntityUidWrapper) -> EntityUid {
        value.0
    }
}

impl IntoValue for EntityUidWrapper {
    fn into_value_with(self, handle: &Ruby) -> Value {
        let type_name = self.0.type_name().to_string();
        let id = self.0.id().escaped().to_string();
        let class = handle.get_inner(&ENTITY_UID);

        class.new_instance((type_name, id)).unwrap().into()
    }
}

impl TryConvert for EntityUidWrapper {
    fn try_convert(value: Value) -> Result<Self, magnus::Error> {
        let ruby = Ruby::get_with(value);
        match value.respond_to("to_hash", false) {
            Ok(true) => {
                let value: Value = value.funcall_public("to_hash", ())?;
                let value: JsonValueWithNoDuplicateKeys = deserialize(&ruby, value)?;
                Ok(Self(EntityUid::from_json(value.into()).map_err(|e| {
                    Error::new(ruby.get_inner(&PARSE_ERROR), e.to_string())
                })?))
            }
            Err(e) => Err(Error::new(ruby.exception_runtime_error(), e.to_string())),
            _ => Err(Error::new(
                ruby.exception_arg_error(),
                format!("no implicit conversion of {} into EntityUid", unsafe {
                    value.classname()
                }),
            ))?,
        }
    }
}

pub fn to_euid_value(euid: &EntityUid) -> Value {
    EntityUidWrapper::new(euid.clone()).into_value_with(&Ruby::get().unwrap())
}

pub fn init(ruby: &Ruby) -> Result<(), Error> {
    Lazy::force(&ENTITY_UID, ruby);

    Ok(())
}
