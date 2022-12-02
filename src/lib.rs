mod day_one;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::day_one::CALORIES;

    #[test]
    fn test_elf_with_highest_calories() {
        unsafe {
            let elf = day_one::find_elf_with_highest_calories(&CALORIES);
            assert_eq!(elf.1, 72718)
        }
    }

    #[test]
    fn test_find_top_elf_with_highest_calories() {
        unsafe {
            let elf = day_one::find_top_elf_with_highest_calories();
            assert_eq!(213089, elf)
        }
    }
}
