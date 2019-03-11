
use std::collections::HashMap;
use crate::runtime::runtime::RuntimeValue::*;

pub enum RuntimeValue {
    RuntimeInteger(i64),
    RuntimeFloat(f64),
    RuntimeBoolean(bool),
    RuntimeString(String),
    RuntimeNone,
}

impl RuntimeValue {

    fn add(self, other: RuntimeValue) -> RuntimeValue {
        match self {
            RuntimeInteger(v1) => match other {
                RuntimeFloat(v2) => RuntimeFloat(v1 as f64 + v2),
                _ => RuntimeInteger(v1 + other.int())
            }
        }
    }

    fn int(self) -> RuntimeValue {
        match self {
            RuntimeInteger(v) |
            RuntimeFloat(v)   => RuntimeInteger(v as i64),
            _ => panic!()
        }
    }
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
