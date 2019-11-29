use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::ops::Add;
use std::fmt::{Display, Formatter, Debug};

struct Play {
    name: String,
    chapters: Vec<String>,
}

struct SanitizeError;

impl Error for SanitizeError {}

impl Display for SanitizeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "The file doesn't have headers and footers (less than 4 chapter)")
    }
}
// When main -> Result<_, Error> it's Debug that is called to be printed
impl Debug for SanitizeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

struct SourceError{
    message: String,
    cause: Option<Box<dyn Error>>
}

impl SourceError {

    fn new(message: String, cause: Option<Box<dyn Error>>) -> SourceError {
        SourceError{
            message,
            cause
        }
    }
}

impl Display for SourceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error for source play")?;
        match self.cause {
            Some(ref cause) => {
                write!(f, "\n")?;
                Display::fmt(cause, f)
            },
            None => Ok(())
        }
    }
}

impl Debug for SourceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Error for SourceError {

}


fn load_play(play_name: String) -> Result<Play, Box<dyn Error>> {
    let file_name = play_name.to_lowercase().replace(' ', "").add(".txt");
    let file = File::open(&file_name).map_err(|err| SourceError::new(format!("Can't read file {}", &file_name), Some(Box::new(err))))?;
    let text = get_text(file)?;
    let splited = split_chapter(&text)?;
    let chapters = remove_header(splited)?;
    Ok(Play {
        name: play_name,
        chapters,
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

fn remove_header(chapters: Vec<String>) -> Result<Vec<String>, Box<dyn Error>> {
    if chapters.len() < 4 {
        Err(SanitizeError{}.into())
    } else {
        Ok(chapters[2..chapters.len() - 1].iter().cloned().collect())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let play = load_play("Romeo and Juliette no text".to_string())?;
    println!("Loaded {} with {} chapters", play.name, play.chapters.len());
    Ok(())
}