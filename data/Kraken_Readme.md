# Kraken Data Download

Historical market data (also known as trading history or tick data)
provides a detailed record of every trade that happens on our markets, including the following information:

## Data schema 

* The exact date and time of each trade,
* The price at which each trade occurred,
* The amount of volume that was traded.

## Timestamp resolution

Kraken returns [millisecond-precision unix timestamps](https://github.com/ccxt/ccxt/issues/6039)


## Download link

https://support.kraken.com/hc/en-us/articles/360047543791-Downloadable-historical-market-data-time-and-sales-

**Important**: When downloading the quarterly tick data, you must rename the unzipped folder to 

Kraken_Trading_History

before copying it in the data folder otherwise the data import will fail due to unrecognized file path. 


FIX:

ETH2SETH.csv