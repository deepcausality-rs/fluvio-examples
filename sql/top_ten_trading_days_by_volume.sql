SELECT
  timestamp datetime,
  sum(volume) volume,

FROM kraken_xbtusd

SAMPLE BY 1d
ALIGN TO CALENDAR WITH OFFSET '00:00'
ORDER BY volume DESC
limit 10;
