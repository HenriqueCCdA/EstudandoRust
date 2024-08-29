use std::thread;

fn main() {

    let data = "1186132257556472396329754262496285070856
    234701860851907960690014725639383979667071
    060941727832387476692195238079525788823652
    5459303330302837584953271357440410488978857
    342978126992021643898087354880841372095653
    216278424637452589860345374828574668";

    let mut children = vec![];

    let chunked_data = data.split_whitespace();

    for (i, data_segment) in chunked_data.enumerate() {
        println!("data segment {} is \"{}\"", i, data_segment);

        children.push(thread::spawn(move || -> u32 {
            let result = data_segment
                        .chars()
                        .map(|c| c.to_digit(10).expect("should be a digit"))
                        .sum();
            println!("processed segment {}, result={}", i, result);
            result
        }));
    }

    let final_result = children.into_iter().map(|c| c.join().unwrap()).sum::<u32>();

    println!("Final sum result: {}", final_result);
}
