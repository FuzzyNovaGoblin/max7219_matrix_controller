use super::symbol::{self, Symbol};
use std::collections::HashMap;

static mut LETTER_CACHE: Option<HashMap<char, symbol::Symbol>> = None;

pub fn get_symbol(symbol: &char) -> Option<symbol::Symbol> {
    init_cache();
    unsafe {
        match &LETTER_CACHE {
            Some(symbol_cache) => symbol_cache.get(symbol).cloned(),
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
        if let Some(symbol_cahce) = LETTER_CACHE.clone() {
            symbol_cahce.keys().cloned().collect()
        } else {
            panic!()
        }
    }
}
pub fn cache_values() -> Vec<Symbol> {
    init_cache();
    unsafe {
        if let Some(symbol_cahce) = &LETTER_CACHE {
            symbol_cahce.values().cloned().collect()
        } else {
            panic!()
        }
    }
}

pub fn is_available_symbol(symbol: &char) -> bool {
    init_cache();
    unsafe {
        if let Some(symbol_cahce) = &LETTER_CACHE {
            symbol_cahce.contains_key(symbol)
        } else {
            false
        }
    }
}

fn build_cache() {
    let mut symbol_cache = HashMap::new();

    symbol_cache.insert(
        'A',
        symbol::Symbol::new(
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

    symbol_cache.insert(
        'B',
        symbol::Symbol::new(
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

    symbol_cache.insert(
        'C',
        symbol::Symbol::new(
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
    symbol_cache.insert(
        'D',
        symbol::Symbol::new(
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

    symbol_cache.insert(
        'E',
        symbol::Symbol::new(
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
    symbol_cache.insert(
        'F',
        symbol::Symbol::new(
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
    symbol_cache.insert(
        'G',
        symbol::Symbol::new(
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

    symbol_cache.insert(
        'H',
        symbol::Symbol::new(
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

    symbol_cache.insert(
        'I',
        symbol::Symbol::new(
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

    symbol_cache.insert(
        'J',
        symbol::Symbol::new(
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
    symbol_cache.insert(
        'K',
        symbol::Symbol::new(
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
    symbol_cache.insert(
        'L',
        symbol::Symbol::new(
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

    symbol_cache.insert(
        'M',
        symbol::Symbol::new(
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

    symbol_cache.insert(
        'N',
        symbol::Symbol::new(
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
    symbol_cache.insert(
        'O',
        symbol::Symbol::new(
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
    symbol_cache.insert(
        'P',
        symbol::Symbol::new(
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

    symbol_cache.insert(
        'Q',
        symbol::Symbol::new(
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

    symbol_cache.insert(
        'R',
        symbol::Symbol::new(
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

    symbol_cache.insert(
        'S',
        symbol::Symbol::new(
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
    symbol_cache.insert(
        'T',
        symbol::Symbol::new(
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
    symbol_cache.insert(
        'U',
        symbol::Symbol::new(
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
    symbol_cache.insert(
        'V',
        symbol::Symbol::new(
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
    symbol_cache.insert(
        'W',
        symbol::Symbol::new(
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
    symbol_cache.insert(
        'X',
        symbol::Symbol::new(
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
    symbol_cache.insert(
        'Y',
        symbol::Symbol::new(
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
    symbol_cache.insert(
        'Z',
        symbol::Symbol::new(
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
    symbol_cache.insert(
        ' ',
        symbol::Symbol::new(
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
    symbol_cache.insert(
        '.',
        symbol::Symbol::new(
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
    symbol_cache.insert(
        '?',
        symbol::Symbol::new(
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
    symbol_cache.insert(
        '!',
        symbol::Symbol::new(
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
    symbol_cache.insert(
        ',',
        symbol::Symbol::new(
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

    symbol_cache.insert(
        ';',
        symbol::Symbol::new(
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

    symbol_cache.insert(
        ':',
        symbol::Symbol::new(
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
        LETTER_CACHE = Some(symbol_cache);
    }
}
