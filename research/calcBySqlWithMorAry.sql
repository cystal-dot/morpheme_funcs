EXPLAIN ANALYZE
WITH keyword_morphemes AS (
    SELECT array_agg(DISTINCT m) AS morphemes
    FROM unnest(to_morpheme_array('ドリル')) AS m
)
SELECT
    p.*,
    CASE
        WHEN total_count = 0 THEN 0.0
        ELSE common_count::float / total_count::float
    END AS score
FROM
    product p
CROSS JOIN LATERAL (
    SELECT array_agg(DISTINCT m) AS morphemes
    FROM unnest(to_morpheme_array(p.name)) AS m
) AS product_morphemes
CROSS JOIN keyword_morphemes
CROSS JOIN LATERAL (
    SELECT
        COUNT(DISTINCT m) AS common_count
    FROM unnest(keyword_morphemes.morphemes) AS m
    WHERE m = ANY(product_morphemes.morphemes)
) AS common
CROSS JOIN LATERAL (
    SELECT
        COUNT(DISTINCT m) AS total_count
    FROM (
        SELECT unnest(keyword_morphemes.morphemes)
        UNION
        SELECT unnest(product_morphemes.morphemes)
    ) AS m
) AS total
ORDER BY score DESC
LIMIT 10;
