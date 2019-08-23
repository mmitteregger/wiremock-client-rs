use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct Timing {
    #[serde(rename = "addedDelay", with = "crate::serde::u32_negative_to_option")]
    added_delay: Option<u32>,
    #[serde(rename = "processTime", with = "crate::serde::u32_negative_to_option")]
    process_time: Option<u32>,
    #[serde(rename = "responseSendTime", with = "crate::serde::u32_negative_to_option")]
    response_send_time: Option<u32>,
}

impl Timing {
    /// The delay added to the response via the stub or global configuration.
    pub fn added_delay(&self) -> Option<u32> {
        self.added_delay
    }

    /// The amount of time spent handling the stub request
    pub fn process_time(&self) -> Option<u32> {
        self.process_time
    }

    /// The amount of time taken to send the response to the client
    pub fn response_send_time(&self) -> Option<u32> {
        self.response_send_time
    }

    /// The total request time from start to finish, minus added delay
    pub fn serve_time(&self) -> Option<u32> {
        match (self.process_time, self.response_send_time) {
            (Some(process_time), Some(response_send_time)) => {
                Some(process_time + response_send_time)
            },
            _ => None
        }
    }

    /// The total request time including added delay
    pub fn total_time(&self) -> Option<u32> {
        match (self.serve_time(), self.added_delay) {
            (Some(serve_time), Some(added_delay)) => {
                Some(serve_time + added_delay)
            },
            _ => None
        }
    }
}
