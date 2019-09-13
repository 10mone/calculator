use std::io;

fn read_as_char() -> Vec<char>{
    let mut c = String::new();
    io::stdin().read_line(&mut c).unwrap();
    c.trim().split_whitespace()
        .map(|e| e.parse().ok().unwrap()).collect()
}
fn is_symbol(symbol: &char) -> bool{
    let lib: Vec<char> = vec!['+', '-', '/', '*'];
    for i in &lib{
        if symbol == i{
            return true
        }
    }
    false
}
fn add(a: &i32, b: &i32) -> i32{
    a + b
}
fn sub(a: &i32, b: &i32) -> i32{
    a - b
}
fn div(a: &i32, b: &i32) -> i32{
    a / b
}
fn mul(a: &i32, b: &i32) -> i32{
    a * b
}
fn calculate(input: Vec<char>) {
    let mut nums: Vec<i32> = Vec::new();
    let input_len = input.len();
    let mut length = 0;
    let mut result = 0;

    for i in input.clone(){
        if is_symbol(&i){
            length += 1;
            let b: i32 = nums.pop().unwrap();
            let a: i32 = nums.pop().unwrap();
            if i == '+'{
                if length == input_len{
                    result = add(&a,&b);
                }else{
                    nums.push(add(&a,&b));
                }
            }if i == '-'{
                if length == input_len{
                    result = sub(&a,&b);
                }else{
                    nums.push(sub(&a,&b));
                }
            }if i == '*'{
                if length == input_len{
                    result = mul(&a,&b);
                }else{
                    nums.push(mul(&a,&b));
                }
            }if i == '/'{
                if length == input_len{
                    result = div(&a,&b);
                }else{
                    nums.push(div(&a,&b));
                }
            }
        }else{
            length += 1;
            nums.push((i as i32) - 48);
        }
    }
    println!("{}", result);
}
fn make_rev_polish(input: Vec<char>) -> Vec<char>{
    let mut nums: Vec<char> = Vec::new();
    let mut symbol: Vec<char> = Vec::new();
    let mut result: Vec<char> = Vec::new();

    for i in &input{
        if is_symbol(i){
            symbol.push(*i);
        }else{
            nums.push(*i);
        }
    }
    for i in &nums{
        result.push(*i);
    }
    for i in &symbol{
        result.push(*i);
    }
    result
}

fn main(){
    let input = read_as_char();
    let mut symbol: Vec<char> = Vec::new();
    let mut flag = false;
    let mut ff: Vec<char> = Vec::new();
    let mut result: Vec<char> = Vec::new();

    for i in &input{
        if *i == ')'{
            let temp = make_rev_polish(ff.clone());
            for j in &temp{
                result.push(*j);
            }
            flag = false;
        }else if flag {
            ff.push(*i);
        }else if *i == '('{
            flag = true;
        }else if is_symbol(i){
            symbol.push(*i);
        }else{
            result.push(*i);
        }
    }
    for i in &symbol{
        result.push(*i);
    }

    calculate(result);
}