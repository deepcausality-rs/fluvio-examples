SELECT
    timestamp,
    sum(price * volume) / sum(volume) AS vwap,
    sum(volume) as volume,

FROM kraken_xbteur
WHERE timestamp IN '2023'
SAMPLE BY 5m
ALIGN TO CALENDAR WITH OFFSET '00:00'
;