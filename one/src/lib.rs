use metrics::*;
pub fn add(a: i32, b: i32) -> i64 {
    let start = std::time::Instant::now();

    increment_counter!("add");
    increment_gauge!("total", 1f64);
    let c = a as i64 + b as i64;
    histogram!("add", start.elapsed());
    c
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
