use std::fmt::{Formatter, Result};

use crate::compiler::ast::*;

pub fn generate_hpp_file(f: &mut Formatter<'_>, file: &File) -> Result {
    write!(f, "#pragma once\n")?;
    write!(f, "#include \"message_buffer.hpp\"\n")?;
    write!(f, "#include <rapidjson/document.h>\n")?;
    write!(f, "#include <array>\n")?;
    write!(f, "\n")?;

    let ordered_data_types = {
        let mut data_types = file
            .data_types()
            .iter()
            .map(|a| (a, a.max_rank()))
            .collect::<Vec<(&DataType, usize)>>();

        data_types.sort_by(|(_, a), (_, b)| b.cmp(a));
        data_types
    };

    for (data_type, _) in &ordered_data_types {
        match data_type.kind() {
            DataTypeKind::Structure(structure) => {
                super::structure::generate_hpp_structure_declaration(f, structure)?;
            }
            DataTypeKind::Variant(variant) => {
                super::variant::generate_hpp_variant_declaration(f, variant)?;
            }
            _ => {}
        }
    }

    write!(f, "\n")?;

    for (data_type, _) in &ordered_data_types {
        match data_type.kind() {
            DataTypeKind::Structure(structure) => {
                super::structure::generate_hpp_structure(f, file, data_type, structure)?;
                write!(f, "\n")?;
            }
            DataTypeKind::Variant(variant) => {
                super::variant::generate_hpp_variant(f, file, data_type, variant)?;
            }
            _ => {}
        }
    }

    Ok(())
}
