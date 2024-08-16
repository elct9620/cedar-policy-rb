use cedar_policy::Response;
use magnus::{method, Error, Module, Ruby};

use crate::{decision::RDecision, diagnostics::RDiagnostics, CEDAR_POLICY};

#[magnus::wrap(class = "CedarPolicy::Response")]
pub struct RResponse(Response);

impl RResponse {
    pub fn decision(&self) -> RDecision {
        self.0.decision().into()
    }

    pub fn diagnostics(&self) -> RDiagnostics {
        self.0.diagnostics().into()
    }
}

impl From<Response> for RResponse {
    fn from(response: Response) -> Self {
        RResponse(response)
    }
}

pub fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.get_inner(&CEDAR_POLICY);
    let class = module.define_class("Response", ruby.class_object())?;
    class.define_method("decision", method!(RResponse::decision, 0))?;
    class.define_method("diagnostics", method!(RResponse::diagnostics, 0))?;

    Ok(())
}
