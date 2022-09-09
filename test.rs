use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    let mut sum = 0;
    for i in 1..1_000_000_000 {
        let n1: u8 = rng.gen();
        sum+= i * n1;
    }

    println!("{sum}");
}
