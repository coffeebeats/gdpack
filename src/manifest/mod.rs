/* ------------------------------- Mod: Addon ------------------------------- */

mod addon;

/* ------------------------------ Mod: Manifest ----------------------------- */

mod manifest;

pub use manifest::parse_from;
pub use manifest::write_to;
pub use manifest::Manifest;
pub use manifest::MANIFEST_FILENAME;
pub use manifest::MANIFEST_SECTION_KEY_ADDONS;
pub use manifest::MANIFEST_SECTION_KEY_ADDONS_DEV;
