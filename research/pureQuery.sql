DROP INDEX IF EXISTS idx_product_name;
CREATE INDEX idx_product_name ON product USING btree (name); 
EXPLAIN ANALYZE
SELECT 
  name,
  to_morpheme_array(name) as morpheme_array,
  to_morpheme_array('ドリル') as keyword_morpheme_array,
  array(
    SELECT unnest 
    FROM unnest(to_morpheme_array(name)) 
    WHERE unnest = ANY(to_morpheme_array('ドリル'))
  ) as common_elements,
  cardinality(
    array(
      SELECT unnest 
      FROM unnest(to_morpheme_array(name)) 
      WHERE unnest = ANY(to_morpheme_array('ドリル'))
    )
  )::float / 
  cardinality(
    array(
      SELECT DISTINCT element 
      FROM (
        SELECT unnest(to_morpheme_array(name)) as element
        UNION ALL
        SELECT unnest(to_morpheme_array('ドリル'))
      ) combined
    )
  ) as score
FROM product
ORDER BY score DESC
;