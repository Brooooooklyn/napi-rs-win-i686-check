use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
pub struct MyClass(u32);

#[napi]
impl MyClass {
    #[napi(constructor)]
    pub fn new() -> Result<Self> {
        Ok(Self(13))
    }
}
