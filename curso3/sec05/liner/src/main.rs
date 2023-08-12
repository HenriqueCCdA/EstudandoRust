mod linear_search;
use linear_search::linear_search;

fn main() {
    let nums = vec![1, 2, 3, 4, 5];
    let target = 4;
    let result = linear_search(&nums, target);

    if result == -1 {
        println!("O número {} não foi encontrado no vetor.", target);
    } else {
        println!("O número {} for encontrado no indice {} do vetor", target, result);
    }
}
