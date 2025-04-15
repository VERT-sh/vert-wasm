use icns::IconFamily;
use std::io::BufReader;
use thiserror::Error;
use wasm_bindgen::prelude::*;

#[derive(Debug, Error)]
pub enum IcnsError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

impl From<IcnsError> for JsValue {
    fn from(val: IcnsError) -> Self {
        JsValue::from(format!("{:?}", val))
    }
}

#[wasm_bindgen]
pub fn parse_icns(file: Box<[u8]>) -> Result<JsValue, IcnsError> {
    let reader = BufReader::new(file.as_ref());
    let icon_family = IconFamily::read(reader)?;
    let icons = icon_family
        .elements
        .into_iter()
        .map(|i| i.data)
        .map(JsValue::from)
        .collect::<Vec<_>>();

    let value = JsValue::from(icons);

    Ok(value)
}
