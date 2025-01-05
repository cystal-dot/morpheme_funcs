-- 商品テーブルを作成
DROP TABLE IF EXISTS product;
CREATE TABLE product (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    morpheme_array TEXT[]
);