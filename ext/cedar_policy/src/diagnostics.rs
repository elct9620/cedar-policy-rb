use cedar_policy::Diagnostics;
use magnus::{method, Error, Module, Ruby};

use crate::CEDAR_POLICY;

#[magnus::wrap(class = "CedarPolicy::Diagnostics")]
pub struct RDiagnostics(Diagnostics);

impl RDiagnostics {
    fn reason(&self) -> Vec<String> {
        self.0.reason().map(|r| r.to_string()).collect()
    }
}

impl From<&Diagnostics> for RDiagnostics {
    fn from(diagnostics: &Diagnostics) -> Self {
        RDiagnostics(diagnostics.clone())
    }
}

pub fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.get_inner(&CEDAR_POLICY);
    let class = module.define_class("Diagnostics", ruby.class_object())?;
    class.define_method("reason", method!(RDiagnostics::reason, 0))?;

    Ok(())
}
