mod player;

fn main() {
    player::play_movie("Snatch.mp4");
    player::play_audio("rhcp.mp3");
    clean::perform_cleanup();
    clean::files::clean_files();
}

mod clean {
    pub fn perform_cleanup() {
        println!("Cleaning HDD...");
    }

    pub mod files {
        pub fn clean_files() {
            println!("Removing unused files...");
        }
    }
}
