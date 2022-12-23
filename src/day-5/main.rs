use std::collections::VecDeque;
use std::env;
use std::fs;
use std::io::Result;
use std::path::PathBuf;

fn get_current_working_dir() -> Result<PathBuf> {
    env::current_dir()
}

fn get_crates() -> Vec<VecDeque<&'static str>> {
    let mut crates = Vec::new();
    let mut crate_1 = VecDeque::new();
    let mut crate_2 = VecDeque::new();
    let mut crate_3 = VecDeque::new();
    let mut crate_4 = VecDeque::new();
    let mut crate_5 = VecDeque::new();
    let mut crate_6 = VecDeque::new();
    let mut crate_7 = VecDeque::new();
    let mut crate_8 = VecDeque::new();
    let mut crate_9 = VecDeque::new();

    //<editor-fold desc="Crates">
    //                 [B] [L]     [J]
    //             [B] [Q] [R]     [D] [T]
    //             [G] [H] [H] [M] [N] [F]
    //         [J] [N] [D] [F] [J] [H] [B]
    //     [Q] [F] [W] [S] [V] [N] [F] [N]
    // [W] [N] [H] [M] [L] [B] [R] [T] [Q]
    // [L] [T] [C] [R] [R] [J] [W] [Z] [L]
    // [S] [J] [S] [T] [T] [M] [D] [B] [H]
    //  1   2   3   4   5   6   7   8   9
    //</editor-fold>

    //<editor-fold desc="Crate 1">
    crate_1.push_back("W");
    crate_1.push_back("L");
    crate_1.push_back("S");
    //</editor-fold>

    //<editor-fold desc="Crate 2">
    crate_2.push_back("Q");
    crate_2.push_back("N");
    crate_2.push_back("T");
    crate_2.push_back("J");
    //</editor-fold>

    //<editor-fold desc="Crate 3">
    crate_3.push_back("J");
    crate_3.push_back("F");
    crate_3.push_back("H");
    crate_3.push_back("C");
    crate_3.push_back("S");
    //</editor-fold>

    //<editor-fold desc="Crate 4">
    crate_4.push_back("B");
    crate_4.push_back("G");
    crate_4.push_back("N");
    crate_4.push_back("W");
    crate_4.push_back("M");
    crate_4.push_back("R");
    crate_4.push_back("T");
    //</editor-fold>

    //<editor-fold desc="Crate 5">
    crate_5.push_back("B");
    crate_5.push_back("Q");
    crate_5.push_back("H");
    crate_5.push_back("D");
    crate_5.push_back("S");
    crate_5.push_back("L");
    crate_5.push_back("R");
    crate_5.push_back("T");
    //</editor-fold>

    //<editor-fold desc="Crate 6">
    crate_6.push_back("L");
    crate_6.push_back("R");
    crate_6.push_back("H");
    crate_6.push_back("F");
    crate_6.push_back("V");
    crate_6.push_back("B");
    crate_6.push_back("J");
    crate_6.push_back("M");
    //</editor-fold>

    //<editor-fold desc="Crate 7">
    crate_7.push_back("M");
    crate_7.push_back("J");
    crate_7.push_back("N");
    crate_7.push_back("R");
    crate_7.push_back("W");
    crate_7.push_back("D");
    //</editor-fold>

    //<editor-fold desc="Crate 8">
    crate_8.push_back("J");
    crate_8.push_back("D");
    crate_8.push_back("N");
    crate_8.push_back("H");
    crate_8.push_back("F");
    crate_8.push_back("T");
    crate_8.push_back("Z");
    crate_8.push_back("B");
    //</editor-fold>

    //<editor-fold desc="Crate 9">
    crate_9.push_back("T");
    crate_9.push_back("F");
    crate_9.push_back("B");
    crate_9.push_back("N");
    crate_9.push_back("Q");
    crate_9.push_back("L");
    crate_9.push_back("H");
    //</editor-fold>

    crates.push(crate_1);
    crates.push(crate_2);
    crates.push(crate_3);
    crates.push(crate_4);
    crates.push(crate_5);
    crates.push(crate_6);
    crates.push(crate_7);
    crates.push(crate_8);
    crates.push(crate_9);
    return crates;
}

fn main() {
    let working_dir = get_current_working_dir();
    let file_path = format!("{}/src/day-5/input.txt", working_dir.unwrap().display());
    let contents = fs::read_to_string(file_path).expect("Unable to read file");

    let mut crates = get_crates();
    contents.lines().for_each(|line| {
        let t: String = line.chars().filter(|c| c.is_digit(10)).collect();
        let mut amount = String::from("");
        let mut from = String::from("");
        let mut to = String::from("");
        match t.len() == 4 {
            true => {
                amount = format!("{}{}", t.chars().nth(0).unwrap(), t.chars().nth(1).unwrap());
                from = format!("{}", t.chars().nth(2).unwrap());
                to = format!("{}", t.chars().nth(3).unwrap());
            }
            false => {
                amount = format!("{}", t.chars().nth(0).unwrap());
                from = format!("{}", t.chars().nth(1).unwrap());
                to = format!("{}", t.chars().nth(2).unwrap());
            }
        }

        let mut source = crates
            .get(from.parse::<usize>().unwrap() - 1)
            .unwrap()
            .clone();
        let mut target = crates
            .get(to.parse::<usize>().unwrap() - 1)
            .unwrap()
            .clone();

        for _ in 0..amount.parse::<usize>().unwrap() {
            let element_to_move = source.pop_front().unwrap();
            target.push_front(element_to_move);
        }
        crates[from.parse::<usize>().unwrap() - 1] = source;
        crates[to.parse::<usize>().unwrap() - 1] = target;
    });
    println!(
        "{}{}{}{}{}{}{}{}{}",
        crates[0].get(0).unwrap(),
        crates[1].get(0).unwrap(),
        crates[2].get(0).unwrap(),
        crates[3].get(0).unwrap(),
        crates[4].get(0).unwrap(),
        crates[5].get(0).unwrap(),
        crates[6].get(0).unwrap(),
        crates[7].get(0).unwrap(),
        crates[8].get(0).unwrap()
    );
}
