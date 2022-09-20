use std::io::Result;

// either!(condition, expr; expr)
#[macro_export]
macro_rules! either {
    ($cond: expr, $t:expr; $f:expr) => {
        if $cond {
            $t
        } else {
            $f
        }
    };
}

#[macro_export]
macro_rules! printil {

    ($x:tt) => {
        use std::io::{stdout, Write};
        print!("{}",$x);
        stdout().flush();
    };

    ($($x:tt)+$($i:ident),*) => {
        use std::io::{stdout, Write};
        print!($($x)+, $($i)*);
        stdout().flush();
    };
}

pub fn get_input() -> Result<String> {
    let mut buffer = String::new();

    printil!("> ");
    std::io::stdin().read_line(&mut buffer)?;

    Ok(buffer.trim().to_owned())
}

// pub fn clear() {
//     printil!("\r\x1b[2J\r\x1b[H");
// }
