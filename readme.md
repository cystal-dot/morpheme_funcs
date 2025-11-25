# morpheme-funcs

形態素解析を使って文字列の類似度を計算するためのPostgreSQL拡張機能です。  
pgrxを使用してRustで実装されています。

## 機能

- `calculate_similar_score(text, text)`: 2つの文字列の形態素解析による類似度スコアを計算します
- `to_morpheme_array(text)`：文字列を形態素解析して配列に変換します

## セットアップ
DevContainerで開く

下記コマンドでpgrxのinstallとbuild
```
sudo apt-get update && sudo apt-get install -y bison flex
cargo install cargo-pgrx
cargo pgrx init
cargo build
```

```
cargo pgrx run 
create extension morpheme_funcs;
```

### テスト用データをinsertする場合
16000件の商品データをproductテーブルにinsertするためのスクリプトを用意しています  
cargo pgrx run でDBが起動します。DBを起こした後にスクリプトを動かしてください。
```
cd sql
./apply_sqls.sh
```

## 使用例
```sql
-- テキスト同士の類似度スコアを計算
product=# SELECT calculate_similar_score('大ねじ小ねじ', 'ねじ');
 calculate_similar_score 
--------------------------
       0.3333333333333333


-- テキストを形態素解析して配列に変換
product=# SELECT to_morpheme_array('形態素解析機能');
 to_morpheme_array  
--------------------
 {形態素,機能,解析}
```

## パフォーマンス
`research`ディレクトリには、拡張機能のパフォーマンス評価に関する実験データが含まれています。
