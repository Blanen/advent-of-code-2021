use tokio::fs;
use itertools::Itertools;

fn count_increments(it: impl Iterator<Item = i32>) -> i32 {
    it.tuple_windows()
    .fold( 0,| acc, (a, b)| {
        if b > a {
            acc + 1
        }
        else{
            acc
        }
    })
}

pub async fn day_one() {
    let input = fs::read_to_string("./data/1.data")
        .await
        .unwrap();

    let input_split_parsed = input
        .split("\n")
        .map(|x| x.parse::<i32>().unwrap());

    let first = count_increments(input_split_parsed.clone());

    let x = input_split_parsed
        .tuple_windows::<(_,_,_)>()
        .map(|(a, b, c)| a + b + c);

    let second = count_increments(x);

    println!("{first}, {second}", first=first, second=second);
}