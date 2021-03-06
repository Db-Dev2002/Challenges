#[inline]
pub fn input() -> Vec<u64> {
    let data: Vec<u64> = include_str!("day1.input")
        .trim()
        .lines()
        .map(|l| l.parse::<u64>().unwrap())
        .collect();
    data
}

//all values should be positive
#[inline]
pub fn part_a() -> u64 {
    let data = input();
    let mut answer = 0;
    for i in data.windows(2) {
        if i[1] > i[0] {
            answer += 1;
        }
    }
    answer
}

#[inline]
pub fn part_b() -> u64 {
    let data = input();
    let mut answer = 0;
    for i in data.windows(4) {
        if i[3] > i[0] {
            answer += 1;
        }
    }
    answer
}
