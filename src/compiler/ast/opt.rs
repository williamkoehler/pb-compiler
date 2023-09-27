use std::collections::HashMap;

// That contains options
pub trait Optioned {
    fn options(&self) -> &HashMap<String, Vec<super::Expression>>;
    fn options_mut(&mut self) -> &mut HashMap<String, Vec<super::Expression>>;

    fn add_option(&mut self, name: String, arguments: Vec<super::Expression>) -> bool;

    fn option(&self, name: &str) -> Option<&Vec<super::Expression>>;

    fn is_option_enabled(&self, name: &str) -> bool {
        if let Some(opt) = self.option(name) {
            if let [super::Expression::Value(value)] = opt.as_slice() {
                match value {
                    super::Value::True | super::Value::Null => return true,
                    _ => {}
                }
            }
        }

        false
    }

    #[inline(always)]
    fn is_option_disabled(&self, name: &str) -> bool {
        !self.is_option_enabled(name)
    }

    fn is_option_enabled_at(&self, name: &str, index: usize) -> bool {
        if let Some(opt) = self.option(name) {
            if index < opt.len() {
                if let super::Expression::Value(value) = &opt[index] {
                    match value {
                        super::Value::True | super::Value::Null => return true,
                        _ => {}
                    }
                }
            }
        }

        false
    }

    #[inline(always)]
    fn is_option_disabled_at(&self, name: &str, index: usize) -> bool {
        !self.is_option_enabled_at(name, index)
    }

    fn option_mut(&mut self, name: &str) -> Option<&mut Vec<super::Expression>>;
}
