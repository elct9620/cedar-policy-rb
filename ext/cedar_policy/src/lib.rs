use magnus::{value::Lazy, Error, RModule, Ruby};

mod authorizer;
mod decision;
mod entities;
mod entity_uid;
mod error;
mod policy_set;
mod request;
mod response;

static CEDAR_POLICY: Lazy<RModule> = Lazy::new(|ruby| ruby.define_module("CedarPolicy").unwrap());

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.get_inner(&CEDAR_POLICY);

    error::init(ruby)?;
    authorizer::init(ruby, &module)?;
    entity_uid::init(ruby, &module)?;
    entities::init(ruby, &module)?;
    request::init(ruby, &module)?;
    response::init(ruby, &module)?;
    policy_set::init(ruby, &module)?;
    decision::init(ruby, &module)?;

    Ok(())
}
