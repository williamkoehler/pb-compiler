use super::Fielded;
use super::Identified;

pub struct DataType {
    identifier: super::Identifier,
    kind: DataTypeKind,

    // Additional informations about the data type
    max_rank: usize,
}

impl DataType {
    pub fn from(kind: DataTypeKind) -> Self {
        Self {
            identifier: match &kind {
                DataTypeKind::Boolean => super::Identifier::from_str("bool"),
                DataTypeKind::Int8 => super::Identifier::from_str("int8"),
                DataTypeKind::UInt8 => super::Identifier::from_str("uint8"),
                DataTypeKind::Int16 => super::Identifier::from_str("int16"),
                DataTypeKind::UInt16 => super::Identifier::from_str("uint16"),
                DataTypeKind::Int32 => super::Identifier::from_str("int32"),
                DataTypeKind::UInt32 => super::Identifier::from_str("uint32"),
                DataTypeKind::Int64 => super::Identifier::from_str("int64"),
                DataTypeKind::UInt64 => super::Identifier::from_str("uint64"),
                DataTypeKind::Single => super::Identifier::from_str("single"),
                DataTypeKind::Double => super::Identifier::from_str("double"),
                DataTypeKind::String => super::Identifier::from_str("string"),
                DataTypeKind::Alias(alias) => alias.identifier().clone(),
                DataTypeKind::Structure(structure) => structure.identifier().clone(),
                DataTypeKind::Variant(variant) => variant.identifier().clone(),
            },
            kind,

            max_rank: 0,
        }
    }

    #[inline]
    pub fn kind(&self) -> &DataTypeKind {
        &self.kind
    }

    #[inline]
    pub fn kind_mut(&mut self) -> &mut DataTypeKind {
        &mut self.kind
    }

    pub fn size(&self) -> usize {
        match &self.kind {
            DataTypeKind::Boolean => 1,
            DataTypeKind::Int8 => 1,
            DataTypeKind::Int16 => 2,
            DataTypeKind::Int32 => 4,
            DataTypeKind::Int64 => 8,
            DataTypeKind::UInt8 => 1,
            DataTypeKind::UInt16 => 2,
            DataTypeKind::UInt32 => 4,
            DataTypeKind::UInt64 => 8,
            DataTypeKind::Single => 4,
            DataTypeKind::Double => 8,
            DataTypeKind::String => 2,
            DataTypeKind::Alias(_) => 0,
            DataTypeKind::Structure(structure) => structure.min_size(),
            DataTypeKind::Variant(variant) => variant.min_size(),
        }
    }

    #[inline]
    pub fn max_rank(&self) -> usize {
        self.max_rank
    }

    #[inline]
    pub fn update_max_rank(&mut self, max_rank: usize) {
        self.max_rank = std::cmp::max(self.max_rank, max_rank);
    }
}

impl super::Identified for DataType {
    #[inline]
    fn identifier(&self) -> &super::Identifier {
        &self.identifier
    }

    #[inline]
    fn identifier_mut(&mut self) -> &mut super::Identifier {
        &mut self.identifier
    }
}

pub enum DataTypeKind {
    Boolean,
    Int8,
    Int16,
    Int32,
    Int64,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Single,
    Double,
    String,

    Alias(super::Alias),
    Structure(super::Structure),
    Variant(super::Variant),
}
