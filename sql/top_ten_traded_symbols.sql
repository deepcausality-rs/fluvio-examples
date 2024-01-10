
-- Most traded symbols
SELECT *
FROM kraken_symbols
ORDER BY number_of_rows DESC
LIMIT 10
;

-- Least traded symbols
SELECT *
FROM kraken_symbols
ORDER BY number_of_rows ASC
LIMIT 10
;

-- ethaed 278
