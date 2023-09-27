use std::fmt::{Formatter, Result};

use crate::compiler::ast::*;

fn generate_hpp_field_writer(
    f: &mut Formatter<'_>,
    file: &File,
    depth: usize,
    indent: &mut String,
    reference: &str,
    field: &Field,
) -> Result {
    if let Some(id) = field.reference().get_id() {
        if let Some(data_type) = file.data_type(id) {
            match data_type.kind() {
                DataTypeKind::Boolean |
                DataTypeKind::Int8 |
                DataTypeKind::Int16 |
                DataTypeKind::Int32 |
                DataTypeKind::Int64 |
                DataTypeKind::UInt8 |
                DataTypeKind::UInt16 |
                DataTypeKind::UInt32 |
                DataTypeKind::UInt64 |
                DataTypeKind::Single |
                DataTypeKind::Double => {
                    write!(
                        f, 
                        "{indent}_temp_{parent_depth}.AddMember(\"{field}\", rapidjson::Value({reference}), _allocator);\n", 
                        parent_depth = depth - 1, 
                        field = field.identifier()
                    )?;
                }
                DataTypeKind::String => {
                    write!(
                        f, 
                        "{indent}_temp_{parent_depth}.AddMember(\"{field}\", rapidjson::Value({reference}.data(), {reference}.size(), _allocator), _allocator);\n", 
                        parent_depth = depth - 1, 
                        field = field.identifier(),
                    )?;
                }
                DataTypeKind::Structure(structure) => {
                    write!(f, "{indent}{{\n")?;
                
                    {
                        indent.push('\t');
                    
                        write!(f, "{indent}rapidjson::Value _temp_{depth} = rapidjson::Value(rapidjson::kObjectType);\n")?;

                        generate_hpp_structure_writer(
                            f,
                            file,
                            depth + 1,
                            indent,
                            reference,
                            structure,
                        )?;

                        write!(
                            f, 
                            "{indent}_temp_{parent_depth}.AddMember(\"{field}\", _temp_{depth}, _allocator);\n", 
                            parent_depth = depth - 1, 
                            field = field.identifier()
                        )?;

                        indent.pop();
                    }
                    
                    write!(f, "{indent}}}\n")?;
                }
                DataTypeKind::Variant(variant) => {
                    write!(f, "{indent}{{\n")?;
                
                    {
                        indent.push('\t');
                    
                        write!(f, "{indent}rapidjson::Value _temp_{depth} = rapidjson::Value(rapidjson::kObjectType);\n")?;

                        generate_hpp_variant_writer(f, file, depth + 1, indent, reference, variant)?;

                        write!(
                            f, 
                            "{indent}_temp_{parent_depth}.AddMember(\"{field}\", _temp_{depth}, _allocator);\n", 
                            parent_depth = depth - 1, 
                            field = field.identifier()
                        )?;

                        indent.pop();
                    }
                    
                    write!(f, "{indent}}}\n")?;
                }
                DataTypeKind::Alias(_) => {}
            }
        }
    }

    Ok(())
}

pub fn generate_hpp_structure_writer(
    f: &mut Formatter<'_>,
    file: &File,
    depth: usize,
    indent: &mut String,
    reference: &str,
    structure: &Structure,
) -> Result {
    write!(f, "{indent}{{\n")?;

    indent.push('\t');

    for field in structure.fields() {
        let reference = format!("{reference}.{}", field.identifier());

        generate_hpp_field_writer(f, file, depth, indent, &reference, field)?;
    }

    indent.pop();

    write!(f, "{indent}}}\n")?;

    Ok(())
}

pub fn generate_hpp_variant_writer(
    f: &mut Formatter<'_>,
    file: &File,
    depth: usize,
    indent: &mut String,
    reference: &str,
    variant: &Variant,
) -> Result {
    // Start of variant
    {
        let reference = format!("{reference}.kind");        

        write!(
            f, 
            "{indent}_temp_{parent_depth}.AddMember(\"_kind\", rapidjson::Value({reference}), _allocator);\n", 
            parent_depth = depth - 1,
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
                "{indent}case {}Kind::{}:\n",
                variant.identifier(),
                field.identifier().get_pascal_case()
            )?;
            write!(f, "{indent}{{\n")?;

            {
                indent.push('\t');

                generate_hpp_field_writer(f, file, depth, indent, &reference, field)?;
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
