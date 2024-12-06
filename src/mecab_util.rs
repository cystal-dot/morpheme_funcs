use mecab::Tagger;
use rustc_hash::FxHashSet;
use std::sync::Once;

static mut TAGGER: Option<Tagger> = None;
static INIT: Once = Once::new();

pub fn get_tagger() -> &'static mut Tagger {
    unsafe {
        INIT.call_once(|| {
            TAGGER = Some(Tagger::new("-O wakati"));
        });
        TAGGER.as_mut().unwrap()
    }
}

pub fn wakati(input: &str) -> String {
    let tagger = get_tagger();
    if tagger.parse_nbest_init(input) {
        tagger
            .next()
            .map(|result| result.to_string().trim_end().to_string())
            .unwrap_or_default()
    } else {
        String::new()
    }
}

pub fn wakati_to_set(input: &str) -> FxHashSet<String> {
    let parsed_result = wakati(input);
    parsed_result
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}
