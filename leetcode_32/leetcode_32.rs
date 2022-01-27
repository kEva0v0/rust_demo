
use std::io;
use std::collections::VecDeque;

fn longest_valid_parentheses(s: String) -> i32 {
    let mut charsArray = s.as_bytes();
    let length:usize = charsArray.len();
    if(length == 0 || length == 1) {
        return 0;
    }
    let mut dp:Vec<usize> = Vec::new();
    for index in 0..length+10 {
        dp.push(0 as usize);
    }
    for index in 0..length-1 {
        if (index > 0 && (charsArray[index] as char) == ')') {
            if ((charsArray[index - 1] as char) == '('){
                dp[index] = (if(index > 1) {dp[index - 2]} else {0}) + 2;
            } else {
                if((index as isize) - (dp[index - 1] as isize) - 1 >= 0 && (charsArray[index - dp[index - 1] - 1] as char) == '(') {
                    let mut current = 2;
                    let mut pre = if(index >= 2 && (index as isize) - (dp[index - 1] as isize) - 2 >= 0) {
                        dp[index - dp[index - 1] - 2]
                    } else {
                        0
                    };
                    dp[index] = dp[index - 1] + current + pre;
                }
            }
        }
    }
    let mut answer:usize = 0;
    for index in 0..length {
        answer = if(answer > dp[index]) {answer} else {dp[index]};
    }
    return answer as i32;
}

fn main() {
    let mut inputString = String::new();
    io::stdin().read_line(&mut inputString).expect("Failed to read line");
    println!("{}", longest_valid_parentheses(inputString.to_string()));
}