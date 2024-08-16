use cedar_policy::Diagnostics;
use magnus::{Error, Module, Ruby};

use crate::CEDAR_POLICY;

#[magnus::wrap(class = "CedarPolicy::Diagnostics")]
pub struct RDiagnostics(Diagnostics);

impl From<&Diagnostics> for RDiagnostics {
    fn from(diagnostics: &Diagnostics) -> Self {
        RDiagnostics(diagnostics.clone())
    }
}

pub fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.get_inner(&CEDAR_POLICY);
    module.define_class("Diagnostics", ruby.class_object())?;

    Ok(())
}
