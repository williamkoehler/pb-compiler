use std::collections::{HashMap, HashSet, VecDeque};

use super::ast::*;

use regex::Regex;

struct Node {
    pub is_alias: bool,
    pub links: Vec<usize>,

    pub dfs_visited_mark: bool,
    pub dfs_mark: bool,
}

pub struct Semantic {}

impl Semantic {
    pub fn new() -> Self {
        Self {}
    }

    pub fn analyse(mut self, compiler: &mut super::Compiler, file: &mut File) {
        self.analyze_dependencies(compiler, file);
        self.analyze_options(compiler, file);
    }

    fn analyze_dependencies(&mut self, compiler: &mut super::Compiler, file: &mut File) {
        let data_type_identifier_regex =
            Regex::new(r"^[A-Z][a-z]*(?:[A-Z][a-z]*|[0-9]+)*$").unwrap();
        let field_identifier_regex = Regex::new(r"^[a-z][a-z0-9]*(?:_[a-z0-9]+)*$").unwrap();

        let mut data_type_names = HashMap::new();

        let mut nodes = Vec::with_capacity(file.data_types().len());

        // Add nodes
        for (id, data_type) in file.data_types().iter().enumerate() {
            // Add data type name
            if let Some(name) = data_type.identifier().get_opt() {
                if data_type_names.insert(name.clone(), id).is_some() {
                    compiler.diagnose(super::diagnostic::err_redefined_data_type(name));
                }

                // Check case
                match data_type.kind() {
                    DataTypeKind::Structure(_) | DataTypeKind::Variant(_) => {
                        if !data_type_identifier_regex.is_match(name) {
                            compiler
                                .diagnose(super::diagnostic::err_invalid_data_type_identifer(name));
                        }
                    }
                    _ => {}
                }
            }

            match data_type.kind() {
                DataTypeKind::Boolean
                | DataTypeKind::Int8
                | DataTypeKind::UInt8
                | DataTypeKind::Int16
                | DataTypeKind::UInt16
                | DataTypeKind::Int32
                | DataTypeKind::UInt32
                | DataTypeKind::Int64
                | DataTypeKind::UInt64
                | DataTypeKind::Single
                | DataTypeKind::Double
                | DataTypeKind::String => {
                    nodes.push(Node {
                        is_alias: false,
                        links: Vec::new(),
                        dfs_visited_mark: false,
                        dfs_mark: false,
                    });
                }
                DataTypeKind::Structure(structure) => {
                    nodes.push(Node {
                        is_alias: false,
                        links: Vec::new(),
                        dfs_visited_mark: false,
                        dfs_mark: false,
                    });

                    // Check redefinition inside structure
                    let mut field_names = HashSet::new();

                    for field in structure.fields() {
                        if let Some(name) = field.identifier().get_opt() {
                            if !field_names.insert(name.clone()) {
                                compiler.diagnose(super::diagnostic::err_redefined_field(name));
                            }

                            // Check case
                            if !field_identifier_regex.is_match(name) {
                                compiler.diagnose(
                                    super::diagnostic::err_invalid_data_type_identifer(name),
                                );
                            }
                        }
                    }
                }
                DataTypeKind::Variant(variant) => {
                    nodes.push(Node {
                        is_alias: false,
                        links: Vec::new(),
                        dfs_visited_mark: false,
                        dfs_mark: false,
                    });

                    // Check redefinition inside variant
                    let mut field_names = HashSet::new();

                    for field in variant.fields() {
                        if let Some(name) = field.identifier().get_opt() {
                            if !field_names.insert(name.clone()) {
                                compiler.diagnose(super::diagnostic::err_redefined_field(name));
                            }

                            // Check case
                            if !field_identifier_regex.is_match(name) {
                                compiler.diagnose(
                                    super::diagnostic::err_invalid_data_type_identifer(name),
                                );
                            }
                        }
                    }
                }
                DataTypeKind::Alias(_) => {
                    nodes.push(Node {
                        is_alias: true,
                        links: Vec::new(),
                        dfs_visited_mark: false,
                        dfs_mark: false,
                    });
                }
            }
        }

        // Add links
        for (src_id, data_type) in file.data_types().iter().enumerate() {
            match data_type.kind() {
                DataTypeKind::Structure(structure) => {
                    for field in structure.fields() {
                        if let Some(reference) = field.reference().get_opt() {
                            if let Some(dst_id) = data_type_names.get(reference) {
                                nodes[src_id].links.push(*dst_id);
                            } else {
                                compiler.diagnose(super::diagnostic::err_undeclared_data_type(
                                    reference,
                                ));
                            }
                        }
                    }
                }
                DataTypeKind::Variant(variant) => {
                    for field in variant.fields() {
                        if let Some(reference) = field.reference().get_opt() {
                            if let Some(dst_id) = data_type_names.get(reference) {
                                nodes[src_id].links.push(*dst_id);
                            } else {
                                compiler.diagnose(super::diagnostic::err_undeclared_data_type(
                                    reference,
                                ));
                            }
                        }
                    }
                }
                DataTypeKind::Alias(alias) => {
                    if let Some(reference) = alias.reference().get_opt() {
                        if let Some(dst_id) = data_type_names.get(reference) {
                            nodes[src_id].links.push(*dst_id);
                        } else {
                            compiler
                                .diagnose(super::diagnostic::err_undeclared_data_type(reference));
                        }
                    }
                }
                _ => {}
            }
        }

        // Check for cyclical dependencies
        {
            let mut stack: Vec<usize> = Vec::with_capacity(nodes.len());
            let mut queue: VecDeque<(usize, usize)> = VecDeque::with_capacity(nodes.len());

            #[inline(always)]
            fn update_data_type(
                compiler: &mut super::Compiler,
                file: &mut File,
                rank: usize,
                id: usize,
                parent_id: usize,
            ) {
                // Update data type
                let mut size = 0;

                if let Some(data_type) = file.data_type_mut(id) {
                    data_type.update_max_rank(rank);

                    size = data_type.size();
                } else {
                    compiler.diagnose(super::diagnostic::internal_error("Invalid data type id"));
                }

                // Update parent data type
                if let Some(data_type) = file.data_type_mut(parent_id) {
                    match data_type.kind_mut() {
                        DataTypeKind::Structure(structure) => {
                            structure.update_min_size(size);
                        }
                        DataTypeKind::Variant(variant) => {
                            variant.update_min_size(size);
                        }
                        _ => {}
                    }
                } else {
                    compiler.diagnose(super::diagnostic::internal_error("Invalid data type id"));
                }
            }

            loop {
                if let Some((id, rank)) = queue.pop_front() {
                    // Unmark
                    if rank <= stack.len() {
                        for rank in (rank - 1..stack.len()).rev() {
                            let id = stack[rank];
                            nodes[id].dfs_mark = false;

                            if rank > 0 {
                                update_data_type(compiler, file, rank, id, stack[rank - 1]);
                            }
                        }

                        stack.resize(rank - 1, 0usize);
                    }

                    // Mark and queue
                    {
                        let node = &mut nodes[id];

                        if !node.dfs_mark {
                            node.dfs_visited_mark = true;
                            node.dfs_mark = true;

                            // Add neighbours to queue
                            for node_id in &node.links {
                                queue.push_front((*node_id, rank + 1));
                            }

                            stack.push(id);
                        } else {
                            // Node was already visited which means a cycle is present
                            compiler.diagnose(super::diagnostic::err_cyclical_dependency(
                                stack
                                    .iter()
                                    .map(|id| file.data_types()[*id].identifier().get().to_string())
                                    .collect::<Vec<String>>()
                                    .as_slice(),
                            ))
                        }
                    }
                } else {
                    if let Some(id) = nodes.iter().position(|node| !node.dfs_visited_mark) {
                        queue.push_front((id, 1usize));
                    } else {
                        break;
                    }
                }
            }

            for rank in (1..stack.len()).rev() {
                if rank > 0 {
                    update_data_type(compiler, file, rank, stack[rank], stack[rank - 1]);
                }
            }
        }

        // Simplify dependencies
        {
            for (src_id, src_node) in nodes.iter().enumerate() {
                // Only simplify for non alias nodes
                if !src_node.is_alias {
                    for (link_id, dst_id) in src_node.links.iter().enumerate() {
                        let mut dst_node = &nodes[*dst_id];

                        // Only simplify alias nodes
                        // if dst_node.is_alias {
                        {
                            let mut dst_id = *dst_id;
                            while dst_node.is_alias {
                                dst_id = dst_node.links[0];
                                dst_node = &nodes[dst_id];
                            }

                            if let Some(data_type) = file.data_type_mut(src_id) {
                                match data_type.kind_mut() {
                                    DataTypeKind::Structure(structure) => {
                                        if let Some(field) = structure.field_mut(link_id) {
                                            field.reference_mut().set_id(dst_id);
                                        }
                                    }
                                    DataTypeKind::Variant(variant) => {
                                        if let Some(field) = variant.field_mut(link_id) {
                                            field.reference_mut().set_id(dst_id);
                                        }
                                    }
                                    _ => {}
                                }
                            } else {
                                compiler.diagnose(super::diagnostic::internal_error(
                                    "Invalid data type id",
                                ));
                            }
                        }
                    }
                }
            }
        }
    }

