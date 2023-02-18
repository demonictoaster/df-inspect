use std::error::Error;

use clap::Parser;
use csv::StringRecord;
use tabled::{builder::Builder, Style};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[clap(author = "Michael Vauthey", version, about)]
/// A dataframe inspection tool for the CLI written in Rust
pub struct Args {
    /// Path of the file to inspect
    filepath: String,

    /// First row contains headers (bool, optional)
    #[arg(default_value = "false", long)]
    no_headers: bool,

    /// Number of rows to display (int, optional)
    #[arg(short, long)]
    nrows: Option<usize>,

    /// Subset of columns to display (string(s), optional)
    #[arg(short, long, num_args(0..))]
    cols: Option<Vec<String>>,
}

pub struct CsvData {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

impl CsvData {
    fn new(headers: Vec<String>, rows: Vec<Vec<String>>) -> CsvData {
        CsvData {
            headers: headers,
            rows: rows,
        }
    }
}

fn read_csv(path: &str, no_headers: bool) -> CsvData {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(!no_headers)
        .from_path(path)
        .unwrap();

    let headers_data = if no_headers {
        let ncols = reader.headers().unwrap().len();
        let mut colnames = Vec::new();
        for i in 0..ncols {
            colnames.push(format!("col{}", i).to_string());
        }
        colnames
    } else {
        get_data_from_record(reader.headers().unwrap().clone())
    };

    let mut rows_data = Vec::new();
    for record in reader.records() {
        let row_data = get_data_from_record(record.unwrap());
        rows_data.push(row_data);
    }
    CsvData::new(headers_data, rows_data)
}

fn get_data_from_record(record: StringRecord) -> Vec<String> {
    record.iter().map(|s| s.to_string()).collect()
}

fn format_table(data: &mut CsvData, nrows: Option<usize>, cols: Option<Vec<String>>) {
    // keep selected nr of rows
    match nrows {
        Some(n) => {
            if n < data.rows.len() {
                data.rows = data.rows[0..n].to_vec();
            }
        }
        None => (),
    }

    // keep selected columns
    match cols {
        Some(cols) => {
            // find indexes of selected columns
            let mut col_idx: Vec<usize> = Vec::new();
            for col in cols.iter() {
                for (idx, header) in data.headers.iter().enumerate() {
                    if col == header {
                        col_idx.push(idx);
                    }
                }
            }

            // filter entries to keep selected columns
            filter_by_index(&mut data.headers, &col_idx);
            for row in data.rows.iter_mut() {
                filter_by_index(row, &col_idx);
            }
        }
        None => (),
    }
}

fn filter_by_index<T>(data: &mut Vec<T>, indexes: &Vec<usize>) {
    let mut idx: usize = 0;
    data.retain(|_| {
        let v = idx;
        idx += 1;
        indexes.contains(&v)
    });
}

fn display_table(data: CsvData) {
    let mut builder = Builder::default();

    builder.add_record(data.headers);

    for row in data.rows.iter() {
        //let row_data: Vec<&str> = row.iter().collect();
        builder.add_record(row);
    }

    let mut table = builder.build();
    table.with(Style::rounded());
    println!("{}", table);
}

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let mut data = read_csv(&args.filepath, args.no_headers);
    format_table(&mut data, args.nrows, args.cols);
    display_table(data);

    Ok(())
}
