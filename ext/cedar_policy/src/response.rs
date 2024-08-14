use cedar_policy::Response;
use magnus::{method, Error, Module, RModule, Ruby};

use crate::decision::RDecision;

#[magnus::wrap(class = "CedarPolicy::Response")]
pub struct RResponse(Response);

impl RResponse {
    pub fn decision(&self) -> RDecision {
        self.0.decision().into()
    }
}

impl From<Response> for RResponse {
    fn from(response: Response) -> Self {
        RResponse(response)
    }
}

pub fn init(ruby: &Ruby, module: &RModule) -> Result<(), Error> {
    let class = module.define_class("Response", ruby.class_object())?;
    class.define_method("decision", method!(RResponse::decision, 0))?;
    Ok(())
}
