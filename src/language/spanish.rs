use language::Language;

impl Language {
    pub fn spanish() -> Language {
        Language {
            title: "TIC TAC TOE".to_string(),
            winner: "Ganador".to_string(),
            loser: "Perdedor".to_string(),
            tied: "Atado".to_string(),
        }
    }
}
