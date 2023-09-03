// Standard Uses

// Crate Uses
use crate::shaping::patch::Actions;

// External Uses
use knuffel::Decode;

#[derive(Default, Clone, Debug, PartialEq, Decode)]
pub struct Replacer {
    #[knuffel(argument)]
    pub name: String,

    #[knuffel(property)]
    pub location: String,

    #[knuffel(property)]
    pub reference_location: Option<String>,

    #[knuffel(property)]
    pub reference: Option<String>,

    // #[knuffel(child, unwrap(argument))]
    // pub flags: Option<String>,
    #[knuffel(child, unwrap(argument))]
    pub from: String,

    #[knuffel(child, unwrap(argument))]
    pub to: String,

    #[knuffel(child)]
    pub actions: Option<Actions>,
}
