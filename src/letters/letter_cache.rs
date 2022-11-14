use super::letter::{self, Letter};
use std::collections::HashMap;

static mut LETTER_CACHE: Option<HashMap<char, letter::Letter>> = None;

pub fn get_letter(letter: &char) -> Option<letter::Letter> {
    init_cache();
    unsafe {
        match &LETTER_CACHE {
            Some(letter_cache) => letter_cache.get(letter).cloned(),
            None => None,
        }
    }
}

pub fn init_cache() {
    unsafe {
        if LETTER_CACHE.is_none() {
            build_cache();
        }
    }
}

pub fn cache_keys() -> Vec<char> {
    init_cache();
    unsafe {
        if let Some(letter_cahce) = LETTER_CACHE.clone() {
            letter_cahce.keys().cloned().collect()
        } else {
            panic!()
        }
    }
}
pub fn cache_values() -> Vec<Letter> {
    init_cache();
    unsafe {
        if let Some(letter_cahce) = &LETTER_CACHE {
            letter_cahce.values().cloned().collect()
        } else {
            panic!()
        }
    }
}

pub fn is_available_letter(letter: &char) -> bool {
    init_cache();
    unsafe {
        if let Some(letter_cahce) = &LETTER_CACHE {
            letter_cahce.contains_key(letter)
        } else {
            false
        }
    }
}

