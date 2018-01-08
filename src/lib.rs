extern crate pest;
extern crate pest_derive;

// This include forces recompiling this source file if the grammar file changes.
// Uncomment it when doing changes to the .pest file
#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("scl.pest");


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
