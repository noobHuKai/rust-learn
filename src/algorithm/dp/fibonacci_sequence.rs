//! # 斐波那数列


/// 递归形式的斐波那数列
fn fib_recursive(n: i32) -> i32 {
    if n == 1 || n == 2 {
        return 1;
    }
    return fib_recursive(n - 1) + fib_recursive(n - 2);
}

/// 动态规划的斐波那数列
fn fib_dp(n: i32) -> i32 {
    if n == 1 || n == 2 {
        return 1;
    }
    let mut res = vec![1, 1];
    for i in 2..n {
        res.push(res[(i - 1) as usize] + res[(i - 2) as usize])
    }
    return res.pop().unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_recursive() {
        let timer = std::time::Instant::now();
        println!("fib({}) = {}", 30, super::fib_recursive(30));  //832040
        println!("elapsed time: {} us", timer.elapsed().as_micros());  //5163 us
    }

    #[test]
    fn test_dp() {
        let timer = std::time::Instant::now();
        println!("fib({}) = {}", 30, super::fib_dp(30));  //832040
        println!("elapsed time: {} us", timer.elapsed().as_micros());// 9 us
    }
}