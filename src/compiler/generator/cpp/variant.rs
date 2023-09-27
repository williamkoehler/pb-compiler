use std::fmt::{Formatter, Result};

use crate::compiler::ast::*;

pub fn generate_hpp_variant_declaration(f: &mut Formatter<'_>, variant: &Variant) -> Result {
    write!(f, "enum {}Kind : uint16_t;\n", variant.identifier())?;
    write!(f, "class {};\n", variant.identifier())?;

    Ok(())
}

pub fn generate_hpp_variant(
    f: &mut Formatter<'_>,
    file: &File,
    data_type: &DataType,
    variant: &Variant,
) -> Result {
    let mut indent = "\t".to_string();

    write!(f, "enum {}Kind : uint16_t\n", variant.identifier())?;
    write!(f, "{{\n")?;
    for (index, field) in variant.fields().iter().enumerate() {
        write!(
            f,
            "{indent}{} = {},\n",
            field.identifier().get_pascal_case(),
            index + 1,
        )?;
    }
    write!(f, "}};\n")?;

    write!(f, "class {}\n", variant.identifier())?;
    write!(f, "{{\n")?;

    // Private
    {
        write!(f, "private:\n")?;

        // Friends
        super::friends::generate_hpp_friends(f, file, &mut indent, data_type.max_rank())?;
        write!(f, "\n")?;

        // Fields
        {
            write!(f, "{indent}{}Kind kind;\n", variant.identifier())?;
            write!(f, "{indent}std::array<uint8_t, StaticMax<\n")?;

            for field in variant.fields() {
                write!(
                    f,
                    "{indent}\t\t{}, // {}\n",
                    super::reference::stringify_hpp_reference(file, field.reference()),
                    field.identifier()
                )?;
            }

            write!(f, "{indent}\t\tuint8_t>::value>\n")?;
            write!(f, "{indent}\tvalue;\n")?;
        }
    }

    // Public
    {
        write!(f, "public:\n")?;

        // Deconstructor
        {
            write!(f, "{indent}~{}()\n", variant.identifier())?;
            write!(f, "{indent}{{\n")?;

            {
                write!(f, "{indent}\tswitch(kind)\n")?;
                write!(f, "{indent}\t{{\n")?;

                {
                    for field in variant.fields() {
                        write!(
                            f,
                            "{indent}\tcase {}Kind::{}: \n",
                            variant.identifier(),
                            field.identifier().get_pascal_case()
                        )?;

                        {
                            if let Some(id) = field.reference().get_id() {
                                if let Some(data_type) = file.data_type(id) {
                                    match data_type.kind() {
                                        DataTypeKind::Structure(structure) => {
                                            write!(
                                                f,
                                                "{indent}\t\t(({data_type}*)this->value.data())->~{data_type}();\n",
                                                data_type = structure.identifier()
                                            )?;
                                        }
                                        DataTypeKind::Variant(variant) => {
                                            write!(
                                                f,
                                                "{indent}\t\t(({data_type}*)this->value.data())->~{data_type}();\n",
                                                data_type = variant.identifier()
                                            )?;
                                        }
                                        _ => {}
                                    }
                                }
                            }

                            write!(f, "{indent}\t\tbreak;\n")?;
                        }
                    }
                }

                write!(f, "{indent}\t}}\n")?;
            }

            write!(f, "{indent}}}\n")?;
        }

        // Getter and setter
        write!(
            f,
            "{indent}const {}Kind& Kind() const {{ return this->kind; }}\n",
            variant.identifier().get_pascal_case()
        )?;
        write!(f, "\n")?;

        for field in variant.fields() {
            write!(
                f,
                "{indent}const {data_type}& Get{field_pascal}() const {{ return *({data_type}*)this->value.data(); }}\n",
                field_pascal = field.identifier().get_pascal_case(),
                data_type = super::reference::stringify_hpp_reference(file, field.reference()),
            )?;

            write!(
                f,
                "{indent}{variant}& Set{field_pascal}(const {data_type}& value) {{ this->~{variant}(); this->kind = {variant}Kind::{field_pascal}; *({data_type}*)this->value.data() = value; return *this; }}\n",
                variant = variant.identifier(),
                field_pascal = field.identifier().get_pascal_case(),
                data_type = super::reference::stringify_hpp_reference(file, field.reference()),
            )?;

            write!(f, "\n")?;
        }

        // Message Buffer
        if let Some(opt) = variant.option("message_buffer") {
            let (enable_reader, enable_writer) = match opt.as_slice() {
                [Expression::Value(enable_reader), Expression::Value(enable_writer)] => {
                    (enable_reader.is_true(), enable_writer.is_true())
                }
                [Expression::Value(enable)] => (enable.is_true(), enable.is_true()),
                _ => (false, false),
            };
            if enable_reader {
                write!(f, "\n")?;
                super::message_buffer::generate_hpp_variant_reader(f, file, &mut indent, variant)?;
            }
            if enable_writer {
                write!(f, "\n")?;
                super::message_buffer::generate_hpp_variant_writer(f, file, &mut indent, variant)?;
            }
        }

        // Json
        if let Some(opt) = variant.option("json") {
            let (enable_reader, enable_writer) = match opt.as_slice() {
                [Expression::Value(enable_reader), Expression::Value(enable_writer)] => {
                    (enable_reader.is_true(), enable_writer.is_true())
                }
                [Expression::Value(enable)] => (enable.is_true(), enable.is_true()),
                _ => (false, false),
            };
            if enable_reader {
                write!(f, "\n")?;
                super::json::generate_hpp_variant_reader(f, file, &mut indent, variant)?;
            }
            if enable_writer {
                write!(f, "\n")?;
                super::json::generate_hpp_variant_writer(f, file, &mut indent, variant)?;
            }
        }

        // if variant.is_option_enabled_at("rapidjson", 0) {
        //     write!(f, "\n")?;
        //     super::rapidjson::generate_hpp_variant_reader(f, file, &mut indent, variant)?;
        // }
        // if variant.is_option_enabled_at("rapidjson", 1) {
        //     write!(f, "\n")?;
        //     super::rapidjson::generate_hpp_variant_writer(f, file, &mut indent, variant)?;
        // }
    }

    write!(f, "}};\n")?;

    Ok(())
}
