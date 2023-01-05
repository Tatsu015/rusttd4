use tokio::time::{sleep, Duration};

fn fetch() {
    println!("Fetch program")
}

fn pc_up() {
    println!("PC count up")
}

fn decode() {
    println!("Decode instruction")
}

fn execute() {
    println!("Execute instruction")
}

#[tokio::main]
async fn main() {
    loop {
        fetch();
        pc_up();
        decode();
        execute();
        sleep(Duration::from_millis(1000)).await;
    }
}
