
use std::fmt::Debug;

pub fn putone<T: Debug>(a: T) {
    println!("{:?}", a);
}

macro_rules! puts {
    ( $( $a: expr ),* ) => {
        {
            $( print!("{:?} ", $a); )*
            println!("");
        }
    }
}

#[cfg(test)]
mod tests {

    // For these tests to make sense, 
    // run `cargo test -- --nocapture`
    //
    
    #[derive(Debug)]
    enum Color {
        Red,
        Blue,
    }
    
    #[derive(Debug)]
    struct Apple {
        width: i64,
        color: Color,
    }

    #[test]
    fn test_putone() {
        super::putone(2);
    }

    #[test]
    fn test_puts() {
        puts!(2, "hi", &[2,3,4]);
        puts!(Apple { width: 4, color: Color::Blue }, Color::Red)
    }
}
