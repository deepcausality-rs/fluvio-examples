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

You can download either the full dataset (recommended, but but big), or the 
quarterly dataset, which is significantly smaller. Either one works. 

https://support.kraken.com/hc/en-us/articles/360047543791-Downloadable-historical-market-data-time-and-sales-

## (!!!) Important (!!!)

Make sure that the unzipped folder is in the project [data folder](data) ***and*** the absolute path to the data folder is set in the [import_data.toml](import_config.toml)  config file. This is crucial to make the data import work. 
