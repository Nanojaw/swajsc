use crate::ast;
use wasmparser::validate;

pub fn parse_wasm(src: &[u8]) -> Result<ast::Module, wasmparser::BinaryReaderError> {
    // make sure that the module is valid before parsing
    validate(src)?;

    todo!()
}
