# Releases
## 0.2.0
### Minor Breaking Changes
* The `Vextract::vocab` vector can no longer be accessed publicly
### Minor Changes
* Documentation completed
## 0.2.1
### Minor Changes
* Changed the way the `Vextract::vocab` vector is rebuilt by the struct methods `Vextract::{pstrip, make_lower, remove_nums}`
# Future versions
## 0.2.2
* Changed the way punctuations are identified and removed so that it can recognise punctuation within a word, like `people's`
## 0.3.0
* Expose Vextract directly from `lib.rs`