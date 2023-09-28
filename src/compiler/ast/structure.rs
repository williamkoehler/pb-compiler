use std::collections::HashMap;

pub struct Structure {
    identifier: super::Identifier,

    fields: Vec<super::Field>,
    options: HashMap<String, Vec<super::Expression>>,

    // Options
    structure_options: StructureOptions,

    min_size: usize,
}

impl Structure {
    pub fn new() -> Self {
        Self {
            identifier: super::Identifier::new(),

            fields: Vec::new(),
            options: HashMap::new(),

            structure_options: StructureOptions::default(),

            min_size: 0,
        }
    }

    pub fn structure_options(&self) -> &StructureOptions {
        &self.structure_options
    }

    pub fn structure_options_mut(&mut self) -> &mut StructureOptions {
        &mut self.structure_options
    }
}

impl super::Identified for Structure {
    #[inline]
    fn identifier(&self) -> &super::Identifier {
        &self.identifier
    }

    #[inline]
    fn identifier_mut(&mut self) -> &mut super::Identifier {
        &mut self.identifier
    }
}

impl super::Fielded for Structure {
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
        self.min_size
    }

    /// Update minimal in memory size
    #[inline]
    fn update_min_size(&mut self, size: usize) {
        self.min_size += size;
    }
}

impl super::Optioned for Structure {
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

#[derive(Default)]
pub struct StructureOptions {
    pub message_buffer: (bool, bool),
    pub json: (bool, bool),
}