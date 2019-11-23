use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::ops::Add;

struct Play {
    name: String,
    chapters: Vec<String>
}

fn load_play(play_name: String) -> Result<Play, Box<dyn Error>> {
    let file_name = play_name.to_lowercase().replace(' ',"").add(".txt");
    let file = File::open(file_name)?;
    let text = get_text(file)?;
    let splited = split_chapter(&text)?;
    let chapters = remove_header(splited)?;
    Ok(Play{
        name: play_name,
        chapters
    })
}

fn split_chapter(text: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let regex = regex::Regex::new(r"<<[^>]+>>")?;
    Ok(regex.split(text).map(|chapter| String::from(chapter.trim())).collect())
}

fn get_text(mut play_file: File) -> Result<String, Box<dyn Error>> {
    let mut result = String::new();
    play_file.read_to_string(&mut result)?;
    Ok(result)
}

fn remove_header(mut chapters: Vec<String>) -> Result<Vec<String>, Box<dyn Error>> {
    Ok(chapters[2..chapters.len()-1].iter().cloned().collect())
}

fn main() -> Result<(), Box<dyn Error>> {
    let play = load_play("Romeo and Juliette".to_string())?;
    println!("Loaded {} with {} chapters", play.name, play.chapters.len());
    Ok(())
}