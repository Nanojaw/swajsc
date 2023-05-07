use crate::ast;
use wasmparser::validate;

pub fn parse_wasm(src: &[u8]) -> Result<ast::Module, String> {
    // make sure that the module is valid before parsing
    match validate(src) {
        Err(e) => return Err(format!("Wasm module was invalid! Error message: {}", e.message())),
        _ => ()
    }

    todo!()
}
