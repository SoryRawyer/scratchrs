
fn main() {
    let s = &[1,2,3,1,1,1];
    println!("{}", counting(s, 1));

    println!("{}", postfix_eval(&String::from("1 2 +")));
    println!("{}", postfix_eval(&String::from("3 7 *")));
    println!("{}", postfix_eval(&String::from("8 2 /")));
    println!("{}", postfix_eval(&String::from("10 11 8 * * 2 /")));
}

fn counting(haystack: &[i32], needle: i32) -> isize {
    // return the number of times needle appears in haystack
    let mut count: isize = 0;
    for num in haystack.iter() {
        if needle == *num {
            count += 1;
        }
    }
    count
}

fn postfix_eval(expr: &str) -> i32 {
    // go through string, when you see a symbol, evaluate it using the previous two operators
    let mut arguments: Vec<i32> = Vec::new();
    let mut cur: String = String::new();
    for c in expr.chars() {
        match c {
            ' ' => {
                if cur == "" {
                    continue
                }
                arguments.push(cur.parse::<i32>().unwrap());
                cur = String::new();
            }
            '+' => {
                // add
                let right = arguments.pop().unwrap();
                let left = arguments.pop().unwrap();
                arguments.push(left + right);
            }
            '-' => {
                // sub
                let right = arguments.pop().unwrap();
                let left = arguments.pop().unwrap();
                arguments.push(left - right);
            }
            '*' => {
                // mult
                let right = arguments.pop().unwrap();
                let left = arguments.pop().unwrap();
                arguments.push(left * right);
            }
            '/' => {
                // divide
                let right = arguments.pop().unwrap();
                let left = arguments.pop().unwrap();
                arguments.push(left / right);
            }
            '%' => {
                // divide
                let right = arguments.pop().unwrap();
                let left = arguments.pop().unwrap();
                arguments.push(left % right);
            }
            _ => cur.push(c),
        }
    }
    arguments.pop().unwrap()
}
