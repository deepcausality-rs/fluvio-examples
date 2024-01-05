SELECT
  count,
  min(number_of_rows),
  max(number_of_rows),
  avg(number_of_rows),
  sum(number_of_rows),

FROM kraken_symbols
;