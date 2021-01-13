// Vextractor: A simple rust library for vocabulary processing
// Copyright (C) 2020 Saadi Save

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

#![allow(dead_code)]

use fs::read_to_string;
use std::collections::HashSet;
use std::fs;

fn contains_str(vec: &Vec<String>, s: &String) -> bool {
    vec.iter().any(|x| x == s)
}

fn char_index(s: &String, c: char) -> usize {
    s.chars().position(|ch| ch == c).unwrap()
}

/// Struct that holds all the variables required for deriving the vocabulary
/// of a text file.
#[derive(Clone)]
pub struct Vextract {
    punc: String,
    plist: Vec<String>,
    alist: Vec<String>,
    voc: HashSet<String>,
    vocab: Vec<String>,

    /// Holds the orignal content of the text file as a [`String`] for direct 
    /// access
    pub text: String,
}

impl Vextract {
    /// Initialise a new [`Vextract`] struct.
    ///
    /// ## Quick Example
    ///
    /// ```no_run
    /// let x = Vextract::new(
    /// "somepath/somefile.txt", // file containing the text to be processed
    /// vec!["EU", "etc.", "i.e.", "e.g."], // Acronyms
    /// vec!["Germany", "France", "Belgium", "Italy"] // Proper Nouns
    /// );
    /// ```
    pub fn new(filepath: &str, al: Vec<&str>, pl: Vec<&str>) -> Vextract {
        let mut vset: HashSet<String> = HashSet::new();
        let ftext = read_to_string(filepath).expect("unable to read file");

        for i in ftext.split('\n') {
            for j in i.split(' ') {
                vset.insert(j.to_string());
            }
        }

        let mut tmp = Vextract {
            punc: "!\"#$%&\'()*+,-./:;<=>?@[\\]^_`{|}~“”„‚‘’（）".to_string(),
            plist: pl.iter().map(|s| s.clone().to_string()).collect(),
            alist: al.iter().map(|s| s.clone().to_string()).collect(),
            voc: vset,
            text: ftext,
            vocab: Vec::new(),
        };
        tmp.pstrip();
        tmp.make_lower();
        tmp.remove_nums();

        tmp
    }

    /// Strip puctuation from the words that aren't completely processed yet.
    ///
    /// ## Quick Example
    ///
    /// ```no_run
    /// x.pstrip(); // x is a Vextract struct
    /// ```
    ///
    /// ## Notes
    ///
    /// While this method is called by default in the initialiser, it can be
    /// used to remove punctuation added post-initialisation by the
    /// [`Vextract::add_punctuation`] method.
    pub fn pstrip(&mut self) {
        let mut tmp: HashSet<String> = HashSet::new();

        for i in self.voc.iter() {
            let mut cpunc = true;
            let mut j = i.clone();

            while cpunc {
                let mut x = (j.chars().count() as i64) - 1;
                if x < 0 {
                    x = 0;
                }

                let y = x as usize;

                if contains_str(&self.alist, i) {
                    tmp.insert((*i).to_string());
                } else {
                    if self.punc.contains(j.chars().nth(y).unwrap_or('0')) {
                        j.pop();
                        continue;
                    }
                    if self.punc.contains(j.chars().nth(0).unwrap_or('0')) {
                        let b = j.chars().nth(0).unwrap();
                        j = j.replace(b, "");
                        continue;
                    }
                }

                cpunc = false;
            }

            tmp.insert(j);
        }

        self.voc.clear();
        self.voc.extend(tmp.clone());
        self.vocab.clear();
        self.vocab.extend(tmp);
    }

    fn make_lower(&mut self) {
        let mut tmp: HashSet<String> = HashSet::new();

        for i in self.voc.iter() {
            let j = i.clone();

            if contains_str(&self.plist, &j) || contains_str(&self.alist, &j) {
                tmp.insert(j);
            } else {
                tmp.insert(j.to_lowercase());
            }
        }

        self.voc.clear();
        self.voc.extend(tmp.clone());
        self.vocab.clear();
        self.vocab.extend(tmp);
    }

    fn remove_nums(&mut self) {
        let mut tmp: HashSet<String> = HashSet::new();

        for i in self.voc.iter() {
            let j = i.clone();
            match j.parse::<f64>() {
                Ok(_s) => (),
                Err(_s) => {
                    tmp.insert(j);
                }
            }
        }

        self.voc.clear();
        self.voc.extend(tmp.clone());
        self.vocab.clear();
        self.vocab.extend(tmp);
    }

    /// Adds punctuation to the punctuation string in the default struct.
    /// 
    /// ## Quick Example
    /// You might want to remove the ideographic full-stop (period) used
    /// in CJK languages `。` `(U+3002)`. Do:
    /// ```no_run
    /// x.add_punctuation("。"); // x is a Vextract struct
    /// x.pstrip(); // removes the newly added punctuation character
    /// ```
    /// 
    /// ## Notes
    /// 
    /// Ensure that the character is enclosed by `""` when using the function.
    /// This is because the function uses `&str` as the data type. This may
    /// be used to add multiple punctuation characters at once:
    /// ```no_run
    /// x.add_punctuation("。«»");
    /// ```
    /// However, this is not recommended for the sake of style. Instead, use
    /// ```no_run
    /// x.add_punctuation("。");
    /// x.add_punctuation("«");
    /// x.add_punctuation("»");
    /// ```
    pub fn add_punctuation(&mut self, s: &str) {
        self.punc += s;
    }

    /// Returns the vocabulary as a [`Vec<String>`] containing each word as an 
    /// element.
    pub fn get_vocab(&self) -> Vec<String> {
        self.vocab.clone()
    }

    /// Returns the vocabulary as a [`String`] with each word on a new line.
    pub fn get_pretty_vocab(&self) -> String {
        let mut x = String::new();
        for y in self.vocab.clone() {
            x += format!("{}\n", y).as_str();
        }

        x
    }

    /// Returns the vocabulary as a [`Vec<String>`] containing each word as an 
    /// element, and sorted in ascending order.
    pub fn get_sorted_vocab(&self) -> Vec<String> {
        let mut x = self.vocab.clone();
        x.sort();
        x
    }

    /// Returns the vocabulary as a [`String`] with each word on a new line,
    /// and sorted in ascending order.
    pub fn get_sorted_pretty_vocab(&self) -> String {
        let mut x = String::new();
        let mut y = self.vocab.clone();
        y.sort();

        for y in y {
            x += format!("{}\n", y).as_str();
        }

        x
    }

    /// Returns the number of words in the vocabulary.
    pub fn get_len(&self) -> u32 {
        self.vocab.len() as u32
    }

    /// Writes the output of [`Vextract::get_sorted_pretty_vocab`] to a file.
    pub fn write_to_file(&self, filepath: &str) {
        fs::write(filepath, self.get_sorted_pretty_vocab()).expect("Err");
    }
}
