// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut counter: HashMap<&str, isize> = HashMap::new();

    for n in note {
        *counter.entry(n).or_insert(0) += 1;
    }

    for n in magazine {
        match counter.get_mut(n) {
            Some(1) => {
                counter.remove(n);

                if counter.is_empty() {
                    return true;
                }
            }
            Some(t) => *t -= 1,
            _ => {}
        };
    }

    false
}
