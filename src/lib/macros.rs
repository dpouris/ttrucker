#[macro_export]
macro_rules! select {
    ($key: ident, $idx: ident, $max_idx: expr, $($highlight: tt)*) => {
            match $key {
                Key::ArrowUp => {
                    if $idx > 1 {
                        $idx -= 1;
                        $($highlight)*($idx)
                    }
                }
                Key::ArrowDown => {
                    if $idx != $max_idx as i32 {
                        $idx += 1;
                        $($highlight)*($idx)
                    }
                }
                Key::Enter => break,
                _ => (),
            }
    };

    ($key: ident, $idx: ident, $max_idx: tt) => {
            match $key {
                Key::ArrowUp => {
                    if $idx > 1 {
                        $idx -= 1;
                    }
                }
                Key::ArrowDown => {
                    if $idx != $max_idx as i32 {
                        $idx += 1;
                    }
                }
                Key::Enter => break,
                _ => (),
            }
        $idx
    }
}

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