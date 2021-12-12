use tokio::fs;
use itertools::Itertools;

pub async fn day_one() {
    let input = fs::read_to_string("./data/1.data")
        .await
        .unwrap();

    let input_split_parsed = input
        .split("\n")
        .map(|x| x.parse::<i32>().unwrap());

    let x = input_split_parsed
        .tuple_windows()
        .fold( 0,| acc, (a, b)| {
            if b > a {
                acc + 1
            }
            else{
                acc
            }
        });

    println!("{x}", x=x);
}