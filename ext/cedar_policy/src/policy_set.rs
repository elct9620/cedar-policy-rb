use std::str::FromStr;

use cedar_policy::PolicySet;
use magnus::{function, method, Error, Module, Object, RModule, Ruby};

use crate::error::PARSE_ERROR;

#[magnus::wrap(class = "CedarPolicy::PolicySet")]
struct PolicySetWrapper(PolicySet);

impl PolicySetWrapper {
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

pub fn init(ruby: &Ruby, module: &RModule) -> Result<(), Error> {
    let class = module.define_class("PolicySet", ruby.class_object())?;
    class.define_singleton_method("new", function!(PolicySetWrapper::new, 0))?;
    class.define_singleton_method("from_str", function!(PolicySetWrapper::from_str, 1))?;
    class.define_method("empty?", method!(PolicySetWrapper::is_empty, 0))?;

    Ok(())
}
