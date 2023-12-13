fn basic_fn() -> i64 {
    print!("meow");
    42
}

fn fib(n: u32) -> u32 {
    return if n <= 2 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}


fn factorial(n: u32) -> u32 {
    let mut product = 1;
    for i in 1..=n {
        product *= dbg!(i);
    }
    product
}

fn match_fn() -> () {
    let input = 'X';
    match input {
        'q'                       => println!("Quitting"),
        'a' | 's' | 'w' | 'd'     => println!("Moving around"),
        '0'..='9'                 => println!("Number input"),
        key if key.is_lowercase() => println!("Lowercase: {key}"),
        _                         => println!("Something else"),
    } 
}

fn loop_fn() {
    for n in 1..=10 {
        print!("{}", n);
    }
}

fn array_fn() {
    let mut a: [i8; 10] = [42; 10];
    a[5] = 0;
    println!("a: {a:?}");
}

fn string_fn() {
    let mut x = format!("124");
    x = "123".to_string();
    print!("ok: {}", x);
}

fn transpose(matrix: &mut [[i32; 3]; 3]) -> &mut [[i32; 3]; 3] {
    /*
    In Rust, the choice of using usize for array indexing is platform-specific,
    representing the size of memory in bytes on the target architecture. This 
    ensures that the index type aligns with the addressing capabilities of the hardware.
    usize is defined by the target architecture, typically 32 or 64 bits, adapting
    to the maximum object size in memory. Using usize promotes portability by avoiding
    issues that may arise with explicit use of u32 or u64 for array indices on architectures
    with different usize sizes. In summary, usize facilitates writing portable,
    architecture-independent code in Rust, aligning with the language's emphasis on safety
    and performance through adherence to underlying hardware architecture.
    */
    
    let mut start: usize = 0;
    let mut temp: i32;
    
    for i in 0..3 {
        for j in start..3 {
              temp = matrix[i][j];
              matrix[i][j] = matrix[j][i];
              matrix[j][i] = temp;
        }
        start = start + 1;
    }
    
    matrix
}

fn call_transpose() {
    let mut matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];
    
    matrix = *transpose(& mut matrix);
    
    println!("transposed: {:#?}", matrix);
}

fn main() {
    // match_fn();
    // basic_fn();
    // println!("{}", fib(15));
    // loop_fn();
    // array_fn();
    // println!("{}", factorial(3));
    // string_fn();
    // array_fn();
    call_transpose();
    
}