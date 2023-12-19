use std::path::PathBuf;

/* -------------------------------------------------------------------------- */
/*                              Struct: Checkout                              */
/* -------------------------------------------------------------------------- */

pub struct Checkout {
    pub path: PathBuf,
    pub revision: super::Reference,
}
