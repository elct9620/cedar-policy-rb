use magnus::{value::Lazy, Error, RModule, Ruby};

mod error;
mod policy_set;

static CEDAR_POLICY: Lazy<RModule> = Lazy::new(|ruby| ruby.define_module("CedarPolicy").unwrap());

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.get_inner(&CEDAR_POLICY);

    error::init(ruby)?;
    policy_set::init(ruby, &module)?;

    Ok(())
}
