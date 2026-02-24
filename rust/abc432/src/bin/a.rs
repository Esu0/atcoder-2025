use proconio::input;

fn main() {
    input! {
        mut abc: [u32; 3],
    }
    abc.sort_unstable();
    println!("{}", abc[2]*100+abc[1]*10+abc[0]);
}