use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct Timing {
    #[serde(rename = "addedDelay")]
    added_delay: u32,
    #[serde(rename = "processTime")]
    process_time: u32,
    #[serde(rename = "responseSendTime")]
    response_send_time: u32,
}

impl Timing {
    /// The delay added to the response via the stub or global configuration.
    pub fn added_delay(&self) -> u32 {
        self.added_delay
    }

    /// The amount of time spent handling the stub request
    pub fn process_time(&self) -> u32 {
        self.process_time
    }

    /// The amount of time taken to send the response to the client
    pub fn response_send_time(&self) -> u32 {
        self.response_send_time
    }

    /// The total request time from start to finish, minus added delay
    pub fn serve_time(&self) -> u32 {
        self.process_time + self.response_send_time
    }

    /// The total request time including added delay
    pub fn total_time(&self) -> u32 {
        self.serve_time() + self.added_delay
    }
}
