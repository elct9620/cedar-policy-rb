use std::str::FromStr;

use cedar_policy::{ParseErrors, PolicySet};
use magnus::{function, method, scan_args::scan_args, Error, Module, Object, RModule, Ruby, Value};

use crate::error::PARSE_ERROR;

#[magnus::wrap(class = "CedarPolicy::PolicySet")]
pub struct RPolicySet(PolicySet);

impl RPolicySet {
    fn new(ruby: &Ruby, args: &[Value]) -> Result<Self, Error> {
        let args = scan_args::<(), _, (), (), (), ()>(args)?;
        let (policy,): (Option<String>,) = args.optional;

        match policy {
            Some(policy) => Self::from_str(&policy)
                .map_err(|e| Error::new(ruby.get_inner(&PARSE_ERROR), e.to_string())),
            None => Ok(Self(PolicySet::new())),
        }
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn policy_ids(&self) -> Vec<String> {
        self.0.policies()
            .map(|policy| {
                // Try to get the @id annotation first, fall back to auto-generated ID
                policy.annotation("id")
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| policy.id().to_string())
            })
            .collect()
    }
}

impl From<&RPolicySet> for PolicySet {
    fn from(policy: &RPolicySet) -> Self {
        policy.0.clone()
    }
}

impl FromStr for RPolicySet {
    type Err = ParseErrors;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(PolicySet::from_str(s)?))
    }
}

pub fn init(ruby: &Ruby, module: &RModule) -> Result<(), Error> {
    let class = module.define_class("PolicySet", ruby.class_object())?;
    class.define_singleton_method("new", function!(RPolicySet::new, -1))?;
    class.define_method("empty?", method!(RPolicySet::is_empty, 0))?;
    class.define_method("policy_ids", method!(RPolicySet::policy_ids, 0))?;

    Ok(())
}
