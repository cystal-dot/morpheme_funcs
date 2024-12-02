EXPLAIN ANALYZE
SELECT * FROM product ORDER BY calculate_morpheme_funcs(name, 'ドリル') DESC LIMIT 10;