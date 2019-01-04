mod point;
use std::env;
use std::format;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();

    // TODO clean up
    if args.len() > 1 {
        let filename = &args[1];
        let open_err_message = format!("could not open file: {}", filename);
        let f = File::open(filename).expect(&open_err_message);

        let mut point_set: Vec<point::Point> = Vec::new();

        for (line_num, line) in BufReader::new(f).lines().map(|l| l.unwrap()).enumerate() {
            point_set.push(get_point_from_string(line, line_num.to_string()));
        }

        let set = point::PointSet::new(point_set);

        let start = Instant::now();
        let result = point::solve(set, point::SolveMethod::NearestNeighbor);

        println!("{}", result.get_result_string());
        println!("elapsted time: {:?}", start.elapsed());
    } else {
        let mut buf = String::new();
        from_stdin(&mut buf).expect("failed to read from stdin");

        let points_str = buf.split("\n").collect::<Vec<&str>>();

        let mut point_set: Vec<point::Point> = Vec::new();

        for (index, point_str) in points_str.iter().enumerate() {
            point_set.push(get_point_from_string(
                String::from(*point_str),
                index.to_string(),
            ));
        }

        let set = point::PointSet::new(point_set);

        let start = Instant::now();
        let result = point::solve(set, point::SolveMethod::NearestNeighbor);

        println!("{}", result.get_result_string());
        println!("elapsted time: {:?}", start.elapsed());
    }
}

fn from_stdin(mut buffer: &mut String) -> io::Result<()> {
    io::stdin().read_to_string(&mut buffer)?;
    Ok(())
}

// TODO check for errors!
fn get_point_from_string(string: String, name: String) -> point::Point {
    let points_str: Vec<&str> = string
        .trim_left()
        .split(" ")
        .filter(|s| !s.is_empty())
        .collect();

    let x = points_str[0].parse::<i32>().unwrap();
    let y = points_str[1].parse::<i32>().unwrap();

    point::Point::new(name.as_str(), x, y)
}
