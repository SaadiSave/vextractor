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

#![crate_type = "lib"]

//! `vextractor` is a simple library for extracting the vocabulary of a text
//! file.
//!
//! ## About
//!
//! `vextractor` works for any language in any script supported by unicode, as
//! long as the language separates words with a unicode space `' '` `(U+20)`.
//!
//! ## Quick Example
//!
//! ```no_run
//! extern crate vextractor;
//! use vextractor::vex::Vextract;
//!
//! let x = Vextract::new(
//!     "somepath/somefile.txt", // inputfile
//!     vec!["EU", "etc.", "i.e.", "e.g."], // Acronyms
//!     vec!["Germany", "France", "Belgium", "Italy"] // Proper Nouns
//! );
//!
//! println!("{}", x.get_pretty_vocab()); // Prints the vocabulary
//! println!("{}", x.get_sorted_pretty_vocab()); // Sorts, then prints
//!
//! x.write_to_file("somepath/somefile.txt"); // Writes vocab to a text file
//! ```
//!
//! ## Licence
//!
//! `vextractor` is licensed under GNU AFFERO GENERAL PUBLIC LICENSE version 3
//! . Please read the `LICENSE.md` file for more information.

pub mod vex;
