use language::Language;

impl Language {
    pub fn english() -> Language {
        Language {
            title: "TIC TAC TOE".to_string(),
            winner: "Winner".to_string(),
            loser: "Loser".to_string(),
            tied: "Tied".to_string(),
        }
    }
}
