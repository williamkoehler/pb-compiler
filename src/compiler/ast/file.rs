use std::collections::HashMap;

pub struct File {
    name: String,

    data_types: Vec<super::DataType>,
    options: HashMap<String, Vec<super::Expression>>,
}

impl File {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            data_types: Vec::new(),
            options: HashMap::new(),
        }
    }

    #[inline]
    pub fn data_types(&self) -> &[super::DataType] {
        self.data_types.as_slice()
    }

    #[inline]
    pub fn data_types_mut(&mut self) -> &mut [super::DataType] {
        self.data_types.as_mut_slice()
    }

    pub fn add_data_type(&mut self, data_type: super::DataType) {
        self.data_types.push(data_type);
    }
    pub fn add_structure(&mut self, structure: super::Structure) {
        self.data_types
            .push(super::DataType::from(super::DataTypeKind::Structure(
                structure,
            )));
    }
    pub fn add_variant(&mut self, variant: super::Variant) {
        self.data_types
            .push(super::DataType::from(super::DataTypeKind::Variant(variant)));
    }
    pub fn add_alias(&mut self, alias: super::Alias) {
        self.data_types
            .push(super::DataType::from(super::DataTypeKind::Alias(alias)));
    }

    pub fn data_type(&self, id: usize) -> Option<&super::DataType> {
        self.data_types.get(id)
    }

    pub fn data_type_mut(&mut self, id: usize) -> Option<&mut super::DataType> {
        self.data_types.get_mut(id)
    }
}

impl super::Optioned for File {
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
