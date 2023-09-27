use std::fmt::{Formatter, Result};

use crate::compiler::ast::*;

pub fn generate_hpp_friends(f: &mut Formatter<'_>, file: &File, indent: &mut String, rank: usize) -> Result {
    for data_type in file.data_types() {
        if data_type.max_rank() < rank {
            match data_type.kind() {
                DataTypeKind::Structure(structure) => {
                    write!(f, "{indent}friend class {};\n", structure.identifier())?;
                }
                DataTypeKind::Variant(variant) => {
                    write!(f, "{indent}friend class {};\n", variant.identifier())?;
                }
                _ => {}
            }
        }
    }

    Ok(())
}
