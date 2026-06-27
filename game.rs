const MAX_LEVEL: u32 = 10;

fn main() {
    let health = 100;
    let player = "Hero";
    let velocity = 5.0;

    println!("Player: {}, Health: {}, Velocity: {}", player, health, velocity);

    if MAX_LEVEL >= 10 {
        println!("Max level reached");
    }
    
}