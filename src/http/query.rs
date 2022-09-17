use std::collections::HashMap;

#[derive(Debug)]
pub struct Query<'a> {
    data: HashMap<&'a str, Value<'a>>,
}

#[derive(Debug)]
pub enum Value<'a> {
    Single(&'a str),
    Multiple(Vec<&'a str>),
}

impl<'a> Query<'a> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'a> From<&'a str> for Query<'a> {
    fn from(s: &'a str) -> Self {
        let mut data = HashMap::new();
        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut value = "";
            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                value = &sub_str[i + 1..];
            }
            data.entry(key)
                .and_modify(|existing| match existing {
                    Value::Single(prev_value) => {
                        *existing = Value::Multiple(vec![prev_value, value]);
                    }
                    Value::Multiple(vec) => vec.push(value),
                })
                .or_insert(Value::Single(value));
        }
        Query { data }
    }
}
