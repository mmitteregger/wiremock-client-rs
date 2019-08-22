use serde::{Deserialize, Serialize};

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
/// The fault to apply (instead of a full, valid response).
pub enum Fault {
    CONNECTION_RESET_BY_PEER,
    EMPTY_RESPONSE,
    MALFORMED_RESPONSE_CHUNK,
    RANDOM_DATA_THEN_CLOSE,
    #[doc(hidden)]
    __Nonexhaustive,
}
