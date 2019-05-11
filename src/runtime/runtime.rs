
use std::collections::HashMap;
use crate::runtime::runtime::RuntimeValue::*;
use std::fmt;

#[derive(Debug, Clone)]
pub enum RuntimeValue {
    RuntimeInteger(i64),
    RuntimeFloat(f64),
    RuntimeBoolean(bool),
    RuntimeString(String),
    RuntimeFunc(fn(Vec<RuntimeValue>) -> RuntimeValue),
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

    pub fn minus(self, other: RuntimeValue) -> RuntimeValue {
        match self {
            RuntimeInteger(v1) => match other {
                RuntimeInteger(v2) => RuntimeInteger(v1 - v2),
                RuntimeFloat(v2)   => RuntimeFloat(v1 as f64 - v2),
                _ => unimplemented!()
            },
            _ => unimplemented!()
        }
    }
    
    pub fn multiply(self, other: RuntimeValue) -> RuntimeValue {
        match self {
            RuntimeInteger(v1) => match other {
                RuntimeInteger(v2) => RuntimeInteger(v1 * v2),
                RuntimeFloat(v2) => RuntimeFloat(v1 as f64 * v2),
                _ => self.multiply(other.int()),
            },
            RuntimeFloat(v1) => match other {
                RuntimeFloat(v2)   => RuntimeFloat(v1 * v2),
                RuntimeInteger(v2) => RuntimeFloat(v1 + (v2 as f64)),
                _ => self.multiply(other.float()), 
            },

            _ => unimplemented!()
        }
    }

    pub fn divide(self, other: RuntimeValue) -> RuntimeValue {
        match self {
            RuntimeInteger(v1) => match other {
                RuntimeInteger(v2) => RuntimeFloat(v1 as f64 / v2 as f64),
                RuntimeFloat(v2)   => RuntimeFloat(v1 as f64 / v2),
                _ => self.divide(other.int()),
            },
            _ => unimplemented!()
        }

    }

    pub fn float(self) -> RuntimeValue {
        match self {
            RuntimeFloat(v)   => self,
            RuntimeInteger(v) => RuntimeFloat(v as f64),
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

impl fmt::Display for RuntimeValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RuntimeString(v) => write!(f, "{}", v),
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
pub struct Scope {
    parent: Option<Box<Scope>>,
    map: HashMap<String, RuntimeValue>,
}


impl Scope {

    pub fn create_standard_lib() -> Self {

        let mut lib: HashMap<String, RuntimeValue> = HashMap::new();

        lib.insert("print".to_string(), 
                   RuntimeFunc(|args: Vec<RuntimeValue>| {
            for v in args {
                print!("{} ", v);
            }
            println!();
            RuntimeValue::RuntimeNone
        }));

        let rv = Scope { parent: None, map: lib };

        rv
    }

    
    pub fn new(sc: Option<Box<Scope>>) -> Scope {
        Scope {
            parent: sc,
            map: HashMap::new(),
        }
    }

    pub fn find(&self, key: String) -> Option<&RuntimeValue> {
        self.map.get(&key)
                .or_else(|| self.parent.as_ref().and_then(|m| m.find(key)))
    }

    pub fn insert(&mut self, key: String, val: RuntimeValue) {
        self.map.insert(key, val);
    }
}
