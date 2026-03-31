pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    // so looks like everytime when seeing an operator
    // it does the calculation and put back the result
    // by popping out last 2 numbers
    let mut vec = Vec::new();

    for token in tokens {
        if token == "+" {
            let first = vec.pop().unwrap();
            let second = vec.pop().unwrap();
            vec.push(first + second);
        }
        if token == "-" {
            let first = vec.pop().unwrap();
            let second = vec.pop().unwrap();
            vec.push(second - first);
        }
        if token == "*" {
            let first = vec.pop().unwrap();
            let second = vec.pop().unwrap();
            vec.push(first * second);
        }
        if token == "/" {
            let first = vec.pop().unwrap();
            let second = vec.pop().unwrap();
            vec.push(second / first);
        }
        if let Ok(n) = token.parse::<i32>() {
            vec.push(n);
        }
    }

    vec.pop().expect("not a valid rpn")
}
