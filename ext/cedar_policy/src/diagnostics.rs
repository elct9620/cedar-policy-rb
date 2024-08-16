use cedar_policy::Diagnostics;
use magnus::{method, Error, Module, RArray, Ruby};

use crate::{error::RAuthorizationError, CEDAR_POLICY};

#[magnus::wrap(class = "CedarPolicy::Diagnostics")]
pub struct RDiagnostics(Diagnostics);

impl RDiagnostics {
    fn reason(&self) -> Vec<String> {
        self.0.reason().map(|r| r.to_string()).collect()
    }

    fn errors(&self) -> RArray {
        self.0.errors().for_each(|e| println!("{}", e));
        let errors = self
            .0
            .errors()
            .map(|e| RAuthorizationError::from(e.clone()));

        RArray::from_iter(errors)
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
    class.define_method("errors", method!(RDiagnostics::errors, 0))?;

    Ok(())
}
