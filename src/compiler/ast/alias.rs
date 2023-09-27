pub struct Alias {
    identifier: super::Identifier,
    reference: super::Reference,
}

impl Alias {
    /// Creates a new empty [`Alias`].
    pub fn new() -> Self {
        Self {
            identifier: super::Identifier::new(),
            reference: super::Reference::new(),
        }
    }

    /// Creates a new [`Alias`]
    pub fn from(identifier: String, reference: String) -> Self {
        Self {
            identifier: super::Identifier::from(identifier),
            reference: super::Reference::from(reference),
        }
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

impl super::Identified for Alias {
    #[inline]
    fn identifier(&self) -> &super::Identifier {
        &self.identifier
    }

    #[inline]
    fn identifier_mut(&mut self) -> &mut super::Identifier {
        &mut self.identifier
    }
}