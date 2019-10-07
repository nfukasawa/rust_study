pub struct Context<'a> {
    params: Vec<(&'a str, &'a str)>,
}

impl<'a> Context<'a> {
    pub fn new(params: Vec<(&'a str, &'a str)>) -> Self {
        Self { params }
    }

    pub fn param<'b, S>(&self, id: S) -> Option<&str>
    where
        S: Into<&'b str>,
    {
        let id = id.into();
        for (k, v) in self.params.iter() {
            if id == *k {
                return Some(v);
            }
        }
        None
    }
}
