use ndarray::Array1;
use num::traits::ToPrimitive;

pub fn mean<T>(series: &Array1<T>, is_sample: bool) -> Option<f64>
where
    T: ToPrimitive + Clone,
{
    let sum: f64 = series.iter().cloned().map(|x| x.to_f64().unwrap()).sum();
    let len = series.len();
    if len == 0 {
        None
    } else {
        if is_sample {
            Some(sum / (len - 1) as f64)
        } else {
            Some(sum / len as f64)
        }
    }
}

pub fn variance<T>(series: &Array1<T>, is_sample: bool) -> Option<f64>
where
    T: ToPrimitive + Clone,
{
    let mean_value = mean(series, is_sample)?;
    let variance: f64 = series
        .iter()
        .map(|x| {
            let x_f64 = x.to_f64().unwrap();
            (x_f64 - mean_value).powi(2)
        })
        .sum();
    let len = series.len();
    if is_sample {
        if len == 0 || len == 1 {
            None
        } else {
            Some(variance / (len - 1) as f64)
        }
    } else {
        if len == 0 {
            None
        } else {
            Some(variance / len as f64)
        }
    }
}

pub fn standard_deviation<T>(series: &Array1<T>, is_sample: bool) -> Option<f64>
where
    T: num::traits::ToPrimitive + Clone,
{
    match variance(series, is_sample) {
        Some(variance) => Some(variance.sqrt()),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::timeseries::from_vec;

    use super::*;

    #[test]
    fn test_population_mean() {
        let series = from_vec(vec![
            128, 219, 316, 189, 512, 98, 155, 110, 468, 177, 203, 73, 252,
        ]);
        let mean = mean(&series, false);
        if let Some(mean) = mean {
            assert_eq!(mean - 223.0769 < 0.0001, true);
        } else {
            panic!("Mean calculation failed.");
        }
    }

    #[test]
    fn test_population_variance() {
        let series = from_vec(vec![
            128, 219, 316, 189, 512, 98, 155, 110, 468, 177, 203, 73, 252,
        ]);
        let var = variance(&series, false);
        if let Some(var) = var {
            assert_eq!(var - 17020.5325 < 0.0001, true);
        } else {
            panic!("Variance calculation failed.");
        }
    }

    #[test]
    fn test_population_std() {
        let series = from_vec(vec![
            128, 219, 316, 189, 512, 98, 155, 110, 468, 177, 203, 73, 252,
        ]);
        let std = standard_deviation(&series, false);
        if let Some(std) = std {
            assert_eq!(std - 130.4627 < 0.0001, true);
        } else {
            panic!("Standard deviation calculation failed.");
        }
    }
}
