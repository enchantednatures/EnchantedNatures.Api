use std::fs::read_to_string;
use std::path::Path;

pub fn read_spec() -> String {
    read_to_string(Path::new("api/enchanted-natures.openapi.spec.yaml".into())).unwrap()
}
