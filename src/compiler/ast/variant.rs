use std::collections::HashMap;

pub struct Variant {
    identifier: super::Identifier,

    fields: Vec<super::Field>,
    options: HashMap<String, Vec<super::Expression>>,

    min_size: usize,
}

impl Variant {
    pub fn new() -> Self {
        Self {
            identifier: super::Identifier::new(),

            fields: Vec::new(),
            options: HashMap::new(),

            min_size: 0,
        }
    }
}

impl super::Identified for Variant {
    #[inline]
    fn identifier(&self) -> &super::Identifier {
        &self.identifier
    }

    #[inline]
    fn identifier_mut(&mut self) -> &mut super::Identifier {
        &mut self.identifier
    }
}

impl super::Fielded for Variant {
    #[inline]
    fn fields(&self) -> &[super::Field] {
        self.fields.as_slice()
    }

    #[inline]
    fn add_field(&mut self, field: super::Field) {
        self.fields.push(field);
    }

    #[inline]
    fn field(&self, id: usize) -> Option<&super::Field> {
        self.fields.get(id)
    }

    #[inline]
    fn field_mut(&mut self, id: usize) -> Option<&mut super::Field> {
        self.fields.get_mut(id)
    }

    /// Get minimal in memory size
    #[inline]
    fn min_size(&self) -> usize {
        self.min_size + /* Kind */ 2
    }

    /// Update minimal in memory size
    #[inline]
    fn update_min_size(&mut self, size: usize) {
        self.min_size = std::cmp::max(self.min_size, size);
    }
}

impl super::Optioned for Variant {
    fn options(&self) -> &HashMap<String, Vec<super::Expression>> {
        &self.options
    }

    fn options_mut(&mut self) -> &mut HashMap<String, Vec<super::Expression>> {
        &mut self.options
    }

    fn add_option(&mut self, name: String, arguments: Vec<super::Expression>) -> bool {
        self.options.insert(name, arguments).is_none()
    }

    fn option(&self, name: &str) -> Option<&Vec<super::Expression>> {
        self.options.get(name)
    }

    fn option_mut(&mut self, name: &str) -> Option<&mut Vec<super::Expression>> {
        self.options.get_mut(name)
    }
}
