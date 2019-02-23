
use std::collections::HashMap;

pub enum RuntimeValue {
    RuntimeInteger,
    RuntimeFloat,
    RuntimeBoolean,
    RuntimeString,
    RuntimeNone,
}

pub struct Scope {
    parent: Option<Box<Scope>>,
    map: HashMap<String, RuntimeValue>,
}


impl Scope {
    pub fn new(sc: Option<Box<Scope>>) -> Scope {
        Scope {
            parent: sc,
            map: HashMap::new(),
        }
    }

    pub fn find(&self, key: String) -> Option<&RuntimeValue> {
        self.map.get(&key).or_else(|| 
            if let Some(m) = &self.parent {
                m.find(key)
            } else { 
                None 
            })
    }

    pub fn insert(&mut self, key: String, val: RuntimeValue) {
        self.map.insert(key, val);
    }
}
