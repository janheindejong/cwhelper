use std::{
    fs::File,
    io::{self, BufReader, Read},
    path::PathBuf,
};

pub fn english() -> Vec<String> {
    split_string(include_str!("words/english.txt"))
}

pub fn dutch() -> Vec<String> {
    split_string(include_str!("words/dutch.txt"))
}

pub fn italian() -> Vec<String> {
    split_string(include_str!("words/italian.txt"))
}

pub fn from_file(filename: &PathBuf) -> Result<Vec<String>, io::Error> {
    let mut reader = BufReader::new(File::open(filename)?);
    let mut s = String::new();
    reader.read_to_string(&mut s)?;
    let words = split_string(&s);
    Ok(words)
}

fn split_string(s: &str) -> Vec<String> {
    s.lines().map(|x| x.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dutch_has_413938_words() {
        assert_eq!(dutch().len(), 413937);
    }

    #[test]
    fn english_has_466550_words() {
        assert_eq!(english().len(), 466550);
    }

    #[test]
    fn italian_has_95152_words() {
        assert_eq!(italian().len(), 95152);
    }
}
