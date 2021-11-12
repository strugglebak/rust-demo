fn main() {
    // dangling reference
    // let r;
    // {
    //     let x = 5;
    //     // r is referring to has gone out of scope
    //     r = &x;
    // }

    // println!("r: {}", r);

    {
        let x = 5;            // ----------+-- 'b
                              //           |
        let r = &x;           // --+-- 'a  |
                              //   |       |
        println!("r: {}", r); //   |       |
                              // --+       |
    }                         // ----------+
}
