const SEPARATOR: char = '=';

#[derive(PartialEq, Debug)]
pub struct Line {
    pub key: String,
    pub value: String,
}

impl Line {
    pub fn new(key: String, value: String) -> Line {
        Line { key, value }
    }

    pub fn get_key(&self) -> &String {
        &self.key
    }

    pub fn get_value(&self) -> &String {
        &self.value
    }
}

impl From<String> for Line {
    fn from(s: String) -> Self {
        let (key, value) = match s.split_once(SEPARATOR) {
            Some(parts) => parts,
            None => panic!("Invalid line: {}", s),
        };

        Line {
            key: key.to_string(),
            value: value.to_string(),
        }
    }
}

impl From<&str> for Line {
    fn from(s: &str) -> Self {
        Line::into(s.to_string().into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_into() {
        let line: Line = "key=value".into();
        assert_eq!(line, Line::new("key".into(), "value".into()));
    }

    #[test]
    fn test_line_into_more_equals() {
        let line: Line = "key=value=value1=value2".into();
        assert_eq!(line, Line::new("key".into(), "value=value1=value2".into()));
    }

    #[test]
    #[should_panic(expected = "Invalid line: key!value")]
    fn test_line_into_fails() {
        let _line: Line = "key!value".into();
    }
}
