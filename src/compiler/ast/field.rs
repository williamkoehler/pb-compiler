pub struct Field {
    identifier: super::Identifier,
    reference: super::Reference,
}

impl Field {
    pub fn new() -> Self {
        Self {
            identifier: super::Identifier::new(),
            reference: super::Reference::new(),
        }
    }

    #[inline]
    pub fn identifier(&self) -> &super::Identifier {
        &self.identifier
    }

    #[inline]
    pub fn identifier_mut(&mut self) -> &mut super::Identifier {
        &mut self.identifier
    }

    #[inline]
    pub fn reference(&self) -> &super::Reference {
        &self.reference
    }

    #[inline]
    pub fn reference_mut(&mut self) -> &mut super::Reference {
        &mut self.reference
    }
}

// That contains fields
pub trait Fielded {
    fn fields(&self) -> &[Field];

    fn add_field(&mut self, field: Field);

    fn field(&self, id: usize) -> Option<&Field>;

    fn field_mut(&mut self, id: usize) -> Option<&mut Field>;

    /// Returns the minimal data type size.
    fn min_size(&self) -> usize;

    /// Update minimal data structure size
    fn update_min_size(&mut self, size: usize);
}