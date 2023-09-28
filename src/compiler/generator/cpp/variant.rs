use std::fmt::{Formatter, Result};

use crate::compiler::ast::*;

pub fn generate_hpp_variant_declaration(
    f: &mut Formatter<'_>,
    indent: &mut String,
    variant: &Variant,
) -> Result {
    write!(f, "{indent}enum {}Kind : uint16_t;\n", variant.identifier())?;
    write!(f, "{indent}class {};\n", variant.identifier())?;

    Ok(())
}

pub fn generate_hpp_variant(
    f: &mut Formatter<'_>,
    file: &File,
    indent: &mut String,
    data_type: &DataType,
    variant: &Variant,
) -> Result {
    write!(f, "{indent}enum {}Kind : uint16_t\n", variant.identifier())?;
    write!(f, "{indent}{{\n")?;
    for (index, field) in variant.fields().iter().enumerate() {
        write!(
            f,
            "{indent}\t{} = {},\n",
            field.identifier().get_pascal_case(),
            index + 1,
        )?;
    }
    write!(f, "{indent}}};\n")?;

    write!(f, "{indent}class {}\n", variant.identifier())?;
    write!(f, "{indent}{{\n")?;

    // Private
    {
        write!(f, "{indent}private:\n")?;

        indent.push('\t');

        // Friends
        super::friends::generate_hpp_friends(f, file, indent, data_type.max_rank())?;
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

        indent.pop();
    }

    // Public
    {
        write!(f, "{indent}public:\n")?;

        indent.push('\t');

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
        if variant.variant_options().message_buffer.0 {
            write!(f, "\n")?;
            super::message_buffer::generate_hpp_variant_reader(f, file, indent, variant)?;
        }
        if variant.variant_options().message_buffer.1 {
            write!(f, "\n")?;
            super::message_buffer::generate_hpp_variant_writer(f, file, indent, variant)?;
        }

        // Json
        if variant.variant_options().json.0 {
            write!(f, "\n")?;
            super::json::generate_hpp_variant_reader(f, file, indent, variant)?;
        }
        if variant.variant_options().json.1 {
            write!(f, "\n")?;
            super::json::generate_hpp_variant_writer(f, file, indent, variant)?;
        }

        indent.pop();
    }

    write!(f, "{indent}}};\n")?;

    Ok(())
}
