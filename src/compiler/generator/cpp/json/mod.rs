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
    write!(f, "{indent}bool Read(const rapidjson::Value& _value)\n")?;
    write!(f, "{indent}{{\n")?;

    {
        indent.push('\t');

        write!(f, "{indent}const rapidjson::Value& _temp_0 = _value;\n")?;
        write!(f, "{indent}if (!_temp_0.IsObject()) return false;\n")?;

        reader::generate_hpp_structure_reader(f, file, 1, indent, "(*this)", structure)?;
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
    write!(f, "{indent}void Write(rapidjson::Value& _value, rapidjson::Document::AllocatorType& _allocator)\n")?;
    write!(f, "{indent}{{\n")?;

    {
        indent.push('\t');

        write!(f, "{indent}rapidjson::Value& _temp_0 = _value;\n")?;
        write!(f, "{indent}_temp_0.SetObject();\n")?;

        writer::generate_hpp_structure_writer(f, file, 1, indent, "(*this)", structure)?;

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
    write!(f, "{indent}bool Read(rapidjson::Value& _value)\n")?;
    write!(f, "{indent}{{\n")?;

    {
        indent.push('\t');

        write!(f, "{indent}const rapidjson::Value& _temp_0 = _value;\n")?;
        write!(f, "{indent}if (!_temp_0.IsObject())\n")?;
        write!(f, "{indent}\treturn false;\n")?;

        reader::generate_hpp_variant_reader(f, file, 1, indent, "(*this)", variant)?;
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
    write!(f, "{indent}void Write(rapidjson::Value& _value, rapidjson::Document::AllocatorType& _allocator)\n")?;
    write!(f, "{indent}{{\n")?;

    {
        indent.push('\t');

        write!(f, "{indent}rapidjson::Value& _temp_0 = _value;\n")?;
        write!(f, "{indent}_temp_0.SetObject();\n")?;

        writer::generate_hpp_variant_writer(f, file, 1, indent, "(*this)", variant)?;

        indent.pop();
    }

    write!(f, "{indent}}}\n")?;

    Ok(())
}
