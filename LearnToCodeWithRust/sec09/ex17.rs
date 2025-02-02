// Hours, minutes
struct ShortDuration(u32, u32);
//Years, months
struct LongDuration(u32, u32);

fn main() {
    let work_shift = ShortDuration(8, 0);
    println!("{} hours {} minutes", work_shift.0, work_shift.1);

    let era = LongDuration(5, 3);
    println!("{} years {} months", era.0, era.1);

    go_to_work(work_shift);
}

fn go_to_work(length: ShortDuration) {
    println!("Passing time {} hours {} minutes", length.0, length.1);
}
