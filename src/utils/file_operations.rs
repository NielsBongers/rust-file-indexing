use polars::prelude::*;

#[allow(unused)]
use log::{error, info, warn};

use std::{
    fs::create_dir_all,
    io::{Error, ErrorKind},
    path::Path,
};

/// Saving the parquet cache.
pub fn save_path_index_cache(file_path: &Path, df: &DataFrame) {
    let cache_file_path = file_path.join("rust-file-index.parquet");

    info!("Saving cache: {:?}", cache_file_path);

    let mut file =
        std::fs::File::create(cache_file_path).expect("Failed to create parquet index file");

    ParquetWriter::new(&mut file)
        .finish(&mut df.clone())
        .unwrap();
}

/// Loading the cache: currently not used.
pub fn _load_path_index_cache(file_path: &Path) -> DataFrame {
    let mut file = std::fs::File::open(file_path).expect("Failed to open file");
    ParquetReader::new(&mut file).finish().unwrap()
}

/// Checks whether a path exists and whether it is a folder.
pub fn check_valid_folder_path(path: &str) -> Result<&Path, Error> {
    let path = Path::new(path);

    if !path.exists() {
        return Err(Error::new(
            ErrorKind::NotFound,
            format!("The specified path does not exist: {:?}", path),
        ));
    }

    if !path.is_dir() {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!("The specified path is not a folder: {:?}", path),
        ));
    }

    Ok(path)
}

/// Printing and saving the analysis DataFrames.
pub fn print_and_save(
    df: &mut DataFrame,
    analysis_folder_path: &Path,
    file_name: &str,
    table_name: &str,
) {
    info!("{}: {:?}", table_name, df);

    let analysis_file_path = analysis_folder_path.join(Path::new(file_name));

    create_dir_all(analysis_folder_path).expect("Failed to create analysis result folder.");

    CsvWriter::new(&mut std::fs::File::create(analysis_file_path).expect("Failed to create file"))
        .include_header(true)
        .with_separator(b',')
        .finish(df)
        .expect("Failed to write df.");
}
