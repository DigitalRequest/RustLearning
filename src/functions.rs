struct Ship {
    draft: u32,
    crew: u32,
}

impl Ship {

    fn is_worth_it(&self) -> bool {
        if (self.draft as f32) - (1.5  * self.crew as f32) > 20 as f32 {
            return true;
        }

        return false;
    }
}

pub struct Dog<'a> {
    pub(crate) name: &'a str,
    pub(crate) weight: i8,
    pub(crate) price: i32,
}

impl Dog<'_> {
    pub(crate) fn bark(&self) {
        println!("{} barks!", self.name);
    }
}

pub fn remove_smallest(numbers: &[u32]) -> Vec<u32> {
    // Check if the input vector is empty
    if numbers.is_empty() {
        return Vec::new();
    }

    // Find the index of the smallest element
    let min_index = numbers.iter().position(|&x| x == *numbers.iter().min().unwrap()).unwrap();

    // Create a new vector without the smallest element
    let result: Vec<u32> = numbers.iter().cloned().enumerate().filter_map(|(i, x)| if i != min_index { Some(x) } else { None }).collect();

    result
}

pub fn sum_of_differences(arr: &[i8]) -> Option<i8> {

    if arr.len() == 0 || arr.len() == 1 {
        return None;
    }

    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort_by(|a, b| b.cmp(a));

    let mut sum: Option<i8> = Some(0);

    for x in 0..sorted_arr.len() - 1 {
        let val = (sorted_arr[x] - sorted_arr[x + 1]).abs();
        sum = sum.map(|s| s + val);
    }

    sum

}

pub fn last_digit(str1: &str, str2: &str) -> i32 {
    let number1: Result<i32, _> = str1.parse();
    let number2: Result<i32, _> = str2.parse();

    match (number1, number2) {
        (Ok(num1), Ok(num2)) => {
            let result = (num1 as u32).pow(num2 as u32);
            let last_digit = (result % 10) as i32;
            println!("Result: {}", result);
            println!("Last Digit: {}", last_digit);
            last_digit
        }
        _ => {
            println!("Failed to convert string to number");
            -1 // or any other suitable default value or error indicator
        }
    }
}

pub fn group_str(str: &str) -> Vec<(char, char)> {
    let mut arr: Vec<char> = str.chars().collect();

    let mut new_vec: Vec<(char, char)> = Vec::new();

    if arr.len() % 2 != 0 {
        arr.push('_');
    }

    for x in (0..arr.len()).step_by(2) {
        if x < arr.len() - 1 {
            new_vec.push((arr[x], arr[x + 1]));
        }
    }

    new_vec
}

pub fn nth_floyd_line(num: i32) -> i32 {

    let mut line: i32 = 1;
    let mut num_at: i32 = 1;

    for row in 1..1_000_000_000 {
        for _ in 0..row {
            if num_at == num {
                return line;
            }
            num_at += 1;
        }
        line += 1;
    }

    line
}