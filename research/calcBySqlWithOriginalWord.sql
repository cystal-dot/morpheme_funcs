EXPLAIN ANALYZE
SELECT 
    id,
    name,
    array_length(morpheme_array, 1) AS total_morphemes,
    COUNT(input_element) AS hits,
    COUNT(input_element)::real / array_length(morpheme_array, 1) AS hit_rate
FROM 
    product,
    unnest(to_morpheme_array('ドリル')) AS input_element
WHERE 
    input_element = ANY(morpheme_array)
GROUP BY 
    id, name, morpheme_array
ORDER BY 
    hit_rate DESC;
