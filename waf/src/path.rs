use std::collections::HashMap;

#[derive(Debug)]
enum Component {
    Component(String),
    Param(String),
    Wildcard,
}

pub struct Path {
    components: Vec<Component>,
}

impl Path {
    pub fn new(path: &str) -> Self {
        let cmps = path
            .split("/")
            .map(|cmp| match cmp {
                "*" => Component::Wildcard,
                cmp if cmp.as_bytes().first() == Some(&b':') => Component::Param(cmp.to_string()),
                cmp => Component::Component(cmp.to_string()),
            })
            .collect();
        Self { components: cmps }
    }

    pub fn matches(&self, path: &str) -> (bool, Option<HashMap<String, String>>) {
        let cmps: Vec<&str> = path.split("/").collect();
        let l1 = self.components.len();
        let l2 = cmps.len();

        if l1 > l2 {
            return (false, None);
        }
        if l1 != l2 {
            match self.components.last() {
                Some(Component::Wildcard) => (),
                _ => return (false, None),
            }
        }

        let mut params = HashMap::new();
        let mut i = 0;
        while i < l1 {
            match &self.components[i] {
                Component::Component(cmp) => {
                    if cmp != cmps[i] {
                        return (false, None);
                    }
                }
                Component::Param(cmp) => {
                    params.insert(cmp.to_string(), cmps[i].to_string());
                }
                Component::Wildcard => return (true, Some(params)),
            }
            i += 1;
        }

        (true, Some(params))
    }
}
