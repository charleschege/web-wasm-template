use log::{trace, Level};
use std::panic;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    match console_log::init_with_level(Level::Trace) {
        Ok(_) => (),
        Err(e) => trace!("{:?}", e),
    }

    Ok(())
}
