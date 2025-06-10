use magnus::{value::Lazy, Error, RModule, Ruby};

mod authorizer;
mod context;
mod decision;
mod diagnostics;
mod entities;
mod entity_uid;
mod error;
mod policy_set;
mod schema;
mod request;
mod response;

static CEDAR_POLICY: Lazy<RModule> = Lazy::new(|ruby| ruby.define_module("CedarPolicy").unwrap());

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.get_inner(&CEDAR_POLICY);

    error::init(ruby)?;
    entity_uid::init(ruby)?;
    entities::init(ruby)?;
    decision::init(ruby)?;
    diagnostics::init(ruby)?;
    context::init(ruby)?;
    authorizer::init(ruby, &module)?;
    request::init(ruby, &module)?;
    response::init(ruby)?;
    schema::init(ruby, &module)?;
    policy_set::init(ruby, &module)?;

    Ok(())
}
