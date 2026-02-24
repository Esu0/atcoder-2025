use proconio::{input, marker};

fn main() {
    input! {
        mut s: marker::Bytes,
    }
    print!("Of");
    let c  = s[0].to_ascii_lowercase();
    print!("{}", c as char);
    println!("{}", std::str::from_utf8(&s[1..]).unwrap());
}
