use rand::Rng;

pub fn play_football() {
    println!("play football ...");
    let n = rand::thread_rng().gen_range(2..=11);
    println!("the n is {}", n);
}