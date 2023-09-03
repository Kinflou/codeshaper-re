// Standard USes

// Crate Uses
use crate::shaping::patch::Actions;

// External Uses

#[derive(Default, Clone, Debug, PartialEq, knuffel::Decode)]
pub struct Builder {
    #[knuffel(argument)]
    pub name: String,

    #[knuffel(child, unwrap(argument))]
    pub location: String,

    #[knuffel(child, unwrap(argument))]
    pub reference_location: Option<String>,

    #[knuffel(child, unwrap(argument))]
    pub r#match: String,

    #[knuffel(child, unwrap(argument))]
    pub build: String,
    pub actions: Actions,
}
