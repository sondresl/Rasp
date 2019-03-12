
use std::collections::HashMap;
use crate::runtime::runtime::RuntimeValue::*;

#[derive(Debug)]
pub enum RuntimeValue {
    RuntimeInteger(i64),
    RuntimeFloat(f64),
    RuntimeBoolean(bool),
    RuntimeString(String),
    RuntimeNone,
}

impl RuntimeValue {

    pub fn add(self, other: RuntimeValue) -> RuntimeValue {
        match self {
            RuntimeInteger(v1) => match other {
                RuntimeInteger(v2) => RuntimeInteger(v1 + v2),
                RuntimeFloat(v2) => RuntimeFloat(v1 as f64 + v2),
                _ => self.add(other.int())
            },
            _ => unimplemented!()
        }
    }

    fn int(self) -> RuntimeValue {
        match self {
            RuntimeInteger(v) => RuntimeInteger(v),
            RuntimeFloat(v)   => RuntimeInteger(v as i64),
            _ => unimplemented!()
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
