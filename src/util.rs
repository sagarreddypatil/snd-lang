use std::collections::HashMap;
use std::sync::Mutex;

use lazy_static::lazy_static;

pub fn leak(s: &str) -> &'static str {
    Box::leak(s.to_string().into_boxed_str())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Symbol {
    pub name: &'static str,
    pub index: usize,
}

lazy_static! {
    static ref SYMBOLS: Mutex<Vec<String>> = Mutex::new(vec![]);
    static ref SYMBOLS_MAP: Mutex<HashMap<&'static str, Symbol>> = Mutex::new(HashMap::new());
}

fn static_symbol(symbol: &Symbol) -> &'static Symbol {
    // safe because we never remove symbols from the map
    unsafe { std::mem::transmute::<&Symbol, &'static Symbol>(symbol) }
}

impl Symbol {
    pub fn new(s: &str) -> &'static Symbol {
        let mut symbols = SYMBOLS.lock().unwrap();
        let mut symbols_map = SYMBOLS_MAP.lock().unwrap();

        if let Some(ref symbol) = symbols_map.get(s) {
            return static_symbol(symbol);
        }

        let name = s.to_string();
        let index = symbols.len();

        symbols.push(name);
        let name = symbols.last().unwrap().as_str();

        // safe because we never remove symbols from the vec
        let name = unsafe { std::mem::transmute::<&str, &'static str>(name) };

        let symbol = Symbol { name, index };
        symbols_map.insert(name, symbol);
        let symbol = symbols_map.get(name).unwrap();

        static_symbol(symbol)
    }
}