fn build_cache() {
    let mut letter_cache = HashMap::new();

    letter_cache.insert(
        'A',
        letter::Letter::new(
            [
                vec![],
                vec![false, false, true],
                vec![false, true, false, true],
                vec![true, false, false, false, true],
                vec![true, true, true, true, true],
                vec![true, false, false, false, true],
                vec![true, false, false, false, true],
                vec![true, false, false, false, true],
            ],
            5,
        ),
    );

    letter_cache.insert(
        'B',
        letter::Letter::new(
            [
                vec![],
                vec![true, true, true, true],
                vec![true, false, false, false, true],
                vec![true, false, false, false, true],
                vec![true, true, true, true],
                vec![true, false, false, false, true],
                vec![true, false, false, false, true],
                vec![true, true, true, true],
            ],
            5,
        ),
    );

    letter_cache.insert(
        'C',
        letter::Letter::new(
            [
                vec![],
                vec![false, true, true, true],
                vec![true, false, false, false, true],
                vec![true],
                vec![true],
                vec![true],
                vec![true, false, false, false, true],
                vec![false, true, true, true],
            ],
            5,
        ),
    );
    letter_cache.insert(
        'D',
        letter::Letter::new(
            [
                vec![],
                vec![true, true, true, true],
                vec![true, false, false, false, true],
                vec![true, false, false, false, true],
                vec![true, false, false, false, true],
                vec![true, false, false, false, true],
                vec![true, false, false, false, true],
                vec![true, true, true, true],
            ],
            5,
        ),
    );

    letter_cache.insert(
        'E',
        letter::Letter::new(
            [
                vec![],
                vec![true, true, true, true, true],
                vec![true],
                vec![true],
                vec![true, true, true],
                vec![true],
                vec![true],
                vec![true, true, true, true, true],
            ],
            5,
        ),
    );
    letter_cache.insert(
        'F',
        letter::Letter::new(
            [
                vec![],
                vec![true, true, true, true, true],
                vec![true],
                vec![true],
                vec![true, true, true],
                vec![true],
                vec![true],
                vec![true],
            ],
            5,
        ),
    );
    letter_cache.insert(
        'G',
        letter::Letter::new(
            [
                vec![],
                vec![true, true, true, true, true],
                vec![true, false, false, false, true],
                vec![true],
                vec![true],
                vec![true, false, true, true, true],
                vec![true, false, false, false, true],
                vec![true, true, true, true, true],
            ],
            5,
        ),
    );

    letter_cache.insert(
        'H',
        letter::Letter::new(
            [
                vec![],
                vec![true, false, false, false, true],
                vec![true, false, false, false, true],
                vec![true, false, false, false, true],
                vec![true, true, true, true, true],
                vec![true, false, false, false, true],
                vec![true, false, false, false, true],
                vec![true, false, false, false, true],
            ],
            5,
        ),
    );

    letter_cache.insert(
        'I',
        letter::Letter::new(
            [
                vec![],
                vec![true, true, true, true, true],
                vec![false, false, true],
                vec![false, false, true],
                vec![false, false, true],
                vec![false, false, true],
                vec![false, false, true],
                vec![true, true, true, true, true],
            ],
            5,
        ),
    );

    letter_cache.insert(
        'J',
        letter::Letter::new(
            [
                vec![],
                vec![false, false, false, false, true],
                vec![false, false, false, false, true],
                vec![false, false, false, false, true],
                vec![false, false, false, false, true],
                vec![true, false, false, false, true],
                vec![true, false, false, false, true],
                vec![false, true, true, true],
            ],
            5,
        ),
    );
    letter_cache.insert(
        'K',
        letter::Letter::new(
            [
                vec![],
                vec![true, false, false, false, true],
                vec![true, false, false, false, true],
                vec![true, false, false, true],
                vec![true, true, true],
                vec![true, false, false, true],
                vec![true, false, false, false, true],
                vec![true, false, false, false, true],
            ],
            5,
        ),
    );
    letter_cache.insert(
        'L',
        letter::Letter::new(
            [
                vec![],
                vec![true],
                vec![true],
                vec![true],
                vec![true],
                vec![true],
                vec![true],
                vec![true, true, true, true, true],
            ],
            5,
        ),
    );

    letter_cache.insert(
        'M',
        letter::Letter::new(
            [
                vec![],
                vec![true, false, false, false, true],
                vec![true, true, false, true, true],
                vec![true, false, true, false, true],
                vec![true, false, false, false, true],
                vec![true, false, false, false, true],
                vec![true, false, false, false, true],
                vec![true, false, false, false, true],
            ],
            5,
        ),
    );

    letter_cache.insert(
        'N',
        letter::Letter::new(
            [
                vec![],
                vec![true, false, false, false, true],
                vec![true, false, false, false, true],
                vec![true, true, false, false, true],
                vec![true, false, true, false, true],
                vec![true, false, false, true, true],
                vec![true, false, false, false, true],
                vec![true, false, false, false, true],
            ],
            5,
        ),
    );
    letter_cache.insert(
        'O',
        letter::Letter::new(
            [
                vec![],
                vec![false, true, true, true],
                vec![true, false, false, false, true],
                vec![true, false, false, false, true],
                vec![true, false, false, false, true],
                vec![true, false, false, false, true],
                vec![true, false, false, false, true],
                vec![false, true, true, true],
            ],
            5,
        ),
    );
    letter_cache.insert(
        'P',
        letter::Letter::new(
            [
                vec![],
                vec![true, true, true, true],
                vec![true, false, false, false, true],
                vec![true, false, false, false, true],
                vec![true, true, true, true],
                vec![true],
                vec![true],
                vec![true],
            ],
            5,
        ),
    );

    letter_cache.insert(
        'Q',
        letter::Letter::new(
            [
                vec![],
                vec![false, true, true],
                vec![true, false, false, true],
                vec![true, false, false, true],
                vec![true, false, false, true],
                vec![true, false, true, true],
                vec![true, false, false, true],
                vec![false, true, true, false, true],
            ],
            5,
        ),
    );

    letter_cache.insert(
        'R',
        letter::Letter::new(
            [
                vec![],
                vec![true, true, true, true],
                vec![true, false, false, false, true],
                vec![true, false, false, false, true],
                vec![true, true, true, true],
                vec![true, false, true],
                vec![true, false, false, true],
                vec![true, false, false, false, true],
            ],
            5,
        ),
    );

    letter_cache.insert(
        'S',
        letter::Letter::new(
            [
                vec![],
                vec![false, true, true, true],
                vec![true, false, false, false, true],
                vec![true],
                vec![false, true, true, true],
                vec![false, false, false, false, true],
                vec![true, false, false, false, true],
                vec![false, true, true, true],
            ],
            5,
        ),
    );
    letter_cache.insert(
        'T',
        letter::Letter::new(
            [
                vec![],
                vec![true, true, true, true, true],
                vec![false, false, true],
                vec![false, false, true],
                vec![false, false, true],
                vec![false, false, true],
                vec![false, false, true],
                vec![false, false, true],
            ],
            5,
        ),
    );
    letter_cache.insert(
        'U',
        letter::Letter::new(
            [
                vec![],
                vec![true, false, false, false, true],
                vec![true, false, false, false, true],
                vec![true, false, false, false, true],
                vec![true, false, false, false, true],
                vec![true, false, false, false, true],
                vec![true, false, false, false, true],
                vec![false, true, true, true],
            ],
            5,
        ),
    );
    letter_cache.insert(
        'V',
        letter::Letter::new(
            [
                vec![],
                vec![true, false, false, false, true],
                vec![true, false, false, false, true],
                vec![true, false, false, false, true],
                vec![true, false, false, false, true],
                vec![true, false, false, false, true],
                vec![false, true, false, true],
                vec![false, false, true],
            ],
            5,
        ),
    );
    letter_cache.insert(
        'W',
        letter::Letter::new(
            [
                vec![],
                vec![true, false, false, false, true],
                vec![true, false, false, false, true],
                vec![true, false, false, false, true],
                vec![true, false, false, false, true],
                vec![true, false, true, false, true],
                vec![true, false, true, false, true],
                vec![false, true, false, true],
            ],
            5,
        ),
    );
    letter_cache.insert(
        'X',
        letter::Letter::new(
            [
                vec![],
                vec![true, false, false, false, true],
                vec![true, false, false, false, true],
                vec![false, true, false, true],
                vec![false, false, true],
                vec![false, true, false, true],
                vec![true, false, false, false, true],
                vec![true, false, false, false, true],
            ],
            5,
        ),
    );
    letter_cache.insert(
        'Y',
        letter::Letter::new(
            [
                vec![],
                vec![true, false, false, false, true],
                vec![true, false, false, false, true],
                vec![true, false, false, false, true],
                vec![false, true, false, true],
                vec![false, false, true],
                vec![false, false, true],
                vec![false, false, true],
            ],
            5,
        ),
    );
    letter_cache.insert(
        'Z',
        letter::Letter::new(
            [
                vec![],
                vec![true, true, true, true, true],
                vec![false, false, false, false, true],
                vec![false, false, false, true],
                vec![false, false, true],
                vec![false, true],
                vec![true],
                vec![true, true, true, true, true],
            ],
            5,
        ),
    );
    letter_cache.insert(
        ' ',
        letter::Letter::new(
            [
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
            ],
            5,
        ),
    );
    letter_cache.insert(
        '.',
        letter::Letter::new(
            [
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![true, true],
                vec![true, true],
            ],
            2,
        ),
    );
    letter_cache.insert(
        '?',
        letter::Letter::new(
            [
                vec![],
                vec![false, true, true, true],
                vec![true, false, false, false, true],
                vec![false, false, false, false, true],
                vec![false, false, false, true],
                vec![false, false, true],
                vec![],
                vec![false, false, true],
            ],
            5,
        ),
    );
    letter_cache.insert(
        '!',
        letter::Letter::new(
            [
                vec![],
                vec![true],
                vec![true],
                vec![true],
                vec![true],
                vec![true],
                vec![],
                vec![true],
            ],
            1,
        ),
    );
    letter_cache.insert(
        ',',
        letter::Letter::new(
            [
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![false, true],
                vec![true],
            ],
            2,
        ),
    );

    letter_cache.insert(
        ';',
        letter::Letter::new(
            [
                vec![],
                vec![],
                vec![],
                vec![true, true],
                vec![true, true],
                vec![],
                vec![false, true],
                vec![true],
            ],
            2,
        ),
    );

    letter_cache.insert(
        ':',
        letter::Letter::new(
            [
                vec![],
                vec![],
                vec![],
                vec![true, true],
                vec![true, true],
                vec![],
                vec![true, true],
                vec![true, true],
            ],
            2,
        ),
    );

    unsafe {
        LETTER_CACHE = Some(letter_cache);
    }
}
