struct Player {
    health: i32,
    velocity: f32,
}

impl Player {
    fn take_damage(&mut self, amount: i32) {
        self.health -= amount;
    }

    fn move_player(&mut self, speed: f32) {
        self.velocity = speed;
    }
}

fn main() {
    let mut player = Player {
        health: 100,
        velocity: 0.0,
    };

    player.move_player(5.0);
    player.take_damage(20);

    println!("Health: {}, Velocity: {}", player.health, player.velocity);
}