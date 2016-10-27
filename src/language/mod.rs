pub mod english;
pub mod spanish;

pub struct Language {
    title: String,
    winner: String,
    loser: String,
    tied: String,
    }

impl Language {

    pub fn title(&self) -> &String {
        &self.title
    }

   pub fn winner(&self) -> &String {
        &self.winner
    }

    pub fn loser(&self) -> &String {
        &self.loser
    }

    pub fn tied(&self) -> &String {
        &self.tied
    }

 }
