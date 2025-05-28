use core::fmt;

#[derive(PartialEq, Debug)]
pub enum Object {
    Int(isize),
    Float(f32),
    String(String),
    Boolean(bool),
    Range(isize, isize),
    List(Vec<Object>),
    NoReturn,
}

impl Object {
    pub fn new<T: Into<Object>>(value: T) -> Self {
        value.into()
    }
}

impl From<Vec<Object>> for Object {
    fn from(value: Vec<Object>) -> Self {
        Object::List(value)
    }
}

impl From<(isize, isize)> for Object {
    fn from(value: (isize, isize)) -> Self {
        Object::Range(value.0, value.1)
    }
}

impl From<isize> for Object {
    fn from(value: isize) -> Self {
        Object::Int(value)
    }
}

impl From<f32> for Object {
    fn from(value: f32) -> Self {
        Object::Float(value)
    }
}

impl From<String> for Object {
    fn from(value: String) -> Self {
        Object::String(value)
    }
}

impl From<&str> for Object {
    fn from(value: &str) -> Self {
        Object::String(value.to_string())
    }
}

impl From<bool> for Object {
    fn from(value: bool) -> Self {
        Object::Boolean(value)
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Int(value) => write!(f, "{}", value),
            Object::Float(value) => write!(f, "{}", value),
            Object::String(value) => write!(f, "{}", value),
            Object::Boolean(value) => write!(f, "{}", value),
            Object::List(value) => {
                let objects = value
                    .iter()
                    .map(|val| format!("{}", val))
                    .collect::<Vec<String>>()
                    .join(";");
                write!(f, "{}", objects)
            }
            _ => write!(f, "Nothing"),
        }
    }
}
