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
    let mut input_rev: Vec<char> = Vec::new();

    input_rev = make_rev_polish(input);

    for i in input_rev.clone(){
        if is_symbol(&i){
            let b: i32 = nums.pop().unwrap();
            let a: i32 = nums.pop().unwrap();
            if i == '+'{
                nums.push(add(&a,&b));
            }if i == '-'{
                nums.push(sub(&a,&b));
            }if i == '*'{
                nums.push(mul(&a,&b));
            }if i == '/'{
                nums.push(div(&a,&b));
            }
        }else{
            nums.push((i as i32) - 48);
        }
    }
    println!("{:?}", nums);
    
}

fn make_rev_polish(input: Vec<char>) -> Vec<char>{
    let mut result: Vec<char> = Vec::new();
    let mut symbol: Vec<char> = Vec::new();
    let mut inparent: Vec<char> = Vec::new();
    let mut flag = false;
println!("-----");
    for i in input.clone(){
        if flag {
            if i == ')' {
                inparent = make_rev_polish(inparent);
                for j in inparent.clone(){
                    result.push(j);
                }
                flag = false;
            }else{
                inparent.push(i);
            }
        }
        
        else if is_symbol(&i){
            if !symbol.is_empty(){
                result.push(symbol.pop().unwrap());
            }
            symbol.push(i);
        }
        
        else{
            if i == '(' {
                flag = true;
            }else{
                result.push(i);
            }
        }
    }
    for j in symbol.clone(){
        result.push(j);
    }
    println!("{:?}",result);
    result
}

fn main(){
    let input = read_as_char();
    let mut symbol: Vec<char> = Vec::new();
    let mut flag = false;
    let mut ff: Vec<char> = Vec::new();
    let mut result: Vec<char> = Vec::new();

    calculate(input);
}