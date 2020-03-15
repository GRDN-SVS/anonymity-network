use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Vote {
    pub election_id: u32,
    pub voter_id: u32,
    pub option: u32,
}