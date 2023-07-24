mod doenca;

use doenca::Doenca;


fn main() {

    let gripe = Doenca::new(
    String::from("Gripe"),
        vec![String::from("Tosse"), String::from("Febre")],
        String::from("Vírus"),
        String::from("Repouso e medicamentos"),
    );

    println!("{}", gripe);
}
