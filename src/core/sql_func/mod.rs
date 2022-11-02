use sea_orm::Iden;

pub struct JsonArrayAppend;
impl Iden for JsonArrayAppend {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        s.write_str("JSON_ARRAY_APPEND").unwrap();
    }
}
