use magnus::{value::Lazy, Error, ExceptionClass, Module, Ruby};

use crate::CEDAR_POLICY;

pub static PARSE_ERROR: Lazy<ExceptionClass> = Lazy::new(|ruby| {
    ruby.get_inner(&CEDAR_POLICY)
        .define_error("ParseError", ruby.exception_standard_error())
        .unwrap()
});

pub fn init(ruby: &Ruby) -> Result<(), Error> {
    Lazy::force(&PARSE_ERROR, ruby);

    Ok(())
}
