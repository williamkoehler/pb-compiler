use std::fmt::{Formatter, Result};

use crate::compiler::ast::*;

#[inline(always)]
fn generate_hpp_field_reader(
    f: &mut Formatter<'_>,
    file: &File,
    depth: usize,
    indent: &mut String,
    reference: &str,
    field: &Field,
) -> Result {
    write!(f, "{indent}{{\n")?;

    {
        indent.push('\t');

        write!(f, "{indent}rapidjson::Value::ConstMemberIterator _temp_it_{depth} = _temp_{parent_depth}.FindMember(\"{field}\");\n", 
            parent_depth = (depth - 1),
            field = field.identifier(),
        )?;
        write!(
            f,
            "{indent}if (_temp_it_{depth} == _temp_{parent_depth}.MemberEnd()) return false;\n",
            parent_depth = (depth - 1),
        )?;

        if let Some(id) = field.reference().get_id() {
            if let Some(data_type) = file.data_type(id) {
                match data_type.kind() {
                    DataTypeKind::Boolean => {
                        write!(
                            f,
                            "{indent}if (!_temp_it_{depth}->value.IsBool()) return false;\n"
                        )?;
                        write!(
                            f,
                            "{indent}{reference} = _temp_it_{depth}->value.GetBool();\n"
                        )?;
                    }
                    DataTypeKind::Int8 => {
                        write!(
                            f,
                            "{indent}if (!_temp_it_{depth}->value.IsInt()) return false;\n"
                        )?;
                        write!(
                            f,
                            "{indent}{reference} = (int8_t)_temp_it_{depth}->value.GetInt();\n"
                        )?;
                    }
                    DataTypeKind::Int16 => {
                        write!(
                            f,
                            "{indent}if (!_temp_it_{depth}->value.IsInt()) return false;\n"
                        )?;
                        write!(
                            f,
                            "{indent}{reference} = (int16_t)_temp_it_{depth}->value.GetInt();\n"
                        )?;
                    }
                    DataTypeKind::Int32 => {
                        write!(
                            f,
                            "{indent}if (!_temp_it_{depth}->value.IsInt()) return false;\n"
                        )?;
                        write!(
                            f,
                            "{indent}{reference} = _temp_it_{depth}->value.GetInt();\n"
                        )?;
                    }
                    DataTypeKind::Int64 => {
                        write!(
                            f,
                            "{indent}if (!_temp_it_{depth}->value.IsInt64()) return false;\n"
                        )?;
                        write!(
                            f,
                            "{indent}{reference} = _temp_it_{depth}->value.GetInt64();\n"
                        )?;
                    }
                    DataTypeKind::UInt8 => {
                        write!(
                            f,
                            "{indent}if (!_temp_it_{depth}->value.IsUint()) return false;\n"
                        )?;
                        write!(
                            f,
                            "{indent}{reference} = (uint8_t)_temp_it_{depth}->value.GetUint();\n"
                        )?;
                    }
                    DataTypeKind::UInt16 => {
                        write!(
                            f,
                            "{indent}if (!_temp_it_{depth}->value.IsUint()) return false;\n"
                        )?;
                        write!(
                            f,
                            "{indent}{reference} = (uint16_t)_temp_it_{depth}->value.GetUint();\n"
                        )?;
                    }
                    DataTypeKind::UInt32 => {
                        write!(
                            f,
                            "{indent}if (!_temp_it_{depth}->value.IsUint()) return false;\n"
                        )?;
                        write!(
                            f,
                            "{indent}{reference} = _temp_it_{depth}->value.GetUint();\n"
                        )?;
                    }
                    DataTypeKind::UInt64 => {
                        write!(
                            f,
                            "{indent}if (!_temp_it_{depth}->value.IsUint64()) return false;\n"
                        )?;
                        write!(
                            f,
                            "{indent}{reference} = _temp_it_{depth}->value.GetUint64();\n"
                        )?;
                    }
                    DataTypeKind::Single => {
                        write!(
                            f,
                            "{indent}if (!_temp_it_{depth}->value.IsFloat()) return false;\n"
                        )?;
                        write!(
                            f,
                            "{indent}{reference} = _temp_it_{depth}->value.GetFloat();\n"
                        )?;
                    }
                    DataTypeKind::Double => {
                        write!(
                            f,
                            "{indent}if (!_temp_it_{depth}->value.IsDouble()) return false;\n"
                        )?;
                        write!(
                            f,
                            "{indent}{reference} = _temp_it_{depth}->value.GetDouble();\n"
                        )?;
                    }
                    DataTypeKind::String => {
                        write!(
                            f,
                            "{indent}if (!_temp_it_{depth}->value.IsString()) return false;\n"
                        )?;
                        write!(
                            f,
                            "{indent}{reference} = std::string_view(_temp_it_{depth}->value.GetString(), _temp_it_{depth}->value.GetStringLength());\n"
                        )?;
                    }
                    DataTypeKind::Structure(structure) => {
                        write!(
                            f,
                            "{indent}if (!_temp_it_{depth}->value.IsObject()) return false;\n"
                        )?;
                        write!(f, "{indent}const rapidjson::Value& _temp_{depth} = _temp_it_{depth}->value;\n")?;
                        generate_hpp_structure_reader(
                            f,
                            file,
                            depth + 1,
                            indent,
                            reference,
                            structure,
                        )?;
                    }
                    DataTypeKind::Variant(variant) => {
                        write!(
                            f,
                            "{indent}if (!_temp_it_{depth}->value.IsObject()) return false;\n"
                        )?;
                        write!(f, "{indent}const rapidjson::Value& _temp_{depth} = _temp_it_{depth}->value;\n")?;
                        generate_hpp_variant_reader(
                            f,
                            file,
                            depth + 1,
                            indent,
                            reference,
                            variant,
                        )?;
                    }
                    DataTypeKind::Alias(_) => {}
                }
            }
        }

        indent.pop();
    }

    write!(f, "{indent}}}\n")?;

    Ok(())
}

