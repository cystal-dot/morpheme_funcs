-- 商品テーブルを作成
-- init-morpehmeでカラムを追加している
CREATE TABLE product (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    morpheme_array TEXT[]
);