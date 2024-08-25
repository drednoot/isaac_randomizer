mod randomizer;

use randomizer::characters::Character;

fn main() {
    let ch = Character::new();

    println!("Hello, {}!", ch);
}
