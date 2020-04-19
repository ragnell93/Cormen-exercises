use chapter3::Stack;

fn main() {
    println!("Creating a new stack of integers");
    let mut st: Stack<i32> = Stack::new(3);
    st.push(1).unwrap();
    st.push(2).unwrap();
    //st.push(5).unwrap();
    println!("{}", st.pop().unwrap());
}
