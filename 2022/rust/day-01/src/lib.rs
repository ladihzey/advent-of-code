pub struct CalorieCounter {
    calories: Vec<u32>,
}

impl CalorieCounter {
    pub fn new(input: &str) -> CalorieCounter {
        let mut calories = input
            .split("\n\n")
            .map(|elf_load| {
                elf_load
                    .lines()
                    .map(|s| s.parse::<u32>().unwrap())
                    .sum::<u32>()
            })
            .collect::<Vec<_>>();

        calories.sort_by(|a, b| b.cmp(a));

        CalorieCounter { calories }
    }

    pub fn find_max_calories_elf_load(&self) -> u32 {
        *self.calories.iter().max().unwrap()
    }

    pub fn sum_top_elf_load_calories(&self, n: usize) -> u32 {
        self.calories.iter().take(n).sum()
    }
}
