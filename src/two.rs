use tokio::fs;
use itertools::Itertools;

pub async fn day_two() {
    let input = fs::read_to_string("./data/2.data")
        .await
        .unwrap();

    let input_split_and_parsed = input
        .split("\n")
        .map(|x| x.split(" "));
    
    let location = input_split_and_parsed
        .fold((0, 0), |acc, line| {
            let parsed_line = line
            .tuple_windows()
            .fold(("", 0), |_, (a, b)| {
                let direction  = a;
                let distance  = b.parse::<i32>().unwrap();
                (direction, distance)
            });
            let sub_vector = match parsed_line.0 {
                "up" => (0,-parsed_line.1),
                "down" => (0,parsed_line.1),
                "forward" => (parsed_line.1, 0),
                _ => (0,0)
            };
            (acc.0 + sub_vector.0, acc.1 + sub_vector.1)
        });
    let answer = location.0 * location.1;
    println!("{answer}", answer=answer);
}