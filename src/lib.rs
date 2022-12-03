pub mod aoc {
    pub fn day_1(input: &str) -> Vec<i32> {
        let mut elfs = input
            .split("\n\n")
            .map(|elf: &str| {
                elf.lines()
                    .map(|calories| calories.trim().parse::<_>().unwrap())
                    .collect::<Vec<_>>()
                    .iter()
                    .sum::<i32>()
            })
            .collect::<Vec<_>>();
        elfs.sort();
        println!(
            "Part 1\n\tMost calories carried by an elf: {}\n\tSum of the top 3 elfs calories: {}",
            elfs.iter().max().unwrap(),
            elfs.iter().rev().take(3).sum::<i32>()
        );
        vec![
            elfs.clone().into_iter().max().unwrap(),
            elfs.iter().rev().take(3).sum::<i32>(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc::day_1;
    #[test]
    fn test_day_1() {
        assert_eq!(
            day_1("1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000"),
            vec![24000, 45000]
        );
    }
}
