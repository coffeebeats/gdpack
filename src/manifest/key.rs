pub(super) const MANIFEST_SECTION_KEY_ADDONS: &str = "addons";
pub(super) const MANIFEST_SECTION_KEY_ADDONS_DEV: &str = "dev-addons";

#[derive(typed_builder::TypedBuilder)]
pub struct Key {
    pub dev: bool,
    #[builder(default, setter(strip_option))]
    pub target: Option<String>,
}

impl Key {
    pub fn last(&self) -> &str {
        match self.dev {
            true => MANIFEST_SECTION_KEY_ADDONS_DEV,
            false => MANIFEST_SECTION_KEY_ADDONS,
        }
    }
}
