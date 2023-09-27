pub mod cpp;

pub struct Generator {
    file: super::ast::File,
}

impl Generator {
    pub fn from(file: super::ast::File) -> Self {
        Self { file: file }
    }
}
