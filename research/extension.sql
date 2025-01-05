DROP INDEX IF EXISTS idx_product_name;
CREATE INDEX idx_product_name ON product USING btree (name); 
EXPLAIN ANALYZE
SELECT 
  name,
  morpheme_array,
  calculate_similar_score(name, 'ドリル') as score
FROM product
ORDER BY
  score DESC
;