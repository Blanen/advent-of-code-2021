mod one;
mod two;
mod three;

#[tokio::main]
async fn main() {
    one::day_one().await;
    //two::day_two().await;
    //three::day_three().await;
}
