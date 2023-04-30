fn main() {
    let input = include_str!("input.txt");
    let mut carries = Vec::new();

    {
        let mut current_carry: usize = 0;
        for (_, line) in input.lines().enumerate() {
            match line.parse::<usize>() {
                Ok(num) => current_carry += num,
                Err(_) => {
                    carries.push(current_carry);
                    current_carry = 0;
                }
            }
        }

        if current_carry != 0 {
            carries.push(current_carry);
        }
    }

    let largest_carry = carries
        .iter()
        .max()
        .expect("No largest carry found in the input");

    println!("part 1: {}", largest_carry);

    carries.sort();
    let top_three_sum = carries.iter().rev().take(3).sum::<usize>();

    println!("part 2: {}", top_three_sum);
}
