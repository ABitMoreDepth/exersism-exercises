pub fn verse(n: i32) -> String {
    let mut output = String::new();
    let n_str: String = n.to_string();

    match n {
        0 => {
            output.push_str("No more bottles of beer on the wall, no more bottles of beer.\n");
            output.push_str("Go to the store and buy some more, 99 bottles of beer on the wall.\n");
        }
        1 => {
            output = "1 bottle of beer on the wall, 1 bottle of beer.\n".to_owned()
                + &"Take it down and pass it around, no more bottles of beer on the wall.\n"
                    .to_string();
        }
        _ => {
            output.push_str(&n_str);
            if n == 1 {
                output.push_str(" bottle of beer on the wall, ");
                output.push_str(&n_str);
                output.push_str(" bottle of beer.\nTake one down and pass it around, ");
            } else {
                output.push_str(" bottles of beer on the wall, ");
                output.push_str(&n_str);
                output.push_str(" bottles of beer.\nTake one down and pass it around, ");
            }
            if n == 2 {
                output.push_str(&(n - 1).to_string());
                output.push_str(" bottle of beer on the wall.\n");
            } else {
                output.push_str(&(n - 1).to_string());
                output.push_str(" bottles of beer on the wall.\n");
            }
        }
    }

    output
}

pub fn sing(start: i32, end: i32) -> String {
    let mut counter: i32 = start;
    let mut output = String::new();

    println!("Start: {}, End: {}", start.to_string(), end.to_string());
    // for counter in (start..end).rev() {
    while counter >= end {
        println!("Creating verse for line {}", counter.to_string());
        output.push_str(&verse(counter));
        counter -= 1;
    }
    output
}
