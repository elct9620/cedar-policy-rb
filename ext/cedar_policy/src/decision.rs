use cedar_policy::Decision;
use magnus::{
    method,
    typed_data::IsEql,
    value::{Lazy, ReprValue},
    Class, Error, IntoValue, Module, RClass, Ruby, TryConvert, Value,
};

use crate::CEDAR_POLICY;

static DECISION: Lazy<RClass> = Lazy::new(|ruby| {
    ruby.get_inner(&CEDAR_POLICY)
        .define_class("Decision", ruby.class_object())
        .unwrap()
});

pub static DECISION_ALLOW: Lazy<Value> =
    Lazy::new(|ruby| RDecision(Decision::Allow).into_value_with(ruby));

pub static DECISION_DENY: Lazy<Value> =
    Lazy::new(|ruby| RDecision(Decision::Deny).into_value_with(ruby));

#[magnus::wrap(class = "CedarPolicy::Decision")]
pub struct RDecision(Decision);

impl IsEql for RDecision {
    fn is_eql(&self, other: Value) -> bool {
        match <&RDecision>::try_convert(other) {
            Ok(other) => self.0 == other.0,
            Err(_) => {
                println!("other: {:?}", other);
                return (self.0 == Decision::Allow) == other.to_bool();
            }
        }
    }
}

impl ToString for RDecision {
    fn to_string(&self) -> String {
        match &self.0 {
            Decision::Allow => "Allow".to_string(),
            Decision::Deny => "Deny".to_string(),
        }
    }
}

impl From<Decision> for RDecision {
    fn from(decision: Decision) -> Self {
        RDecision(decision)
    }
}

pub fn init(ruby: &Ruby) -> Result<(), Error> {
    let class = ruby.get_inner(&DECISION);
    let allow = ruby.get_inner(&DECISION_ALLOW);
    let deny = ruby.get_inner(&DECISION_DENY);

    class.undef_default_alloc_func();

    class.const_set("ALLOW", allow)?;
    class.const_set("DENY", deny)?;

    class.define_method("==", method!(<RDecision as IsEql>::is_eql, 1))?;
    class.define_method("eql?", method!(<RDecision as IsEql>::is_eql, 1))?;
    class.define_method("to_s", method!(RDecision::to_string, 0))?;
    class.define_method("inspect", method!(RDecision::to_string, 0))?;
    Ok(())
}