    fn analyze_options(&mut self, compiler: &mut super::Compiler, file: &mut File) {
        for data_type in file.data_types_mut() {
            match data_type.kind_mut() {
                DataTypeKind::Structure(structure) => {
                    for (_, arguments) in structure.options_mut() {
                        for argument in arguments {
                            *argument =
                                Expression::Value(self.analyse_expression(compiler, argument));
                        }
                    }
                }
                DataTypeKind::Variant(variant) => {
                    for (_, arguments) in variant.options_mut() {
                        for argument in arguments {
                            *argument =
                                Expression::Value(self.analyse_expression(compiler, argument));
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn analyse_expression(
        &mut self,
        compiler: &mut super::Compiler,
        expression: &Expression,
    ) -> Value {
        match expression {
            Expression::Value(value) => value.clone(),
            Expression::UnaryOperator(op, expression) => {
                let value = self.analyse_expression(compiler, expression);
                match value {
                    Value::Null => Value::Null,
                    Value::True => match op {
                        UnaryOperator::LogicalNot => Value::False,
                        _ => {
                            compiler.diagnose(super::diagnostic::err_invalid_expression_operand(
                                *op, &value,
                            ));
                            Value::Null
                        }
                    },
                    Value::False => match op {
                        UnaryOperator::LogicalNot => Value::True,
                        _ => {
                            compiler.diagnose(super::diagnostic::err_invalid_expression_operand(
                                *op, &value,
                            ));
                            Value::Null
                        }
                    },
                    Value::Integer(integer) => match op {
                        UnaryOperator::Negation => Value::Integer(-integer),
                        _ => {
                            compiler.diagnose(super::diagnostic::err_invalid_expression_operand(
                                *op, &value,
                            ));
                            Value::Null
                        }
                    },
                    Value::Real(real) => match op {
                        UnaryOperator::Negation => Value::Real(-real),
                        _ => {
                            compiler.diagnose(super::diagnostic::err_invalid_expression_operand(
                                *op, &value,
                            ));
                            Value::Null
                        }
                    },
                    Value::Literal(_) => {
                        compiler.diagnose(super::diagnostic::err_invalid_expression_operand(
                            *op, &value,
                        ));
                        Value::Null
                    }
                }
            }
            Expression::BinaryOperator(expression_1, op, expression_2) => {
                fn handle_integer(
                    compiler: &mut super::Compiler,
                    op: BinaryOperator,
                    value_1: &Value,
                    integer_1: i64,
                    value_2: &Value,
                    integer_2: i64,
                ) -> Value {
                    match op {
                        BinaryOperator::Addition => Value::Integer(integer_1 + integer_2),
                        BinaryOperator::Subtraction => Value::Integer(integer_1 - integer_2),
                        BinaryOperator::Multiplication => Value::Integer(integer_1 * integer_2),
                        BinaryOperator::Division => Value::Integer(integer_1 / integer_2),
                        BinaryOperator::Modulo => Value::Integer(integer_1 % integer_2),
                        BinaryOperator::Equal => {
                            if integer_1 == integer_2 {
                                Value::True
                            } else {
                                Value::False
                            }
                        }
                        BinaryOperator::NotEqual => {
                            if integer_1 != integer_2 {
                                Value::True
                            } else {
                                Value::False
                            }
                        }
                        BinaryOperator::GreaterThan => {
                            if integer_1 > integer_2 {
                                Value::True
                            } else {
                                Value::False
                            }
                        }
                        BinaryOperator::GreaterThanEqual => {
                            if integer_1 >= integer_2 {
                                Value::True
                            } else {
                                Value::False
                            }
                        }
                        BinaryOperator::LessThan => {
                            if integer_1 < integer_2 {
                                Value::True
                            } else {
                                Value::False
                            }
                        }
                        BinaryOperator::LessThanEqual => {
                            if integer_1 <= integer_2 {
                                Value::True
                            } else {
                                Value::False
                            }
                        }
                        _ => {
                            compiler.diagnose(super::diagnostic::err_invalid_expression_operands(
                                op, value_1, value_2,
                            ));
                            Value::Null
                        }
                    }
                }
                fn handle_real(
                    compiler: &mut super::Compiler,
                    op: BinaryOperator,
                    value_1: &Value,
                    real_1: f64,
                    value_2: &Value,
                    real_2: f64,
                ) -> Value {
                    match op {
                        BinaryOperator::Addition => Value::Real(real_1 + real_2),
                        BinaryOperator::Subtraction => Value::Real(real_1 - real_2),
                        BinaryOperator::Multiplication => Value::Real(real_1 * real_2),
                        BinaryOperator::Division => Value::Real(real_1 / real_2),
                        BinaryOperator::Modulo => Value::Real(real_1 % real_2),
                        BinaryOperator::Equal => {
                            if real_1 == real_2 {
                                Value::True
                            } else {
                                Value::False
                            }
                        }
                        BinaryOperator::NotEqual => {
                            if real_1 != real_2 {
                                Value::True
                            } else {
                                Value::False
                            }
                        }
                        BinaryOperator::GreaterThan => {
                            if real_1 > real_2 {
                                Value::True
                            } else {
                                Value::False
                            }
                        }
                        BinaryOperator::GreaterThanEqual => {
                            if real_1 >= real_2 {
                                Value::True
                            } else {
                                Value::False
                            }
                        }
                        BinaryOperator::LessThan => {
                            if real_1 < real_2 {
                                Value::True
                            } else {
                                Value::False
                            }
                        }
                        BinaryOperator::LessThanEqual => {
                            if real_1 <= real_2 {
                                Value::True
                            } else {
                                Value::False
                            }
                        }
                        _ => {
                            compiler.diagnose(super::diagnostic::err_invalid_expression_operands(
                                op, value_1, value_2,
                            ));
                            Value::Null
                        }
                    }
                }

                let value_1 = self.analyse_expression(compiler, expression_1);
                let value_2 = self.analyse_expression(compiler, expression_2);
                match (&value_1, &value_2) {
                    (Value::Null, Value::Null) => Value::Null,
                    (Value::False, Value::False) => match op {
                        BinaryOperator::LogicalAnd => Value::False,
                        BinaryOperator::LogicalOr => Value::False,
                        _ => {
                            compiler.diagnose(super::diagnostic::err_invalid_expression_operands(
                                *op, &value_1, &value_2,
                            ));
                            Value::Null
                        }
                    },
                    (Value::True, Value::False) => match op {
                        BinaryOperator::LogicalAnd => Value::False,
                        BinaryOperator::LogicalOr => Value::True,
                        _ => {
                            compiler.diagnose(super::diagnostic::err_invalid_expression_operands(
                                *op, &value_1, &value_2,
                            ));
                            Value::Null
                        }
                    },

                    (Value::False, Value::True) => match op {
                        BinaryOperator::LogicalAnd => Value::False,
                        BinaryOperator::LogicalOr => Value::True,
                        _ => {
                            compiler.diagnose(super::diagnostic::err_invalid_expression_operands(
                                *op, &value_1, &value_2,
                            ));
                            Value::Null
                        }
                    },
                    (Value::True, Value::True) => match op {
                        BinaryOperator::LogicalAnd => Value::True,
                        BinaryOperator::LogicalOr => Value::True,
                        _ => {
                            compiler.diagnose(super::diagnostic::err_invalid_expression_operands(
                                *op, &value_1, &value_2,
                            ));
                            Value::Null
                        }
                    },
                    (Value::Integer(integer_1), Value::Integer(integer_2)) => {
                        handle_integer(compiler, *op, &value_1, *integer_1, &value_2, *integer_2)
                    }
                    (Value::Real(real_1), Value::Real(real_2)) => {
                        handle_real(compiler, *op, &value_1, *real_1, &value_2, *real_2)
                    }
                    (Value::Integer(integer), Value::Real(real)) => {
                        handle_real(compiler, *op, &value_1, *integer as f64, &value_2, *real)
                    }
                    (Value::Real(real), Value::Integer(integer)) => {
                        handle_real(compiler, *op, &value_1, *real, &value_2, *integer as f64)
                    }
                    (Value::Literal(literal_1), Value::Literal(literal_2)) => match op {
                        BinaryOperator::Addition => {
                            Value::Literal(format!("{}{}", literal_1, literal_2))
                        }
                        _ => {
                            compiler.diagnose(super::diagnostic::err_invalid_expression_operands(
                                *op, &value_1, &value_2,
                            ));
                            Value::Null
                        }
                    },
                    (Value::Literal(literal), _) => match op {
                        BinaryOperator::Addition => {
                            Value::Literal(format!("{}{}", literal, value_2))
                        }
                        _ => {
                            compiler.diagnose(super::diagnostic::err_invalid_expression_operands(
                                *op, &value_1, &value_2,
                            ));
                            Value::Null
                        }
                    },
                    (_, Value::Literal(literal)) => match op {
                        BinaryOperator::Addition => {
                            Value::Literal(format!("{}{}", value_1, literal))
                        }
                        _ => {
                            compiler.diagnose(super::diagnostic::err_invalid_expression_operands(
                                *op, &value_1, &value_2,
                            ));
                            Value::Null
                        }
                    },
                    _ => {
                        compiler.diagnose(super::diagnostic::err_invalid_expression_operands(
                            *op, &value_1, &value_2,
                        ));
                        Value::Null
                    }
                }
            }
            _ => Value::Null,
        }
    }
}
