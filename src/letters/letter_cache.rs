use super::letter::{self, Letter};
use std::collections::{hash_map::Keys, HashMap};

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
        if let None = LETTER_CACHE {
            build_cahche();
        }
    }
}

pub fn available_letters() -> Vec<char> {
    build_cahche();
    unsafe {
        if let Some(letter_cahce) = &LETTER_CACHE {
            letter_cahce.keys().cloned().collect()
        } else {
            panic!()
        }
    }
}

pub fn is_available_letter(letter: &char) -> bool {
    build_cahche();
    unsafe {
        if let Some(letter_cahce) = &LETTER_CACHE {
            letter_cahce.contains_key(letter)
        } else {
            false
        }
    }
}

fn build_cahche() {
    let mut letter_cache = HashMap::new();

    letter_cache.insert(
        'A',
        letter::Letter::new(
            [
                vec![],
                vec![false, false, true, false, false],
                vec![false, true, false, true, false],
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
                vec![false, false, true, false, false],
                vec![false, false, true, false, false],
                vec![false, false, true, false, false],
                vec![false, false, true, false, false],
                vec![false, false, true, false, false],
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
                vec![false,false,false,false,true],
                vec![false,false,false,false,true],
                vec![false,false,false,false,true],
                vec![false,false,false,false,true],
                vec![true,false,false,false,true],
                vec![true,false,false,false,true],
                vec![false, true, true, true, false],
            ],
            5,
        ),
    );

    unsafe {
        LETTER_CACHE = Some(letter_cache);
    }
}
