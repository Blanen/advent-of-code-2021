mod one;
mod two;

#[tokio::main]
async fn main() {
    //one::day_one().await;
    two::day_two().await;
}
