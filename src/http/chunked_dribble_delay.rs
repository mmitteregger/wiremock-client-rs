use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChunkedDribbleDelay {
    number_of_chunks: u16,
    total_duration: u16,
}

impl ChunkedDribbleDelay {
    pub fn new(number_of_chunks: u16, total_duration: u16) -> ChunkedDribbleDelay {
        ChunkedDribbleDelay {
            number_of_chunks,
            total_duration,
        }
    }

    pub fn number_of_chunks(&self) -> u16 {
        self.number_of_chunks
    }

    pub fn total_duration(&self) -> u16 {
        self.total_duration
    }
}
