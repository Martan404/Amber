use heraclitus_compiler::prelude::*;
use crate::utils::ParserMetadata;

#[derive(Debug, Clone)]
pub struct ImportString {
    pub value: String
}

impl ImportString {
    pub fn std_lib_path(&mut self) {
        if self.value == "std" {
            self.value = "[standard library]".to_string();
        }
    }
}

impl SyntaxModule<ParserMetadata> for ImportString {
    syntax_name!("Import String");

    fn new() -> Self {
        Self {
            value: String::new()
        }
    }

    fn parse(&mut self, meta: &mut ParserMetadata) -> SyntaxResult {
        let value = token_by(meta, |word| word.starts_with('\''))?;
        if value.ends_with('\'') {
            self.value = value[1..value.len() - 1].to_string();
            self.std_lib_path();
        }
        else {
            return error!(meta, meta.get_current_token(), "Import string cannot interpolate expressions")
        }
        Ok(())
    }
}