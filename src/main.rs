use std::io;

fn keycode(c: &str) -> String {
    match c {
        "," => "COMMA".to_owned(),
        "." => "DOT".to_owned(),
        ";" => "SCLN".to_owned(),
        "'" => "SQT".to_owned(),
        "-" => "MINUS".to_owned(),
        "_" => "UNDER".to_owned(),
        "\"" => "DQT".to_owned(),
        "␣" => "SPACE".to_owned(),
        "⟐" => "arcane".to_owned(),
        a => a.to_uppercase()
    }
}

fn keypress(c: &str) -> String {
    format!("&kp {}", c)
}

fn autoshift(c: &str) -> String {
    format!("&as {}", c)
}


fn parse_line(line: String) -> Vec<String> {
    let mut row = vec![];
    for subs in line.trim().split(" ") {
        row.push(keycode(&subs));
    }
    row
}

fn key_at<'a>(keys: &'a Vec<Vec<String>>, row: usize ,col: usize) -> &'a String {
    let keys_at_row = &keys[row];
    &keys_at_row[col]
}

fn main() -> io::Result<()>{
    println!("Hello, world!");
    let mut buffer = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    let mut rows = vec![];
    let lines = io::stdin().lines();
    for line in lines {
        let row = parse_line(line.unwrap());
        // let l = line.unwrap().trim();
        // let chars : Vec<&str> = l.split(" ").collect();
        // let mut row = vec![]; // chars.map(|c| keycode(c)).collect();
        // for c in &chars {
            // row.push(keycode(c));
            // println!("Char: {}", keycode(c));
            // println!("{}", keypress(&keycode(c)));
        // }
        rows.push(row);
    }

    println!("{:?}", rows);

    for row in [0,1,2] {
        for col in 0 .. 10 {
            print!("{} ", key_at(&rows, row, col));
        }
        println!("");
    }
    
    Ok(())
}
