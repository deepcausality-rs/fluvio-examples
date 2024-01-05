SELECT
  timestamp datetime,
  first(price) open,
  max(price) high,
  min(price) low,
  last(price) close,
  sum(volume) volume,

FROM kraken_xbtusd

-- https://questdb.io/docs/reference/operators/date-time/
WHERE timestamp IN '2022'
SAMPLE BY 5m
ALIGN TO CALENDAR WITH OFFSET '00:00'
-- ORDER BY timestamp DESC
;