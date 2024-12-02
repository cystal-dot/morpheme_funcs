use std::collections::HashSet;

use mecab::Tagger;
use pgrx::prelude::*;

::pgrx::pg_module_magic!();

// キーワード同士の類似度を算出
// 類似度は形態素の一致率で算出
#[pg_extern]
fn calculate_morpheme_score(keyword: String, target: String) -> f64 {
    // keywordとtargetを形態素解析し、それぞれの形態素をセットに格納
    let keyword_morphemes: HashSet<String> = wakati_to_set(&keyword);
    let target_morphemes: HashSet<String> = wakati_to_set(&target);

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

fn wakati(input: &str) -> String {
    // MeCabのTaggerを初期化
    let mut tagger = Tagger::new("-O wakati");

    tagger.parse_nbest_init(input);

    // 最初の解析結果を取得
    if let Some(res) = tagger.next() {
        res.to_string()
    } else {
        String::new()
    }
}

// 形態素解析結果をセットに変換
fn wakati_to_set(input: &str) -> HashSet<String> {
    let parsed_result = wakati(input);
    parsed_result
        .split_whitespace() // 形態素をスペース区切りで分割
        .map(|morpheme| morpheme.to_string())
        .collect()
}

// 形態素解析して、形態素の配列を返す
#[pg_extern]
fn to_morpheme_array(input: &str) -> Vec<String> {
    let morphemes_set: HashSet<String> = wakati_to_set(input);

    let mut morphemes_vec: Vec<String> = morphemes_set.into_iter().collect();

    morphemes_vec.sort();

    morphemes_vec
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_calculate_morpheme_score() {
        assert_eq!(
            0.42857142857142855, // 結果をそのままいれてるだけ。この場合は返却値のアサーション自体ナンセンス
            crate::calculate_morpheme_score("keyword1".to_string(), "target2".to_string())
        );
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
