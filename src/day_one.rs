use crate::CALORIES;
use once_cell::sync::Lazy;

#[allow(dead_code)]
pub(crate) fn find_elf_with_highest_calories(calories: &Lazy<Vec<&[i32]>>) -> (usize, i32) {
    let mut highest_elf_with_calories = (0, 0);
    calories.iter().enumerate().for_each(|(elf, cal)| {
        let sum = cal.iter().sum::<i32>();
        if sum > highest_elf_with_calories.1 {
            highest_elf_with_calories = (elf, sum);
        }
    });
    highest_elf_with_calories
}

#[allow(dead_code)]
pub(crate) unsafe fn find_top_elf_with_highest_calories() -> i32 {
    let mut hi_elf_c = 0;
    let mut count = 0;
    let calo = &mut CALORIES;
    while count < 3 {
        let elf = find_elf_with_highest_calories(calo);
        hi_elf_c += elf.1;
        calo.remove(elf.0);
        count += 1;
    }
    hi_elf_c
}
