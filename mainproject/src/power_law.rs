// orginally had many errors, searched how to implement it with data set 
pub fn fit_power_law_distribution(data: &[usize]) -> bool {
    let n = data.len() as f64;
    let sum_log_x = data.iter().map(|&x| f64::ln(x as f64)).sum::<f64>();
    let sum_log_x_log_n = data.iter().map(|&x| f64::ln(x as f64) * f64::ln(n)).sum::<f64>();
    let sum_log_n = f64::ln(n).powi(2);
    let sum_x_log_x = data.iter().map(|&x| x as f64 * f64::ln(x as f64)).sum::<f64>();
    let sum_x = data.iter().sum::<usize>() as f64;

    let alpha = (n * sum_x_log_x - sum_x * sum_log_x) / (n * sum_log_x_log_n - sum_log_n);
    let _xmin = (alpha * n / sum_x).exp(); 

    true 
}
