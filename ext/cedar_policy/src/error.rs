use cedar_policy::AuthorizationError;
use magnus::{value::Lazy, Error, ExceptionClass, Module, Ruby};

use crate::CEDAR_POLICY;

pub static PARSE_ERROR: Lazy<ExceptionClass> = Lazy::new(|ruby| {
    ruby.get_inner(&CEDAR_POLICY)
        .define_error("ParseError", ruby.exception_standard_error())
        .unwrap()
});

pub static ENTITIES_ERROR: Lazy<ExceptionClass> = Lazy::new(|ruby| {
    ruby.get_inner(&CEDAR_POLICY)
        .define_error("EntitiesError", ruby.exception_standard_error())
        .unwrap()
});

pub static AUTHORIZATION_ERROR: Lazy<ExceptionClass> = Lazy::new(|ruby| {
    ruby.get_inner(&CEDAR_POLICY)
        .define_error("AuthorizationError", ruby.exception_standard_error())
        .unwrap()
});

pub static REQUEST_VALIDATION_ERROR: Lazy<ExceptionClass> = Lazy::new(|ruby| {
    ruby.get_inner(&CEDAR_POLICY)
        .define_error("RequestValidationError", ruby.exception_standard_error())
        .unwrap()
});

#[magnus::wrap(class = "CedarPolicy::AuthorizationError")]
pub struct RAuthorizationError(AuthorizationError);

impl ToString for RAuthorizationError {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl From<AuthorizationError> for RAuthorizationError {
    fn from(error: AuthorizationError) -> Self {
        RAuthorizationError(error)
    }
}

pub fn init(ruby: &Ruby) -> Result<(), Error> {
    Lazy::force(&PARSE_ERROR, ruby);
    Lazy::force(&ENTITIES_ERROR, ruby);
    Lazy::force(&REQUEST_VALIDATION_ERROR, ruby);

    ruby.get_inner(&AUTHORIZATION_ERROR);

    Ok(())
}
