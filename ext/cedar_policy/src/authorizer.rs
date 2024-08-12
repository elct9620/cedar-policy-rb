use cedar_policy::{Authorizer, Decision};
use magnus::{function, method, Error, Module, Object, RModule, Ruby};

use crate::{entities::REntities, policy_set::RPolicySet, request::RRequest, response::RResponse};

#[magnus::wrap(class = "CedarPolicy::Authorizer")]
pub struct RAuthorizer(Authorizer);

impl RAuthorizer {
    fn new() -> Self {
        RAuthorizer(Authorizer::new())
    }

    fn is_authorized(&self, request: &RRequest, policy: &RPolicySet, entities: &REntities) -> bool {
        let response = self
            .0
            .is_authorized(request.into(), &policy.into(), &entities.into());
        response.decision() == Decision::Allow
    }

    fn authorize(
        &self,
        request: &RRequest,
        policy: &RPolicySet,
        entities: &REntities,
    ) -> RResponse {
        self.0
            .is_authorized(request.into(), &policy.into(), &entities.into())
            .into()
    }
}

pub fn init(ruby: &Ruby, module: &RModule) -> Result<(), Error> {
    let class = module.define_class("Authorizer", ruby.class_object())?;
    class.define_singleton_method("new", function!(RAuthorizer::new, 0))?;
    class.define_method("authorize", method!(RAuthorizer::authorize, 3))?;
    class.define_method("authorized?", method!(RAuthorizer::is_authorized, 3))?;
    Ok(())
}
