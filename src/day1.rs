use std::str::FromStr;

#[cfg(test)]
mod tests {
    use super::{count_increases, produce_measurements, produce_windows};

    #[test]
    fn test_day1b() {
        let measures = produce_measurements("input/day1b.txt").unwrap();
        let windows = produce_windows(measures);
        let increases = count_increases(windows);
        println!("Increases: {}", increases);
    }
}

pub struct WindowMeasure {
    first: Option<u32>,
    second: Option<u32>,
    third: Option<u32>,
}

impl WindowMeasure {
    pub fn init(first: u32) -> Self {
        WindowMeasure {
            first: Some(first),
            second: None,
            third: None,
        }
    }

    pub fn add_second(&mut self, second: u32) {
        self.second = Some(second);
    }

    pub fn add_third(&mut self, third: u32) {
        self.third = Some(third);
    }

    pub fn sum(&self) -> u32 {
        let mut sum = 0;
        if let Some(n) = self.first {
            sum += n;
        }
        if let Some(n) = self.second {
            sum += n;
        }
        if let Some(n) = self.third {
            sum += n;
        }
        return sum;
    }
}

pub fn count_increases(windows: Vec<WindowMeasure>) -> u32 {
    let mut count = 0;
    for i in 1..windows.len() {
        let prev_sum = windows[i - 1].sum();
        let current_sum = windows[i].sum();
        if current_sum > prev_sum {
            count += 1;
        }
    }
    count
}

pub fn produce_windows(measures: Vec<u32>) -> Vec<WindowMeasure> {
    let mut windows: Vec<WindowMeasure> = Vec::with_capacity(measures.len());
    for i in 0..measures.len() {
        let m = measures[i];
        if i > 1 {
            if let Some(prev_prev_window) = windows.get_mut(i - 2) {
                prev_prev_window.add_third(m);
            }
        }
        if i > 0 {
            if let Some(prev_window) = windows.get_mut(i - 1) {
                prev_window.add_second(m);
            }
        }
        windows.insert(i, WindowMeasure::init(m));
    }
    windows
}

pub fn produce_measurements(input_path: &str) -> Result<Vec<u32>, std::io::Error> {
    let input = std::fs::read_to_string(input_path)?;
    Ok(input
        .split('\n')
        .map(|s| match u32::from_str(s) {
            Ok(n) => n,
            Err(_) => 0,
        })
        .filter(|n| *n > 0)
        .collect())
}
