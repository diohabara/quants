use ndarray::Array1;
use num::traits::ToPrimitive;

/// Compute the mean of a generic series.
pub fn mean<T>(series: &Array1<T>) -> Option<f64>
where
    T: ToPrimitive + Clone,
{
    let sum: f64 = series.iter().cloned().map(|x| x.to_f64().unwrap()).sum();
    let len = series.len() as f64;
    if len == 0.0 {
        None
    } else {
        Some(sum / len)
    }
}

/// Compute the standard deviation of a generic series.
pub fn standard_deviation<T>(array: &Array1<T>) -> Option<f64>
where
    T: num::traits::ToPrimitive + Clone,
{
    let mean_value = mean(array)?;
    let variance: f64 = array
        .iter()
        .map(|x| {
            let x_f64 = x.to_f64().unwrap();
            (x_f64 - mean_value).powi(2)
        })
        .sum();
    let len = array.len() as f64;
    if len == 0.0 || len == 1.0 {
        None
    } else {
        Some((variance / (len - 1.0)).sqrt())
    }
}

#[cfg(test)]
mod tests {
    use crate::timeseries::from_vec;

    use super::*;

    #[test]
    fn test_mean_f64() {
        let series = from_vec(vec![1.0, 2.0, 3.0]);
        let mean = mean(&series);
        assert_eq!(mean, Some(2.0));
    }

    #[test]
    fn test_standard_deviation_f64() {
        let series = from_vec(vec![1.0, 2.0, 3.0]);
        let std_dev = standard_deviation(&series);
        assert_eq!(std_dev, Some(1.0));
    }

    #[test]
    fn test_mean_i32() {
        let series = from_vec(vec![1, 2, 3]);
        let mean = mean(&series);
        assert_eq!(mean, Some(2.0));
    }

    #[test]
    fn test_standard_deviation_i32() {
        let series = from_vec(vec![1, 2, 3]);
        let std_dev = standard_deviation(&series);
        assert_eq!(std_dev, Some(1.0));
    }
}
