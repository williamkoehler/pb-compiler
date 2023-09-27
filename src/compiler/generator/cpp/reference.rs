use crate::compiler::ast::*;

pub fn stringify_hpp_reference<'a>(file: &'a File, reference: &Reference) -> &'a str {
    if let Some(id) = reference.get_id() {
        if let Some(data_type) = file.data_type(id) {
            return match data_type.kind() {
                DataTypeKind::Boolean => "bool",
                DataTypeKind::Int8 => "int8_t",
                DataTypeKind::UInt8 => "uint8_t",
                DataTypeKind::Int16 => "int16_t",
                DataTypeKind::UInt16 => "uint16_t",
                DataTypeKind::Int32 => "int32_t",
                DataTypeKind::UInt32 => "uint32_t",
                DataTypeKind::Int64 => "int64_t",
                DataTypeKind::UInt64 => "uint64_t",
                DataTypeKind::Single => "float",
                DataTypeKind::Double => "double",
                DataTypeKind::String => "std::string_view",
                DataTypeKind::Alias(alias) => alias.identifier().get(),
                DataTypeKind::Structure(structure) => structure.identifier().get(),
                DataTypeKind::Variant(variant) => variant.identifier().get(),
            };
        }
    }

    return "error";
}