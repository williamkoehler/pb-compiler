#[derive(Clone)]
pub struct Reference {
    value: Option<String>,
    id: Option<usize>,
}

impl Reference {
    pub fn new() -> Self {
        Self {
            value: None,
            id: None,
        }
    }

    pub fn from(reference: String) -> Self {
        Self {
            value: Some(reference),
            id: None,
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
            None => "no reference",
        }
    }

    #[inline]
    pub fn get_opt(&self) -> Option<&String> {
        self.value.as_ref()
    }

    #[inline]
    pub fn set(&mut self, name: String) {
        self.value = Some(name);
    }

    #[inline]
    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    #[inline]
    pub fn get_id(&self) -> Option<usize> {
        self.id
    }

    #[inline]
    pub fn set_id(&mut self, id: usize) {
        self.id = Some(id);
    }
}

impl std::fmt::Display for Reference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get())
    }
}
