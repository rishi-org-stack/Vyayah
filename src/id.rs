use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
struct ID {}

impl ID {
    fn new() -> ID {
        ID {}
    }
}
