EXPLAIN ANALYZE
SELECT 
  name,
  morpheme_array,
  calculate_similar_score(name, 'ドリル') as score
FROM product
ORDER BY
  score DESC
;