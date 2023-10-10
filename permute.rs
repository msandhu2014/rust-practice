pub fn permute_binary(n: u16)  ->  () {
    let slate = String::new();
    helper(slate, n);
}

fn helper( mut buffer: String, index: u16) -> () {
    if index == 0 {
        println!("{}", buffer);
        return;
    }

    buffer.push('0');
    helper(buffer.clone(), index - 1);
    buffer.pop();
    buffer.push('1');
    helper(buffer, index - 1);

}

pub fn permute_string(input: String) -> () {
    let mut result = Vec::<String>::new();
    helper_string( &input, String::new(), &mut result, 0);
    println!("num of subsets: {}", result.len());
    result.iter().for_each(|f| println!("{}", f));
}

fn helper_string(input: &String, mut buffer: String, result: &mut Vec<String>, index: usize) {
    if index == input.len() {
        result.push(buffer);
        return
    }

    helper_string( input, buffer.clone(), result, index+1);
    buffer.push(input.as_bytes()[index] as char);
    helper_string( input, buffer, result, index+1);
    return
}


pub fn find_palindromes(input: String) -> () {
    let mut result = Vec::<String>::new();
    helper_palindrome(&input, &mut String::new(), &mut result, 0, 
        &mut String::new());
}
fn helper_palindrome(input: &String,  buffer: &mut String, 
    result: &mut Vec<String>, index: usize, last_string: &mut String) -> () {

    println!("last_str: {}, buffer: {}", last_string, buffer);
    if index >= input.len() {
        if is_plaindrome(last_string) {
            buffer.push_str(&last_string);
            result.push(buffer.to_string());
            buffer.clear();
        }
        return;
    }

    last_string.push(input.chars().nth(index).unwrap());
    helper_palindrome(input, buffer, result, index+1, last_string);
    last_string.pop();
    if is_plaindrome(last_string) {
        buffer.push_str(&last_string);
        buffer.push('|');
        last_string.clear();
        helper_palindrome(input, buffer, result, index+1, last_string)
    } else {
        return;
    }
}

fn is_plaindrome(input: &String) -> bool {
    let mut left = 0;
    let mut right  = input.len() - 1;
    while left < right {
        if input.chars().nth(left) != input.chars().nth(right) {
            return false;
        }
        left += 1;
        right -= 1;
    }
    return true;
}