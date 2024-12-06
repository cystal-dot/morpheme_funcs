# Research Directory

このディレクトリでは、morpheme-funcs拡張機能の形態素ベースの類似度計算機能について、実装方法とパフォーマンスを比較検証しています。

## 拡張機能の主な特徴

### シンプルな類似度計算
- `calculate_morpheme_score(text, text)`関数一つで形態素ベースの類似度が計算可能
- 実装例: `SELECT calculate_morpheme_score('商品A', '商品B')`

## パフォーマンス検証

### 検証内容
- `extension.sql`: 拡張機能による直接的な類似度計算
  - `calculate_morpheme_score`関数を使用した素直な実装
- `pure_sql.sql`: SQLで同等の機能を実装した場合
  - 形態素解析は他の拡張機能（例：`mecab_dict`）でも実現可能ですが、この検証では自作の`to_morpheme_array`関数を使用

### 高速化の検討
形態素解析結果をテーブルに持たせることで、類似度計算の負荷を軽減できます：
```sql
-- 形態素配列カラムとGINインデックスを活用した例
SELECT id, name, calculate_morpheme_score(name, '検索語') as score
FROM products
WHERE morphemes && to_morpheme_array('検索語')
ORDER BY score DESC;