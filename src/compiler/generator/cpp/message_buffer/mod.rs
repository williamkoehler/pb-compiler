mod reader;
mod writer;

use std::fmt::{Formatter, Result};

use crate::compiler::ast::*;

pub fn generate_hpp_structure_reader(
    f: &mut Formatter<'_>,
    file: &File,
    indent: &mut String,
    structure: &Structure,
) -> Result {
    write!(
        f,
        "{indent}bool Deserialize(const server::scripting::sdk::MessageBuffer& _message_buffer)\n"
    )?;
    write!(f, "{indent}{{\n")?;

    {
        indent.push('\t');

        write!(
            f,
            "{indent}uint8_t* _buffer = _message_buffer.GetBuffer();\n"
        )?;
        write!(f, "\n")?;

        // Read size offset
        write!(
            f,
            "{indent}uint16_t _size_offset = *(uint16_t*)_buffer; _buffer += sizeof(uint16_t);\n"
        )?;

        // Check buffer size
        write!(
            f,
            "{indent}if(_message_buffer.GetSize() != (2 + {} + _size_offset))\n",
            structure.min_size()
        )?;
        write!(f, "{indent}\treturn false;\n")?;
        write!(f, "\n")?;

        reader::generate_hpp_structure_reader(f, file, indent, "(*this)", structure)?;
        write!(f, "\n")?;

        write!(f, "{indent}return true;\n")?;

        indent.pop();
    }

    write!(f, "{indent}}}\n")?;

    Ok(())
}

pub fn generate_hpp_structure_writer(
    f: &mut Formatter<'_>,
    file: &File,
    indent: &mut String,
    structure: &Structure,
) -> Result {
    write!(f, "{indent}void Serialize(server::scripting::sdk::MessageBuffer& _message_buffer)\n")?;
    write!(f, "{indent}{{\n")?;

    {
        indent.push('\t');

        write!(f, "{indent}uint16_t _size_offset = 0;\n")?;
        writer::generate_hpp_structure_size_offset(f, file, indent, "(*this)", structure)?;

        // Set buffer size
        write!(
            f,
            "{indent}_message_buffer.SetSize(2 + {} + _size_offset);\n",
            structure.min_size()
        )?;
        write!(f, "\n")?;

        write!(
            f,
            "{indent}uint8_t* _buffer = _message_buffer.GetBuffer();\n"
        )?;
        write!(f, "\n")?;

        // Write size offset
        write!(
            f,
            "{indent}*(uint16_t*)_buffer = _size_offset; _buffer += sizeof(uint16_t);\n"
        )?;

        writer::generate_hpp_structure_writer(f, file, indent, "(*this)", structure)?;

        indent.pop();
    }

    write!(f, "{indent}}}\n")?;

    Ok(())
}

pub fn generate_hpp_variant_reader(
    f: &mut Formatter<'_>,
    file: &File,
    indent: &mut String,
    variant: &Variant,
) -> Result {
    write!(f, "{indent}bool Deserialize(server::scripting::sdk::MessageBuffer& _message_buffer)\n")?;
    write!(f, "{indent}{{\n")?;

    {
        indent.push('\t');

        write!(
            f,
            "{indent}uint8_t* _buffer = _message_buffer.GetBuffer();\n"
        )?;
        write!(f, "\n")?;

        // Read size offset
        write!(
            f,
            "{indent}uint16_t _size_offset = *(uint16_t*)_buffer; _buffer += sizeof(uint16_t);\n"
        )?;

        // Check buffer size
        write!(
            f,
            "{indent}if(_message_buffer.GetSize() != (2 + {} + _size_offset))\n",
            variant.min_size()
        )?;
        write!(f, "{indent}\treturn false;\n")?;
        write!(f, "\n")?;

        reader::generate_hpp_variant_reader(f, file, indent, "(*this)", variant)?;
        write!(f, "\n")?;

        write!(f, "{indent}return true;\n")?;

        indent.pop();
    }

    write!(f, "{indent}}}\n")?;

    Ok(())
}

pub fn generate_hpp_variant_writer(
    f: &mut Formatter<'_>,
    file: &File,
    indent: &mut String,
    variant: &Variant,
) -> Result {
    write!(f, "{indent}void Serialize(server::scripting::sdk::MessageBuffer& _message_buffer)\n")?;
    write!(f, "{indent}{{\n")?;

    {
        indent.push('\t');

        write!(f, "{indent}uint16_t _size_offset = 0;\n")?;
        writer::generate_hpp_variant_size_offset(f, file, indent, "(*this)", variant)?;

        // Set buffer size
        write!(
            f,
            "{indent}_message_buffer.SetSize(2 + {} + _size_offset);\n",
            variant.min_size()
        )?;
        write!(f, "\n")?;

        write!(
            f,
            "{indent}uint8_t* _buffer = _message_buffer.GetBuffer();\n"
        )?;
        write!(f, "\n")?;

        // Write size offset
        write!(
            f,
            "{indent}*(uint16_t*)_buffer = _size_offset; _buffer += sizeof(uint16_t);\n"
        )?;

        writer::generate_hpp_variant_writer(f, file, indent, "(*this)", variant)?;

        indent.pop();
    }

    write!(f, "{indent}}}\n")?;

    Ok(())
}
