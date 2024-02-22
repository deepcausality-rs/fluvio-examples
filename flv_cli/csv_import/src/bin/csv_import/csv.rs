use std::error::Error;
use std::fs;
use std::path::Path;
use csv::ReaderBuilder;
use encoding_rs::UTF_8;
use crate::trade_data::TradeData;

pub(crate) fn read_csv<P>(path: P) -> Result<Vec<TradeData>, Box<dyn Error>>
    where
        P: AsRef<Path>,
{
    let file = fs::read(path).expect("[csv_utils/read_csv_lines]: Could not read file");
    let (res, _, _) = UTF_8.decode(&file);

    let mut content: Vec<TradeData> = Vec::with_capacity(500_000); // fixed pre-allocation

    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(res.as_bytes());


    // https://docs.rs/csv/latest/csv/struct.Reader.html
    for result in rdr.records() {

        match result {
            Ok(record) => {
                let timestamp = record[0]
                    .parse::<u64>()
                    .expect("[csv_utils/read_csv_lines]: Could not parse timestamp");
                let p = record[1]
                    .parse::<f64>()
                    .expect("[csv_utils/read_csv_lines]: Could not parse price");

                let v = record[2]
                    .parse::<f64>()
                    .expect("[csv_utils/read_csv_lines]: Could not parse volume");


                let trade_data = TradeData::new(timestamp, p, v);
                content.push(trade_data);
            }

            Err(err) => {
                println!(
                    "[csv/read_csv_lines]: error reading CSV line: {}",
                    err
                );
            }
        }
    }

    Ok(content)
}