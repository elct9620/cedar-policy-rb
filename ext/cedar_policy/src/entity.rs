use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use cedar_policy::{Entity, RestrictedExpression};
use magnus::{
    function, method, scan_args::scan_args, Error, Module, Object, RHash, RModule, Ruby, Value,
};

use crate::{entity_uid::REntityUid, error::PARSE_ERROR};

#[magnus::wrap(class = "CedarPolicy::Entity")]
pub struct REntity(Entity);

impl REntity {
    fn new(ruby: &Ruby, args: &[Value]) -> Result<REntity, Error> {
        let args = scan_args::<_, _, (), (), (), ()>(args)?;
        let (uid,): (&REntityUid,) = args.required;
        let (attrs,): (Option<RHash>,) = args.optional;

        match attrs {
            Some(attrs) => Ok(Self(
                Entity::new(uid.into(), try_convert_attrs(ruby, &attrs)?, HashSet::new())
                    .map_err(|e| Error::new(ruby.get_inner(&PARSE_ERROR), e.to_string()))?,
            )),
            None => Ok(Self(Entity::with_uid(uid.into()))),
        }
    }

    fn uid(&self) -> REntityUid {
        self.0.uid().into()
    }
}

impl From<&Entity> for REntity {
    fn from(entity: &Entity) -> Self {
        Self(entity.clone())
    }
}

fn to_attr(
    ruby: &Ruby,
    key: String,
    value: String,
) -> Result<(String, RestrictedExpression), Error> {
    Ok((
        key,
        RestrictedExpression::from_str(&value)
            .map_err(|e| Error::new(ruby.get_inner(&PARSE_ERROR), e.to_string()))?,
    ))
}

fn try_convert_attrs(
    ruby: &Ruby,
    attrs: &RHash,
) -> Result<HashMap<String, RestrictedExpression>, Error> {
    Ok(attrs
        .to_hash_map::<String, String>()?
        .iter()
        .map(|(k, v)| to_attr(ruby, k.to_string(), v.to_string()))
        .collect::<Result<HashMap<String, RestrictedExpression>, Error>>()?)
}

pub fn init(ruby: &Ruby, module: &RModule) -> Result<(), Error> {
    let class = module.define_class("Entity", ruby.class_object())?;
    class.define_singleton_method("new", function!(REntity::new, -1))?;
    class.define_method("uid", method!(REntity::uid, 0))?;
    Ok(())
}
