#![allow(missing_docs)]

#[derive(Debug)]
pub struct StrSplit<'a> {
    remainer: Option<&'a str>,
    delimiter: &'a str,
}

impl<'a> StrSplit<'a> {
    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainer: Some(haystack),
            delimiter,
        }
    }
}

impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        let remainer = self.remainer.as_mut()?;
        if let Some(next_delim) = remainer.find(self.delimiter) {
            let until_delimiter = &remainer[..next_delim];
            *remainer = &remainer[(next_delim + self.delimiter.len())..];
            Some(until_delimiter)
        } else {
            self.remainer.take()
        }
    }
}

#[test]
fn it_works() {
    let haystack = "a b c d ";
    let letters: Vec<&str> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d"]);
}

#[test]
fn tail() {
    let haystack = "a b c d ";
    let letters: Vec<&str> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", ""]);
}
