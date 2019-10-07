use std::any::Any;
use std::collections::HashMap;

#[derive(Debug)]
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

    pub fn value<'b, S, T>(&self, id: S) -> Option<&T>
    where
        S: Into<&'b str>,
        T: 'static,
    {
        match self.values.get(id.into()) {
            Some(val) => val.downcast_ref::<T>(),
            None => None,
        }
    }

    pub fn set_value<S, T>(&mut self, id: S, val: T)
    where
        S: Into<String>,
        T: 'static,
    {
        self.values.insert(id.into(), Box::new(val));
    }
}
