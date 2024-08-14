use cedar_policy::{Authorizer, Decision, Entities, PolicySet, Request, Response};
use cedar_policy_core::jsonvalue::JsonValueWithNoDuplicateKeys;
use magnus::{function, method, Error, Module, Object, RModule, Ruby, Value};
use serde_magnus::deserialize;

use crate::{policy_set::RPolicySet, request::RRequest, response::RResponse};

#[magnus::wrap(class = "CedarPolicy::Authorizer")]
pub struct RAuthorizer(Authorizer);

impl RAuthorizer {
    fn new() -> Self {
        RAuthorizer(Authorizer::new())
    }

    fn is_authorized(&self, request: &RRequest, policy: &RPolicySet, entities: Value) -> bool {
        match is_authorized(
            &Ruby::get_with(entities),
            &self.0,
            request.into(),
            &policy.into(),
            entities,
        ) {
            Ok(response) => response.decision() == Decision::Allow,
            Err(_) => false,
        }
    }

    fn authorize(
        &self,
        request: &RRequest,
        policy: &RPolicySet,
        entities: Value,
    ) -> Result<RResponse, Error> {
        match is_authorized(
            &Ruby::get_with(entities),
            &self.0,
            request.into(),
            &policy.into(),
            entities,
        ) {
            Ok(response) => Ok(response.into()),
            Err(error) => Err(error),
        }
    }
}

fn is_authorized(
    ruby: &Ruby,
    authorizer: &Authorizer,
    request: &Request,
    policy: &PolicySet,
    entities: Value,
) -> Result<Response, Error> {
    let entities: JsonValueWithNoDuplicateKeys = deserialize(entities).map_err(|error| {
        Error::new(
            ruby.exception_arg_error(),
            format!("Invalid entities: {}", error.to_string()),
        )
    })?;
    let entities = Entities::from_json_value(entities.into(), None).map_err(|error| {
        Error::new(
            ruby.exception_arg_error(),
            format!("Invalid entities: {}", error.to_string()),
        )
    })?;

    return Ok(authorizer.is_authorized(request, policy, &entities));
}

pub fn init(ruby: &Ruby, module: &RModule) -> Result<(), Error> {
    let class = module.define_class("Authorizer", ruby.class_object())?;
    class.define_singleton_method("new", function!(RAuthorizer::new, 0))?;
    class.define_method("authorize", method!(RAuthorizer::authorize, 3))?;
    class.define_method("authorized?", method!(RAuthorizer::is_authorized, 3))?;
    Ok(())
}
