use heraclitus_compiler::prelude::*;
use crate::{utils::{metadata::ParserMetadata, TranslateMetadata}, modules::{Type, Typed}, translate::{module::TranslateModule, compute::{translate_computation, ArithOp}}};
use super::super::expr::Expr;

#[derive(Debug, Clone)]
pub struct Not {
    expr: Box<Expr>,
    kind: Type
}

impl Typed for Not {
    fn get_type(&self) -> Type {
        self.kind.clone()
    }
}

impl SyntaxModule<ParserMetadata> for Not {
    syntax_name!("Not");

    fn new() -> Self {
        Not {
            expr: Box::new(Expr::new()),
            kind: Type::Bool
        }
    }

    fn parse(&mut self, meta: &mut ParserMetadata) -> SyntaxResult {
        token(meta, "not")?;
        syntax(meta, &mut *self.expr)?;
        Ok(())
    }
}

impl TranslateModule for Not {
    fn translate(&self, meta: &mut TranslateMetadata) -> String {
        let expr = self.expr.translate(meta);
        translate_computation(meta, ArithOp::Not, None, Some(expr))
    }
}