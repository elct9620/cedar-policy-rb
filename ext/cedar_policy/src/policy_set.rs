use std::str::FromStr;

use cedar_policy::PolicySet;
use magnus::{function, method, Error, Module, Object, RModule, Ruby};

use crate::error::PARSE_ERROR;

#[magnus::wrap(class = "CedarPolicy::PolicySet")]
pub struct RPolicySet(PolicySet);

impl RPolicySet {
    fn new() -> Self {
        Self(PolicySet::new())
    }

    fn from_str(ruby: &Ruby, policy: String) -> Result<Self, Error> {
        let policy = PolicySet::from_str(&policy)
            .map_err(|e| Error::new(ruby.get_inner(&PARSE_ERROR), e.to_string()))?;
        Ok(Self(policy))
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl From<&RPolicySet> for PolicySet {
    fn from(policy: &RPolicySet) -> Self {
        policy.0.clone()
    }
}

pub fn init(ruby: &Ruby, module: &RModule) -> Result<(), Error> {
    let class = module.define_class("PolicySet", ruby.class_object())?;
    class.define_singleton_method("new", function!(RPolicySet::new, 0))?;
    class.define_singleton_method("from_str", function!(RPolicySet::from_str, 1))?;
    class.define_method("empty?", method!(RPolicySet::is_empty, 0))?;

    Ok(())
}
