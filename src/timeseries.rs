pub mod statistics;
pub mod window;
use ndarray::{s, Array1};
use rand::Rng;
use window::Window;

pub fn generate_series(size: usize) -> Array1<f64> {
    let mut rng = rand::thread_rng();
    let series: Vec<f64> = (0..size).map(|_| rng.gen_range(0.0..1.0)).collect();
    Array1::from(series)
}

pub fn from_vec<T>(series: Vec<T>) -> Array1<T>
where
    T: Clone + std::fmt::Debug,
{
    Array1::from_vec(series)
}

pub fn volatility(x: &Array1<f64>, window: &Window) -> Array1<f64> {
    let window_size = window.size();
    let len = x.len();
    let mut vol = Vec::with_capacity(len - window_size + 1);

    for i in 0..=(len - window_size) {
        let window_slice = x.slice(s![i..i + window_size]);
        let mean = window_slice.mean().unwrap();
        let variance = window_slice
            .iter()
            .map(|&v| (v - mean).powi(2))
            .sum::<f64>()
            / window_size as f64;
        vol.push(variance.sqrt());
    }

    Array1::from(vol)
}

#[cfg(test)]
mod tests {
    use timeseries::window::Window;

    use super::*;
    use crate::timeseries;

    #[test]
    fn test_from_vec_f64() {
        let vec = vec![1.0, 2.0, 3.0];
        let array = from_vec(vec);
        assert_eq!(array.len(), 3);
        assert_eq!(array[0], 1.0);
        assert_eq!(array[1], 2.0);
        assert_eq!(array[2], 3.0);
    }

    #[test]
    fn test_from_vec_i32() {
        let vec = vec![1, 2, 3];
        let array = from_vec(vec);
        assert_eq!(array.len(), 3);
        assert_eq!(array[0], 1);
        assert_eq!(array[1], 2);
        assert_eq!(array[2], 3);
    }

    #[test]
    fn test_gen_timeseries() {
        let series = timeseries::generate_series(10);
        assert_eq!(series.len(), 10);
        println!("Generated time series: {:?}", series);
    }

    #[test]
    fn test_calculate_volatility() {
        let x = timeseries::generate_series(10);
        let window = Window::new(3, 0);
        let vol = volatility(&x, &window);

        assert_eq!(vol.len(), 8); // 10 - 3 + 1 = 8
        println!("Calculated volatility: {:?}", vol);
    }
}
