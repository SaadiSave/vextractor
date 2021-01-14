<style>
    .mbc {
        color: darkorange;
    }
    .Mbc {
        color: darkred;
    }
    .mc {
        color: green;
    }
    .Mc {
        color: gold;
    }
</style>
# Releases
## 0.2.0
### <span class="mbc">Minor Breaking Changes
* The `Vextract::vocab` vector can no longer be accessed publicly
### <span class="mc">Minor Changes
* Documentation completed
## 0.2.1
### <span class="mc">Minor Changes
* Changed the way the `Vextract::vocab` vector is rebuilt by the struct methods `Vextract::{pstrip, make_lower, remove_nums}`
# Future versions
## 0.2.2
### <span class="Mc">Major Changes
* Changed the way punctuations are identified and removed so that it can recognise punctuation within a word, like `people's`
## 0.3.0
### <span class="Mbc">Major Breaking Changes
* Expose Vextract directly from `lib.rs`