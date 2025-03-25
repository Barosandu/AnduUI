use std::collections::HashMap;

pub(crate) trait ToDict {
    fn to_dict(&self) -> HashMap<String, (String, String)>;
}