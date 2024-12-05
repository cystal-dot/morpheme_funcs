# morpheme-funcs

形態素解析を使って文字列の類似度を計算するためのPostgreSQL拡張機能です。  
pgrxを使用してRustで実装されています。

## 機能

- `calculate_similar_score(text, text)`: 2つの文字列の形態素解析による類似度スコアを計算します
- `to_morpheme_array(text)`：文字列を形態素解析して配列に変換します

## 必要要件

- PostgreSQL 14以上
- Rust
- cargo-pgrx
- Docker (開発環境用)

## セットアップ

### Dockerを使用する場合

1. Dockerコンテナを起動:
```bash
cd docker
docker compose up -d
docker compose exec -u postgres postgres bash
```


# テスト実行
```bash
cargo pgrx run pg14
drop extension if exists morpheme_funcs cascade;
create extension morpheme_funcs;
```

## 使用例

```sql
-- テキスト同士の類似度スコアを計算
product=# SELECT calculate_similar_score('大ねじ小ねじ', 'ねじ');
 calculate_similar_score 
--------------------------
       0.3333333333333333


-- テキストを形態素解析して配列に変換
product=# select to_morpheme_array('形態素解析機能');
 to_morpheme_array  
--------------------
 {形態素,機能,解析}
```

## パフォーマンス
`calculate_similar_score` は、同等の機能を純SQLで実装した場合と比較して高速に動作します。  
実行計画の詳細な比較や具体的なクエリ例については、researchディレクトリを参照してください。  
