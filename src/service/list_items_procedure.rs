use crate::pb;

use super::error::ProtoFieldError;

use crate::error::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Iterate {
    Types,
    Items,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keep {
    Configured,
    NotConfigured,
    InItems,
    NotInItems,
    InTypes,
    None,
}

pub struct Procedure {
    pub get_items: bool,
    pub get_types: bool,
    pub iterate: Iterate,
    pub keep: Keep,
}

impl Procedure {
    fn new(
        get_items: bool,
        get_types: bool,
        iterate: Iterate,
        keep: Keep,
    ) -> Self {
        Self {
            get_items,
            get_types,
            iterate,
            keep,
        }
    }
}

// This implementation was computed by drawing a truth table and trying to
// minimize the number of match arms. It has been formatted for readability.
impl TryFrom<&pb::ListReq> for Procedure {
    type Error = Error;
    fn try_from(req: &pb::ListReq) -> Result<Self, Self::Error> {

        // include_naming is true if any of the naming fields are true
        // If it's false, then it means the types list, which they are indexed
        // by, is not needed.
        let include_naming = req.include_name
            || req.include_market_group
            || req.include_group
            || req.include_category;

        Ok(match (
            pb::Query::from_i32(req.include_configured)
                .ok_or(ProtoFieldError::InvalidQuery {
                    name: "include_configured",
                    value: req.include_configured,
                })?,
            pb::Query::from_i32(req.include_enabled)
                .ok_or(ProtoFieldError::InvalidQuery {
                    name: "include_enabled",
                    value: req.include_enabled,
                })?,
            req.include_json,
            include_naming,
        ) {
            ( pb::Query::Both,  pb::Query::Both,  x, _     ) => Self::new( x,     true,  Iterate::Types, Keep::InTypes       ),
            ( pb::Query::True,  pb::Query::False, _, _     ) => Self::new( false, false, Iterate::None,  Keep::None          ),
            ( _,                pb::Query::False, _, _     ) => Self::new( true,  true,  Iterate::Types, Keep::NotInItems    ),
            ( pb::Query::True,  _,                _, false ) => Self::new( true,  false, Iterate::Items, Keep::Configured    ),
            ( pb::Query::True,  _,                _, true  ) => Self::new( true,  true,  Iterate::Types, Keep::Configured    ),
            ( _,                pb::Query::Both,  _, _     ) => Self::new( true,  true,  Iterate::Types, Keep::NotConfigured ),
            ( pb::Query::False, _,                _, false ) => Self::new( true,  false, Iterate::Items, Keep::NotConfigured ),
            ( pb::Query::False, _,                _, true  ) => Self::new( true,  true,  Iterate::Types, Keep::NotConfigured ),
            ( _,                _,                _, false ) => Self::new( true,  false, Iterate::Items, Keep::InItems       ),
            ( _,                _,                _, true  ) => Self::new( true,  true,  Iterate::Types, Keep::InItems       ),
        })
    }
}
