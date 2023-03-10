fn main() {
    
    let lines = include_str!("../input/input.txt")
        .lines()
        .map(|line| line.parse::<u64>().ok())
        .collect::<Vec<_>>();

    let max = lines
        .split(|line| line.is_none())
        .map(|group| group.iter().map(|x| x.unwrap()).sum::<u64>())
        .max()
        .unwrap();

    println!("{:?}", max);
}
