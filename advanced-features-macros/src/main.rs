// #[macro_export]
// macro_rules! vec {
//     ( $( $x:expr ),* ) => {
//         {
//             let mut temp_vec = Vec::new();
//             $(
//                 temp_vec.push($x);
//             )*
//             temp_vec
//         }
//     };
// }

// ↓

// {
//     let mut temp_vec = Vec::new();
//     temp_vec.push(1);
//     temp_vec.push(2);
//     temp_vec.push(3);
//     temp_vec
// }

// use proc_macro;

// #[some_attribute]
// pub fn some_name(input: TokenStream) -> TokenStream {
// }

fn main() {
    println!("Hello, world!");
}
