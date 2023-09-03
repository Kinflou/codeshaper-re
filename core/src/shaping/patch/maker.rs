// Standard Uses

// Crate Uses

// External Uses
use knuffel::Decode;

#[derive(Default, Clone, Debug, PartialEq, Decode)]
pub struct Maker {
    #[knuffel(argument)]
    pub name: String,

    #[knuffel(child, unwrap(arguments))]
    pub arguments: Vec<String>,
    // #[knuffel(argument)]
    // pub arguments: String,
    #[knuffel(child, unwrap(argument))]
    pub prepare: Option<String>,

    #[knuffel(child, unwrap(argument))]
    pub make: String,
}
