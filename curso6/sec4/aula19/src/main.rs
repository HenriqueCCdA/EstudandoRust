// use crate::archive::arch::arch_file;
use crate::archive::arch::arch_file as arc;



mod archive;

fn main() {
    // arch_file("somefile.txt");
    arc("somefile.txt");
}
