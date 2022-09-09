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

pub fn clear() {
    // let child = std::process::Command::new("clear").spawn();
    printil!("\r\x1b[2J\r\x1b[H");

    // if child.is_ok() {
    //     let _exit_status = child.unwrap().wait();
    // }
}

// pub fn printil(string: &str) {
//     print!("{string}");
//     _ = std::io::stdout().flush();
// }
