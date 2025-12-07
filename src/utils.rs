use anyhow::{Result, anyhow};
use ndarray::Array2;

pub fn ragged_to_arr<T>(table: Vec<Vec<T>>) -> Result<Array2<T>> {
    let rows = table.len();
    let cols = table.first().ok_or(anyhow!("no rows"))?.len();

    let numbers: Vec<T> = table.into_iter().flatten().collect();
    let numbers = Array2::from_shape_vec((rows, cols), numbers)?;

    Ok(numbers)
}
