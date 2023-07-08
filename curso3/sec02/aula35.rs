enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
enum Gender{
    Male, Female
}

fn main() {

    let player:Direction = Direction::Right;

    match player {
        Direction::Up => println!("O jogador foi para cima"),
        Direction::Down => println!("O jogador foi para baixo"),
        Direction::Right => println!("O jogador foi para direita"),
        Direction::Left => println!("O jogador foi para esquerda"),
    }

    let player_male:Gender = Gender::Male;
    let player_female:Gender = Gender::Female;

    println!("{:?}", player_male);
    println!("{:?}", player_female);

}
