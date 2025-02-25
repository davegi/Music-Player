pub struct Score {
    p1_score_string: String,
    p2_score_string: String,
}

impl Score {
    pub fn new() -> Self {
        Self {
            p1_score_string: String::from("0"),
            p2_score_string: String::from("0"),
        }
    }

    pub fn update(&mut self) {}
}
