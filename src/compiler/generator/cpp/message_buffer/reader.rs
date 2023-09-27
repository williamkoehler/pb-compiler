use std::fmt::{Formatter, Result};

use crate::compiler::ast::*;

pub fn generate_hpp_field_reader(
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
                        "{indent}{reference} = *(bool*)_buffer; _buffer += sizeof(bool);\n"
                    )?;
                }
                DataTypeKind::Int8 => {
                    write!(
                        f,
                        "{indent}{reference} = *(int8_t*)_buffer; _buffer += sizeof(int8_t);\n"
                    )?;
                }
                DataTypeKind::Int16 => {
                    write!(
                        f,
                        "{indent}{reference} = *(int16_t*)_buffer; _buffer += sizeof(int16_t);\n"
                    )?;
                }
                DataTypeKind::Int32 => {
                    write!(
                        f,
                        "{indent}{reference} = *(int32_t*)_buffer; _buffer += sizeof(int32_t);\n"
                    )?;
                }
                DataTypeKind::Int64 => {
                    write!(
                        f,
                        "{indent}{reference} = *(int64_t*)_buffer; _buffer += sizeof(int64_t);\n"
                    )?;
                }
                DataTypeKind::UInt8 => {
                    write!(
                        f,
                        "{indent}{reference} = *(uint8_t*)_buffer; _buffer += sizeof(uint8_t);\n"
                    )?;
                }
                DataTypeKind::UInt16 => {
                    write!(
                        f,
                        "{indent}{reference} = *(uint16_t*)_buffer; _buffer += sizeof(uint16_t);\n"
                    )?;
                }
                DataTypeKind::UInt32 => {
                    write!(
                        f,
                        "{indent}{reference} = *(uint32_t*)_buffer; _buffer += sizeof(uint32_t);\n"
                    )?;
                }
                DataTypeKind::UInt64 => {
                    write!(
                        f,
                        "{indent}{reference} = *(uint64_t*)_buffer; _buffer += sizeof(uint64_t);\n"
                    )?;
                }
                DataTypeKind::Single => {
                    write!(
                        f,
                        "{indent}{reference} = *(float*)_buffer; _buffer += sizeof(float);\n"
                    )?;
                }
                DataTypeKind::Double => {
                    write!(
                        f,
                        "{indent}{reference} = *(double*)_buffer; _buffer += sizeof(double);\n"
                    )?;
                }
                DataTypeKind::String => {
                    write!(f, "{indent}{{\n")?;

                    write!(f, "{indent}\tuint16_t _data_size = *(uint16_t*)_buffer; _buffer += sizeof(uint16_t);\n",)?;
                    write!(
                        f,
                        "{indent}\tuint8_t* _data = _buffer; _buffer += _data_size;\n",
                    )?;
                    write!(
                        f,
                        "{indent}\t{reference} = std::string_view((const char*)_data, (size_t)_data_size);\n"
                    )?;

                    write!(f, "{indent}}}\n")?;
                }
                DataTypeKind::Structure(structure) => {
                    generate_hpp_structure_reader(f, file, indent, reference, structure)?;
                }
                DataTypeKind::Variant(variant) => {
                    generate_hpp_variant_reader(f, file, indent, reference, variant)?;
                }
                DataTypeKind::Alias(_) => {}
            }
        }
    }

    Ok(())
}

pub fn generate_hpp_structure_reader(
    f: &mut Formatter<'_>,
    file: &File,
    indent: &mut String,
    reference: &str,
    structure: &Structure,
) -> Result {
    write!(f, "{indent}{{\n")?;

    {
        indent.push('\t');

        for field in structure.fields() {
            let reference = format!("{reference}.{}", field.identifier());

            generate_hpp_field_reader(f, file, indent, &reference, field)?;
        }

        indent.pop();
    }

    write!(f, "{indent}}}\n")?;

    Ok(())
}

pub fn generate_hpp_variant_reader(
    f: &mut Formatter<'_>,
    file: &File,
    indent: &mut String,
    reference: &str,
    variant: &Variant,
) -> Result {
    // Start of variant
    {
        write!(f, "{indent}(&{reference})->~{}();\n", variant.identifier())?;

        let reference = format!("{}.{}", reference, "kind");

        write!(
            f,
            "{indent}{reference} = ({}Kind)*(uint16_t*)_buffer; _buffer += sizeof(uint16_t);\n",
            variant.identifier()
        )?;

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

            indent.push('\t');

            if let Some(id) = field.reference().get_id() {
                if let Some(data_type) = file.data_type(id) {
                    match data_type.kind() {
                        DataTypeKind::String => {
                            write!(
                                f,
                                "{indent}{reference} = std::string_view();\n"
                            )?;
                        }
                        DataTypeKind::Structure(structure) => {
                            write!(
                                f,
                                "{indent}{reference} = {}();\n",
                                structure.identifier()
                            )?;
                        }
                        DataTypeKind::Variant(variant) => {
                            write!(
                                f,
                                "{indent}{reference} = {}();\n",
                                variant.identifier()
                            )?;
                        }
                        _ => {}
                    }
                }
            }

            generate_hpp_field_reader(f, file, indent, &reference, field)?;
            write!(f, "{indent}break;\n")?;

            indent.pop();

            write!(f, "{indent}}}\n")?;
        }
    }

    // End of variant
    {
        write!(f, "{indent}}}\n")?;
    }

    Ok(())
}
