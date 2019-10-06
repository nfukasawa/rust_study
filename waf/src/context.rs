use std::collections::HashMap;

pub struct Context {
    params: Option<HashMap<String, String>>,
}

impl Context {
    pub fn new(params: Option<HashMap<String, String>>) -> Self {
        Self { params }
    }

    pub fn param<'a, S>(&self, id: S) -> Option<&String>
    where
        S: Into<&'a str>,
    {
        match &self.params {
            Some(params) => {
                let id = format!(":{}", id.into());
                params.get(&id)
            }
            None => None,
        }
    }
}
