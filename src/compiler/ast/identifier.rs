use convert_case::{Case, Casing};

#[derive(Clone)]
pub struct Identifier {
    value: Option<String>,
}

impl Identifier {
    pub fn new() -> Self {
        Self { value: None }
    }

    pub fn from(identifier: String) -> Self {
        Self {
            value: Some(identifier.to_string()),
        }
    }

    pub fn from_str(identifier: &str) -> Self {
        Self {
            value: Some(identifier.to_string()),
        }
    }

    #[inline]
    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    #[inline]
    pub fn get(&self) -> &str {
        match &self.value {
            Some(name) => name,
            None => "no identifier",
        }
    }

    pub fn get_camel_case(&self) -> String {
        self.get().to_case(Case::Camel)
    }

    pub fn get_pascal_case(&self) -> String {
        self.get().to_case(Case::Pascal)
    }

    #[inline]
    pub fn get_opt(&self) -> Option<&String> {
        self.value.as_ref()
    }

    #[inline]
    pub fn set(&mut self, name: String) {
        self.value = Some(name);
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get())
    }
}

pub trait Identified {
    fn identifier(&self) -> &super::Identifier;
    fn identifier_mut(&mut self) -> &mut super::Identifier;
}