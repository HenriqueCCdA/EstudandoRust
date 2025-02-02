#[derive(Debug)]
struct TaylorSwiftSong {
    title: String,
    release_year: u32,
    duration_secs: u32,
}

impl TaylorSwiftSong {
    fn new(title: String, release_year: u32, duration_secs: u32) -> Self {
        Self {
            title,
            release_year,
            duration_secs,
        }
    }
}

impl TaylorSwiftSong {

    // Immutable struct value (self parameter taker owernship)
    fn display_song_info(&self) {
        println!("Title: {}", self.title);
        println!("Years since Release: {}", self.years_since_release());
        println!("Duration: {} seconds", self.duration_secs);
    }

    // Mutable struct value(self parameter takes ownership, hasa permission to mutate)
    fn double_length(&mut self) {
        self.duration_secs = self.duration_secs * 2;
    }

    fn is_longer_than(&self, other: &TaylorSwiftSong) -> bool {
        self.duration_secs > other.duration_secs
    }

    fn years_since_release(&self) -> u32 {
        2025 - self.release_year
    }
}


fn main() {
    let blank_space = TaylorSwiftSong::new(String::from("Blank Space"), 2014, 231);

    blank_space.display_song_info()
}
