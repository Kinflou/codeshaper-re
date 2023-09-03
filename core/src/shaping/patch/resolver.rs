// Standard Uses
use std::collections::HashMap;

// Crate Uses

// External Uses
use knuffel::{Decode, DecodeScalar};

#[derive(Default, Clone, Debug, PartialEq, Decode)]
pub struct Resolver {
    #[knuffel(argument)]
    pub name: String,

    #[knuffel(child)]
    pub mode: Mode,

    //#[knuffel(child, unwrap(argument))]
    pub cases: HashMap<String, String>,

    //#[knuffel(child, unwrap(argument))]
    pub list: Vec<String>,

    #[knuffel(argument)]
    pub index: String,

    #[knuffel(argument)]
    pub default: String,
}

#[derive(Default, Clone, Debug, PartialEq, Decode, DecodeScalar)]
pub enum Mode {
    #[default]
    List,
    Switch,
}
