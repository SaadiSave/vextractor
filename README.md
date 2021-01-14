# vextractor
[![Crates.io](https://img.shields.io/crates/v/vextractor?style=for-the-badge)](https://crates.io/crates/vextractor)
[![Crates.io](https://img.shields.io/crates/l/vextractor?style=for-the-badge)](https://github.com/SaadiSave/vextractor/blob/main/LICENSE.md)\
\
`vextractor` is a simple library for extracting the vocabulary of a text file.
## About
`vextractor` works for any language in any script supported by unicode, as long as the language separates words with a unicode space `' '` `(U+20)`.
## Quick Example
```rust
extern crate vextractor;
use vextractor::vex::Vextract;
let x = Vextract::new(
    "somepath/somefile.txt", // file containing the text to be processed
    vec!["EU", "etc.", "i.e.", "e.g."], // Acronyms
    vec!["Germany", "France", "Belgium", "Italy"] // Proper Nouns
);
println!("{}", x.get_pretty_vocab()); // Prints the vocabulary
println!("{}", x.get_sorted_pretty_vocab()); // Sorts, then prints
x.write_to_file("somepath/somefile.txt"); // Writes vocab to a text file
```
## Licence
`vextractor` is licensed under GNU AFFERO GENERAL PUBLIC LICENSE version 3. Please read the `LICENSE.md` file for more information.
