// Relative Modules
pub mod text;
pub mod vcx;

// Standard Uses
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;

// Crate Uses
use crate::target_project::Target;

// External Uses
use eyre::Result;
use once_cell::sync::Lazy;

pub static KINDS_PROVIDER: Lazy<
    HashMap<&'static str, for<'a> fn(&Path) -> Result<Rc<RefCell<dyn Target>>>>,
> = Lazy::new(|| {
    todo!()
    /*
    HashMap::from([
        ("plain_text", TextSolution::from_path_shared as _),
        ("visual_studio.vcx", VCXSolution::from_path_shared as _)
    ])
    */
});