pub fn generate_hpp_structure_reader(
    f: &mut Formatter<'_>,
    file: &File,
    depth: usize,
    indent: &mut String,
    reference: &str,
    structure: &Structure,
) -> Result {
    for field in structure.fields() {
        let reference = format!("{reference}.{}", field.identifier());

        generate_hpp_field_reader(f, file, depth, indent, &reference, field)?;
    }

    Ok(())
}

pub fn generate_hpp_variant_reader(
    f: &mut Formatter<'_>,
    file: &File,
    depth: usize,
    indent: &mut String,
    reference: &str,
    variant: &Variant,
) -> Result {
    // Start of variant
    {
        write!(f, "{indent}rapidjson::Value::ConstMemberIterator _temp_kind_it_{depth} = _temp_{parent_depth}.FindMember(\"_kind\");\n", 
            parent_depth = (depth - 1)
        )?;
        write!(
            f,
            "{indent}if (_temp_kind_it_{depth} == _temp_{parent_depth}.MemberEnd() || !_temp_kind_it_{depth}->value.IsUint()) return false;\n",
            parent_depth = (depth - 1)
        )?;

        write!(f, "{indent}(&{reference})->~{}();\n", variant.identifier())?;

        let reference = format!("{reference}.kind");

        write!(
            f,
            "{indent}{reference} = ({}Kind)_temp_kind_it_{depth}->value.GetUint();\n",
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
                            write!(f, "{indent}{reference} = std::string_view();\n")?;
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

            generate_hpp_field_reader(f, file, depth, indent, &reference, field)?;
            write!(f, "{indent}break;\n")?;

            indent.pop();

            write!(f, "{indent}}}\n")?;
        }

        write!(f, "{indent}default: \n")?;
        write!(f, "{indent}\treturn false;\n")?;
    }

    // End of variant
    {
        write!(f, "{indent}}}\n")?;
    }

    Ok(())
}
