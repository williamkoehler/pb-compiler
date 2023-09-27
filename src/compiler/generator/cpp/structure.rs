use std::fmt::{Formatter, Result};

use crate::compiler::ast::*;

pub fn generate_hpp_structure_declaration(
    f: &mut Formatter<'_>,
    indent: &mut String,
    structure: &Structure,
) -> Result {
    write!(f, "{indent}class {};\n", structure.identifier())?;

    Ok(())
}

pub fn generate_hpp_structure(
    f: &mut Formatter<'_>,
    file: &File,
    indent: &mut String,
    data_type: &DataType,
    structure: &Structure,
) -> Result {
    write!(f, "{indent}class {}\n", structure.identifier())?;
    write!(f, "{indent}{{\n")?;

    // Private
    {
        write!(f, "{indent}private:\n")?;

        indent.push('\t');

        // Friends
        super::friends::generate_hpp_friends(f, file, indent, data_type.max_rank())?;
        write!(f, "\n")?;

        // Fields
        for field in structure.fields() {
            write!(
                f,
                "{indent}{} {};\n",
                super::reference::stringify_hpp_reference(file, field.reference()),
                field.identifier()
            )?;
        }
        write!(f, "\n")?;

        indent.pop();
    }

    // Public
    {
        write!(f, "{indent}public:\n")?;

        indent.push('\t');

        // Getter and setter
        for field in structure.fields() {
            write!(
                f,
                "{indent}const {}& Get{}() const {{ return this->{}; }}\n",
                super::reference::stringify_hpp_reference(file, field.reference()),
                field.identifier().get_pascal_case(),
                field.identifier(),
            )?;

            write!(
                f,
                "{indent}{}& Set{field_pascal}(const {}& {field}) {{ this->{field} = {field}; return *this; }}\n",
                structure.identifier(),
                super::reference::stringify_hpp_reference(file, field.reference()),
                field = field.identifier(),
                field_pascal = field.identifier().get_pascal_case(),
            )?;

            write!(f, "\n")?;
        }

        // Message Buffer
        if let Some(opt) = structure.option("message_buffer") {
            let (enable_reader, enable_writer) = match opt.as_slice() {
                [Expression::Value(enable_reader), Expression::Value(enable_writer)] => {
                    (enable_reader.is_true(), enable_writer.is_true())
                }
                [Expression::Value(enable)] => (enable.is_true(), enable.is_true()),
                _ => (false, false),
            };
            if enable_reader {
                write!(f, "\n")?;
                super::message_buffer::generate_hpp_structure_reader(f, file, indent, structure)?;
            }
            if enable_writer {
                write!(f, "\n")?;
                super::message_buffer::generate_hpp_structure_writer(f, file, indent, structure)?;
            }
        }

        // Json
        if let Some(opt) = structure.option("json") {
            let (enable_reader, enable_writer) = match opt.as_slice() {
                [Expression::Value(enable_reader), Expression::Value(enable_writer)] => {
                    (enable_reader.is_true(), enable_writer.is_true())
                }
                [Expression::Value(enable)] => (enable.is_true(), enable.is_true()),
                _ => (false, false),
            };
            if enable_reader {
                write!(f, "\n")?;
                super::json::generate_hpp_structure_reader(f, file, indent, structure)?;
            }
            if enable_writer {
                write!(f, "\n")?;
                super::json::generate_hpp_structure_writer(f, file, indent, structure)?;
            }
        }

        indent.pop();
    }

    write!(f, "{indent}}};\n")?;

    Ok(())
}
