DROP INDEX IF EXISTS idx_product_morpheme_array;
CREATE INDEX idx_product_morpheme_array ON product USING gin(morpheme_array);
EXPLAIN ANALYZE
SELECT 
  name,
  morpheme_array, -- 予めテーブルに格納しておいた形態素配列
  to_morpheme_array('ドリル') as keyword_morpheme_array,
  array(
    SELECT unnest 
    FROM unnest(to_morpheme_array('ドリル')) 
    WHERE unnest = ANY(morpheme_array)
  ) as common_elements,
  cardinality(
    array(
      SELECT unnest 
      FROM unnest(to_morpheme_array('ドリル')) 
      WHERE unnest = ANY(morpheme_array)
    )
  )::float / 
  cardinality(
    array(
      SELECT DISTINCT element 
      FROM (
        SELECT unnest(to_morpheme_array('ドリル')) as element
        UNION ALL
        SELECT unnest(morpheme_array)
      ) combined
    )
  ) as score
FROM product ORDER BY score DESC
;