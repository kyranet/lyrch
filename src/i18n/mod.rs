pub mod definition;
pub mod el_gr;
pub mod en_us;
pub mod es_es;

use std::collections::HashMap;
use std::iter::FromIterator;
lazy_static! {
    pub static ref OUTPUT: HashMap<String, definition::Language> = HashMap::from_iter(
        vec![
            ("el_gr".to_owned(), self::el_gr::OUTPUT),
            ("en_us".to_owned(), self::en_us::OUTPUT),
            ("es_es".to_owned(), self::es_es::OUTPUT),
        ]
        .into_iter()
    );
}
