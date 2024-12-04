pub mod main {
    use std::fs::read_to_string;
    pub fn solution() {
        let mut leftcol = Vec::new();
        let mut rightcol = Vec::new();

        for line in read_to_string("input/day1").unwrap().lines() {
            let splitline: Vec<&str> = line.split("   ").collect();
            leftcol.push(splitline[0].to_string());
            rightcol.push(splitline[1].to_string());
        }

        leftcol.sort();
        rightcol.sort();

        let mut day1_result = 0;
        let mut day2_result = 0;
        for x in 0..leftcol.len() {
            let non_negative_diff = leftcol[x]
                .parse::<i32>()
                .unwrap()
                .abs_diff(rightcol[x].parse::<i32>().unwrap());
            day1_result += non_negative_diff;

            let numofapperence = rightcol.iter().filter(|&n| *n == leftcol[x]).count();
            day2_result += numofapperence as i32 * leftcol[x].parse::<i32>().unwrap();
        }
        println!("--- Day 1: Historian Hysteria --- \n Day1: {day1_result}, Day2: {day2_result}");
    }
}
