use rustc_hash::FxHashSet;
use std::sync::Once;

use mecab::Tagger;
use pgrx::prelude::*;

::pgrx::pg_module_magic!();

static mut TAGGER: Option<Tagger> = None;
static INIT: Once = Once::new();

fn get_tagger() -> &'static mut Tagger {
    unsafe {
        INIT.call_once(|| {
            TAGGER = Some(Tagger::new("-O wakati"));
        });
        TAGGER.as_mut().unwrap()
    }
}

// キーワード同士の類似度を算出
// 類似度は形態素の一致率で算出
#[pg_extern]
fn calculate_similar_score(keyword: String, target: String) -> f64 {
    // keywordとtargetを形態素解析し、それぞれの形態素をセットに格納
    let keyword_morphemes: FxHashSet<String> = wakati_to_set(&keyword);
    let target_morphemes: FxHashSet<String> = wakati_to_set(&target);

    // 共通形態素の数を計算
    let common_count = keyword_morphemes.intersection(&target_morphemes).count();
    let total_count = keyword_morphemes.union(&target_morphemes).count();

    // 共通形態素の数をもとにスコアを計算
    if total_count == 0 {
        0.0
    } else {
        common_count as f64 / total_count as f64
    }
}

fn wakati(input: &str) -> Option<String> {
    // MeCabのTaggerを初期化
    let tagger = get_tagger();

    if tagger.parse_nbest_init(input) {
        tagger.next().map(|result| result.to_string())
    } else {
        None // 初期化が失敗した場合
    }
}

// 形態素解析結果をセットに変換
fn wakati_to_set(input: &str) -> FxHashSet<String> {
    let parsed_result = wakati(input);
    if let Some(result) = parsed_result {
        result.split_whitespace().map(|s| s.to_string()).collect()
    } else {
        FxHashSet::default()
    }
}

// 形態素解析して、形態素の配列を返す
#[pg_extern]
fn to_morpheme_array(input: &str) -> Vec<String> {
    let morphemes_set: FxHashSet<String> = wakati_to_set(input);

    let mut morphemes_vec: Vec<String> = morphemes_set.into_iter().collect();

    if morphemes_vec.is_empty() {
        morphemes_vec.push("".to_string());
    }

    morphemes_vec.sort();

    morphemes_vec
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_calculate_similar_score() {
        assert_eq!(
            0.0,
            crate::calculate_similar_score("学生です".to_string(), "".to_string())
        );

        assert_eq!(
            0.5,
            crate::calculate_similar_score("学生です".to_string(), "学生".to_string())
        );

        assert_eq!(
            1.0,
            crate::calculate_similar_score("学生です".to_string(), "学生です".to_string())
        );
    }

    #[pg_test]
    fn test_to_morpheme_array() {
        assert_eq!(vec![""], crate::to_morpheme_array(""));

        assert_eq!(
            vec!["です".to_string(), "学生".to_string()],
            crate::to_morpheme_array("学生です")
        );
    }

    #[pg_test]
    fn test_wakati() {
        assert_eq!(Some("学生 です".to_string()), crate::wakati("学生です"));
        assert_eq!(None, crate::wakati(""));
    }

    #[pg_test]
    fn test_wakati_to_set() {
        // 基本的な変換テスト
        let set = crate::wakati_to_set("私は学生です");
        assert!(set.contains("私"));
        assert!(set.contains("は"));
        assert!(set.contains("学生"));
        assert!(set.contains("です"));

        // 重複する単語のテスト
        let set2 = crate::wakati_to_set("私は私です");
        assert_eq!(set2.len(), 3); // 「私」は1回だけカウントされるべき
    }
}

/// This module is required by `cargo pgrx test` invocations.
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    #[must_use]
    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
