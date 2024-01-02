#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Value {
    inner: String,
    kind: ValueKind,
}

impl Value {
    pub fn is_kind_of(&self, kind: ValueKind) -> bool {
        self.kind == kind
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValueKind {
    Number,
    Literal,
}

impl From<u32> for Value {
    fn from(value: u32) -> Self {
        Value {
            inner: value.to_string(),
            kind: ValueKind::Number,
        }
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value {
            inner: value.to_string(),
            kind: ValueKind::Number,
        }
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value {
            inner: value.to_string(),
            kind: ValueKind::Literal,
        }
    }
}

impl TryFrom<Value> for f64 {
    type Error = String;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        value.inner.parse::<f64>().map_err(|e| e.to_string())
    }
}

impl TryFrom<&Value> for f64 {
    type Error = String;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        value.inner.parse::<f64>().map_err(|e| e.to_string())
    }
}

impl From<Value> for String {
    fn from(value: Value) -> Self {
        value.inner
    }
}

impl From<&Value> for String {
    fn from(value: &Value) -> Self {
        value.inner.to_owned()
    }
}

impl ToString for Value {
    fn to_string(&self) -> String {
        self.inner.clone()
    }
}
