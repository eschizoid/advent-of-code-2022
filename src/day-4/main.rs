use std::env;
use std::fs;
use std::io::Result;
use std::path::PathBuf;

fn get_current_working_dir() -> Result<PathBuf> {
    env::current_dir()
}

fn main() {
    let working_dir = get_current_working_dir();
    let file_path = format!("{}/src/day-4/input.txt", working_dir.unwrap().display());
    let contents = fs::read_to_string(file_path).expect("Unable to read file");
    let mut counter = 0;

    contents.lines().for_each(|line| {
        let line = line.split(",").collect::<Vec<&str>>();
        let pair1 = line[0];
        let pair2 = line[1];

        let section1 = pair1.split("-").collect::<Vec<&str>>();
        let section1_start = section1[0].parse::<i32>().unwrap();
        let section1_end = section1[1].parse::<i32>().unwrap();

        let section2 = pair2.split("-").collect::<Vec<&str>>();
        let section2_start = section2[0].parse::<i32>().unwrap();
        let section2_end = section2[1].parse::<i32>().unwrap();

        let difference1 = section1_end - section1_start;
        let difference2 = section2_end - section2_start;

        if (section1_start >= section2_start
            && section1_end <= section2_end
            && difference1 <= difference2)
            == true
        {
            counter += 1;
        } else if (section2_start >= section1_start
            && section2_end <= section1_end
            && difference2 <= difference1)
            == true
        {
            counter += 1;
        }
    });
    println!("{}", counter);
}
