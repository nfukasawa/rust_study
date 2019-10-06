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
                Component::Wildcard => {
                    if i == l1 - 1 {
                        return (true, Some(params));
                    }
                }
            }
            i += 1;
        }

        if params.len() > 0 {
            (true, Some(params))
        } else {
            (true, None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Path;
    use std::collections::HashMap;
    macro_rules! params {
        ( $( $t:expr),* ) => {
            {
                let mut temp_hash = HashMap::new();
                $(
                    temp_hash.insert(format!(":{}", $t.0.to_string()), $t.1.to_string());
                )*
                temp_hash
            }
        };
    }

    #[test]
    fn test_path() {
        let cases = vec![
            ("/foo/bar", "/foo/bar", true, None),
            ("/foo/bar", "/foo/bar/", false, None),
            ("/foo/:id", "/foo/1234", true, Some(params![("id", "1234")])),
            ("/foo/:id", "/foo/1234/", false, None),
            (
                "/foo/bar/:id/*",
                "/foo/bar/fizz/buzz/hoge/fuga",
                true,
                Some(params![("id", "fizz")]),
            ),
            (
                "/foo/bar/:id/*/:id2/*",
                "/foo/bar/fizz/buzz/hoge/fuga",
                true,
                Some(params![("id", "fizz"), ("id2", "hoge")]),
            ),
        ];

        for (route, path, ok, params) in cases.iter() {
            let p = Path::new(route);
            let (res_ok, res_params) = p.matches(path);
            assert_eq!(*ok, res_ok);
            assert_eq!(*params, res_params);
        }
    }

}
