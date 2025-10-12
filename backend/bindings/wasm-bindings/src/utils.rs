pub type JsFallible<T = ()> = ::core::result::Result<T, ::wasm_bindgen::JsValue>;

pub trait FallibleExt<T = ()> {
    fn into_js(self) -> JsFallible<T>;
}

impl<T> FallibleExt<T> for ::aliases::result::Fallible<T> {
    fn into_js(self) -> JsFallible<T> {
        self.map_err(|error| ::wasm_bindgen::JsValue::from_str(&error.to_string()))
    }
}
