pub mod ast;
pub mod diagnostic;
pub mod generator;
pub mod parser;
pub mod semantic;

pub struct Compiler {
    reports: Vec<diagnostic::Report>,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            reports: Vec::new(),
        }
    }

    #[inline]
    pub fn reports(&self) -> &[diagnostic::Report] {
        self.reports.as_slice()
    }

    pub fn compile(&mut self, input: &str) -> Result<(), ()> {
        let mut file = ast::File::new("");

        file.add_data_type(ast::DataType::from(ast::DataTypeKind::Boolean));
        file.add_data_type(ast::DataType::from(ast::DataTypeKind::Int8));
        file.add_data_type(ast::DataType::from(ast::DataTypeKind::UInt8));
        file.add_data_type(ast::DataType::from(ast::DataTypeKind::Int16));
        file.add_data_type(ast::DataType::from(ast::DataTypeKind::UInt16));
        file.add_data_type(ast::DataType::from(ast::DataTypeKind::Int32));
        file.add_data_type(ast::DataType::from(ast::DataTypeKind::UInt32));
        file.add_data_type(ast::DataType::from(ast::DataTypeKind::Int64));
        file.add_data_type(ast::DataType::from(ast::DataTypeKind::UInt64));
        file.add_data_type(ast::DataType::from(ast::DataTypeKind::Single));
        file.add_data_type(ast::DataType::from(ast::DataTypeKind::Double));

        file.add_data_type(ast::DataType::from(ast::DataTypeKind::String));

        file.add_alias(ast::Alias::from("size".to_string(), "int64".to_string()));
        file.add_alias(ast::Alias::from("usize".to_string(), "uint64".to_string()));

        // Parse
        parser::Parser::from(input).parse(self, &mut file);
        if !self.reports.is_empty() {
            return Err(())
        }

        // Semantic
        semantic::Semantic::new().analyse(self, &mut file);
        if !self.reports.is_empty() {
            return Err(())
        }

        println!("{}", generator::Generator::from(file).generate_cplusplus());

        // Generation

        if self.reports.is_empty() {
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn diagnose(&mut self, report: diagnostic::Report) {
        self.reports.push(report);
    }
}
