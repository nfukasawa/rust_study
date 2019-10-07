use std::any::Any;
use std::collections::HashMap;

pub struct Context<'a> {
    params: Option<Vec<(&'a str, &'a str)>>,
    values: HashMap<String, Box<dyn Any>>,
}

impl<'a> Context<'a> {
    pub fn new() -> Self {
        Self {
            params: None,
            values: HashMap::new(),
        }
    }

    pub fn param<'b, S>(&self, id: S) -> Option<&str>
    where
        S: Into<&'b str>,
    {
        match &self.params {
            Some(params) => {
                let id = id.into();
                for (k, v) in params.iter() {
                    if id == *k {
                        return Some(v);
                    }
                }
                None
            }
            None => None,
        }
    }

    pub fn set_params(&mut self, params: Vec<(&'a str, &'a str)>) {
        self.params = Some(params);
    }

    pub fn value<'b, S>(&self, id: S) -> Option<&dyn Any>
    where
        S: Into<&'b str>,
    {
        match self.values.get(id.into()) {
            Some(val) => Some(&*val),
            None => None,
        }
    }

    pub fn set_value<S, T>(&mut self, id: S, val: Box<dyn Any>)
    where
        S: Into<String>,
    {
        self.values.insert(id.into(), val);
    }
}
