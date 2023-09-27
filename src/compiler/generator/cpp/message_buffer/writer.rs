use std::fmt::{Formatter, Result};

use crate::compiler::ast::*;

fn generate_hpp_field_size_offset(
    f: &mut Formatter<'_>,
    file: &File,
    indent: &mut String,
    reference: &str,
    field: &Field,
) -> Result {
    if let Some(id) = field.reference().get_id() {
        if let Some(data_type) = file.data_type(id) {
            match data_type.kind() {
                DataTypeKind::String => {
                    write!(f, "{indent}_size_offset += {reference}.size();\n")?;
                }
                DataTypeKind::Structure(structure) => {
                    generate_hpp_structure_size_offset(f, file, indent, &reference, structure)?;
                }
                DataTypeKind::Variant(variant) => {
                    generate_hpp_variant_size_offset(f, file, indent, &reference, variant)?;
                }
                _ => {}
            }
        }
    }

    Ok(())
}

fn generate_hpp_field_writer(
    f: &mut Formatter<'_>,
    file: &File,
    indent: &mut String,
    reference: &str,
    field: &Field,
) -> Result {
    if let Some(id) = field.reference().get_id() {
        if let Some(data_type) = file.data_type(id) {
            match data_type.kind() {
                DataTypeKind::Boolean => {
                    write!(
                        f,
                        "{indent}*(bool*)_buffer = {reference}; _buffer += sizeof(bool);\n"
                    )?;
                }
                DataTypeKind::Int8 => {
                    write!(
                        f,
                        "{indent}*(int8_t*)_buffer = {reference}; _buffer += sizeof(int8_t);\n"
                    )?;
                }
                DataTypeKind::Int16 => {
                    write!(
                        f,
                        "{indent}*(int16_t*)_buffer = {reference}; _buffer += sizeof(int16_t);\n"
                    )?;
                }
                DataTypeKind::Int32 => {
                    write!(
                        f,
                        "{indent}*(int32_t*)_buffer = {reference}; _buffer += sizeof(int32_t);\n"
                    )?;
                }
                DataTypeKind::Int64 => {
                    write!(
                        f,
                        "{indent}*(int64_t*)_buffer = {reference}; _buffer += sizeof(int64_t);\n"
                    )?;
                }
                DataTypeKind::UInt8 => {
                    write!(
                        f,
                        "{indent}*(uint8_t*)_buffer = {reference}; _buffer += sizeof(uint8_t);\n"
                    )?;
                }
                DataTypeKind::UInt16 => {
                    write!(
                        f,
                        "{indent}*(uint16_t*)_buffer = {reference}; _buffer += sizeof(uint16_t);\n"
                    )?;
                }
                DataTypeKind::UInt32 => {
                    write!(
                        f,
                        "{indent}*(uint32_t*)_buffer = {reference}; _buffer += sizeof(uint32_t);\n"
                    )?;
                }
                DataTypeKind::UInt64 => {
                    write!(
                        f,
                        "{indent}*(uint64_t*)_buffer = {reference}; _buffer += sizeof(uint64_t);\n"
                    )?;
                }
                DataTypeKind::Single => {
                    write!(
                        f,
                        "{indent}*(float*)_buffer = {reference}; _buffer += sizeof(float);\n"
                    )?;
                }
                DataTypeKind::Double => {
                    write!(
                        f,
                        "{indent}*(double*)_buffer = {reference}; _buffer += sizeof(double);\n"
                    )?;
                }
                DataTypeKind::String => {
                    write!(
                        f,
                        "{indent}*(uint16_t*)_buffer = {reference}.size(); _buffer += sizeof(uint16_t);\n"
                    )?;
                    write!(
                        f,
                        "{indent}std::memcpy((void*)_buffer, (void*){reference}.data(), {reference}.size()); _buffer += {reference}.size();\n"
                    )?;
                }
                DataTypeKind::Structure(structure) => {
                    generate_hpp_structure_writer(f, file, indent, &reference, structure)?;
                }
                DataTypeKind::Variant(variant) => {
                    generate_hpp_variant_writer(f, file, indent, &reference, variant)?;
                }
                DataTypeKind::Alias(_) => {}
            }
        }
    }

    Ok(())
}

pub fn generate_hpp_structure_size_offset(
    f: &mut Formatter<'_>,
    file: &File,
    indent: &mut String,
    reference: &str,
    structure: &Structure,
) -> Result {
    for field in structure.fields() {
        let reference = format!("{reference}.{}", field.identifier());

        generate_hpp_field_size_offset(f, file, indent, &reference, field)?;
    }

    Ok(())
}

pub fn generate_hpp_structure_writer(
    f: &mut Formatter<'_>,
    file: &File,
    indent: &mut String,
    reference: &str,
    structure: &Structure,
) -> Result {
    write!(f, "{indent}{{\n")?;

    indent.push('\t');

    for field in structure.fields() {
        let reference = format!("{reference}.{}", field.identifier());

        generate_hpp_field_writer(f, file, indent, &reference, field)?;
    }

    indent.pop();

    write!(f, "{indent}}}\n")?;

    Ok(())
}

pub fn generate_hpp_variant_size_offset(
    f: &mut Formatter<'_>,
    file: &File,
    indent: &mut String,
    reference: &str,
    variant: &Variant,
) -> Result {
    write!(f, "{indent}_size_offset += sizeof(uint16_t);\n")?;

    // Start of switch
    {
        let reference = format!("{reference}.kind");

        write!(f, "{indent}switch({reference})\n")?;
        write!(f, "{indent}{{\n")?;
    }

    // Switch cases
    {
        for field in variant.fields() {
            let reference = format!(
                "(*({}*){reference}.value.data())",
                super::super::reference::stringify_hpp_reference(file, field.reference())
            );

            write!(
                f,
                "{indent}case {}Kind::{}: \n",
                variant.identifier(),
                field.identifier().get_pascal_case()
            )?;
            write!(f, "{indent}{{\n")?;

            {
                indent.push('\t');

                generate_hpp_field_size_offset(f, file, indent, &reference, field)?;
                write!(f, "{indent}break;\n")?;

                indent.pop();
            }

            write!(f, "{indent}}}\n")?;
        }
    }

    // End of switch
    {
        write!(f, "{indent}}}\n")?;
    }

    Ok(())
}

pub fn generate_hpp_variant_writer(
    f: &mut Formatter<'_>,
    file: &File,
    indent: &mut String,
    reference: &str,
    variant: &Variant,
) -> Result {
    // Start of variant
    {
        let reference = format!("{reference}.kind");

        write!(f, "{indent}switch({reference})\n")?;
        write!(f, "{indent}{{\n")?;
    }

    // Variant cases
    {
        for field in variant.fields() {
            let reference = format!(
                "(*({}*){reference}.value.data())",
                super::super::reference::stringify_hpp_reference(file, field.reference())
            );

            write!(
                f,
                "{indent}case {}Kind::{}: \n",
                variant.identifier(),
                field.identifier().get_pascal_case()
            )?;
            write!(f, "{indent}{{\n")?;

            {
                indent.push('\t');

                generate_hpp_field_writer(f, file, indent, &reference, field)?;
                write!(f, "{indent}break;\n")?;

                indent.pop();
            }

            write!(f, "{indent}}}\n")?;
        }
    }

    // End of variant
    {
        write!(f, "{indent}}}\n")?;
    }

    Ok(())
}
