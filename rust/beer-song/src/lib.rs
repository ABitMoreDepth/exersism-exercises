pub fn verse(n: i32) -> String {
    let mut output = String::new();

    match n {
        0 => {
            output.clear();
            output.push_str(concat!(
                "No more bottles of beer on the wall, no more bottles of beer.\n",
                "Go to the store and buy some more, 99 bottles of beer on the wall.\n",
            ));
        }
        _ => {
            output.push_str(&format!(
                concat!(
                    "{num} bottle{plural} of beer on the wall, {num} bottle{plural} ",
                    "of beer.\nTake {singular} down and pass it around, {remaining} ",
                    "bottle{rem_plural} of beer on the wall.\n",
                ),
                num = n,
                plural = if n > 1 { "s" } else { "" },
                singular = if n == 1 { "it" } else { "one" },
                remaining = if n > 1 {
                    (n - 1).to_string()
                } else {
                    "no more".to_string()
                },
                rem_plural = if n > 2 || n == 1 { "s" } else { "" },
            ));
        }
    }

    output
}

pub fn sing(start: i32, end: i32) -> String {
    let mut counter: i32 = start;
    let mut output = String::new();

    while counter >= end {
        output.push_str(&format!(
            "{}{}",
            &verse(counter),
            if counter - 1 >= end { "\n" } else { "" }
        ));
        counter -= 1;
    }
    output
}
