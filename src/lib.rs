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

    /// First row contains headers
    #[arg(default_value = "true", long)]
    has_headers: bool,

    /// Number of rows to display
    #[arg(short, long)]
    nrows: Option<usize>,

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

fn read_csv(path: &str, has_headers: bool) -> CsvData {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(has_headers)
        .from_path(path).unwrap();

    let headers = reader.headers().unwrap().clone();
    let mut rows = Vec::new();
    for result in reader.records() {
        rows.push(result.unwrap());
    };
    CsvData::new(headers, rows)
}

fn format_table(data: &mut CsvData, nrows:Option<usize>, cols: Option<Vec<String>>) {
    // keep selected nr of rows
    match nrows {
        Some(n) => {
            data.rows = data.rows[0..n].to_vec();
        },
        None => ()
    }
    

    // keep selected columns
    match cols {
        Some(inner) => println!("{:?}", inner),
        None => println!("No columns selected!")
    }
}

fn display_table(data: &CsvData) {
    let mut builder = Builder::default();

    let header_data: Vec<&str> = data.headers.iter().collect();
    builder.add_record(header_data);

    for row in data.rows.iter() {
        let row_data: Vec<&str> = row.iter().collect();
        builder.add_record(row_data);
    }

    let mut table = builder.build();
    table.with(Style::ascii_rounded());
    println!("{}", table);
}

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let mut data = read_csv(&args.filepath, args.has_headers);
    format_table(&mut data, args.nrows, args.cols);
    display_table(&data);

    Ok(())
}
