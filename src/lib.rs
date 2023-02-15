use csv::StringRecord;
use std::error::Error;

use clap::Parser;
use tabled::{builder::Builder, Style};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[clap(author="Michael Vauthey", version, about)]
/// A dataframe inspection tool for the CLI written in Rust
pub struct Args {
    /// Path of the file to inspect
    pub filepath: String,

    /// Number of rows to display
    #[arg(default_value = "10", short, long)]
    nrows: usize,

    /// Subset of columns to display
    /// 
    cols: Option<Vec<String>>,

}

pub struct CsvData {
    pub headers: StringRecord,
    pub rows: Vec<StringRecord>,
}

impl CsvData {
    fn new(headers: StringRecord, rows: Vec<StringRecord>) -> CsvData {
        CsvData { headers: headers, rows: rows }
    }   
}

fn read_csv(path: &str) -> Result<CsvData, Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;
    let headers = reader.headers()?.clone();
    let mut rows = Vec::new();
    for result in reader.records() {
        rows.push(result?);
    };
    Ok(CsvData::new(headers, rows))
}

fn display_table(data: &CsvData) {
    let mut builder = Builder::default();

    let header: Vec<&str> = data.headers.iter().collect();
    builder.add_record(header);
    for row in data.rows.iter() {
        let row_data: Vec<&str> = row.iter().collect();
        builder.add_record(row_data);
    }

    let mut table = builder.build();
    table.with(Style::ascii_rounded());
    println!("{}", table);
}

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let data = read_csv(&args.filepath).unwrap();
    display_table(&data);

    Ok(())
}
