mod file;
mod reference;
mod friends;
mod structure;
mod variant;
mod message_buffer;
mod json;

use format::lazy_format;

use super::Generator;

impl Generator {
    pub fn generate_cplusplus(&self) -> String {
        lazy_format!(|f| file::generate_hpp_file(f, &self.file)).to_string()
    }
}
