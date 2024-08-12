use cedar_policy::Decision;
use magnus::{function, method, typed_data::IsEql, Class, Error, Module, Object, RModule, Ruby};

#[magnus::wrap(class = "CedarPolicy::Decision")]
pub struct RDecision(Decision);

fn allow() -> RDecision {
    RDecision(Decision::Allow)
}

fn deny() -> RDecision {
    RDecision(Decision::Deny)
}

impl PartialEq for RDecision {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for RDecision {}

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

pub fn init(ruby: &Ruby, module: &RModule) -> Result<(), Error> {
    let class = module.define_class("Decision", ruby.class_object())?;
    class.undef_default_alloc_func();

    class.define_singleton_method("allow", function!(allow, 0))?;
    class.define_singleton_method("deny", function!(deny, 0))?;

    class.define_method("==", method!(<RDecision as PartialEq>::eq, 1))?;
    class.define_method("eql?", method!(<RDecision as IsEql>::is_eql, 1))?;
    class.define_method("to_s", method!(RDecision::to_string, 0))?;
    class.define_method("inspect", method!(RDecision::to_string, 0))?;
    Ok(())
}
