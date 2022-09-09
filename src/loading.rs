use core::time;

use std::thread::sleep;
use std::io;
use std::io::Write;

fn main() {
    let dur = time::Duration::new(1, 0);
    let loading_steps = vec![
        "[          ]",
        "[==        ]",
        "[====      ]",
        "[======    ]",
        "[========  ]",
        "[========= ]",
        "[==========]",
    ];

    let mut current: f64 = 0.0;
    for step in &loading_steps {
        let done_perc = current / loading_steps.len() as f64;
        let done_string = format!("{}{}%", step, (done_perc * 100.0) as i32);

        if current as usize == loading_steps.len() {
            print(&done_string,"");
            continue;
        }

        print(&done_string, "\r");

        sleep(dur);
        current += 1.0;
    }
}

///Automatically prints a string with a carriage return and without a new line
fn print(string: &str, end: &str) {
    // std::process::Command::new("clear").status().unwrap();
    let new_string = format!("{}{}", string, end);

    print!("{}", new_string);
    io::stdout().flush().unwrap();
}
