mod point;
use std::collections::HashMap;
use std::env;
use std::format;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

struct Tour {
    points: Vec<point::Point>,
}

impl Tour {}

struct PointSet {
    queue: Vec<i32>, // queue of indices
    pointMap: HashMap<String, point::Point>,
    distance_table: Vec<Vec<f32>>,
}

impl PointSet {
    fn new(points: Vec<point::Point>) -> PointSet {
        let mut map: HashMap<String, point::Point> = HashMap::new();

        for point in points.iter() {
            map.insert(point.name, point);
        }

        PointSet {
            pointMap: map,
            queue: (0..points.len() as i32).collect(),
            distance_table: calculate_distance_table(points),
        }
    }

    fn find_nearest_neighbor(&self) -> (i32, bool) {
        if self.queue.len() > 0 {
            let mut min = std::f32::MAX; // there's obviously something wrong with this but whatever

            let current_index = self.queue.remove(0);

            unimplemented!()
        }

        (-1, false)
    }
}

fn calculate_distance_table(points: Vec<point::Point>) -> Vec<Vec<f32>> {
    let mut ret: Vec<Vec<f32>> = Vec::new();

    for (i1, p1) in points.iter().enumerate() {
        let mut v: Vec<f32> = Vec::new();

        for (i2, p2) in points.iter().enumerate() {
            v.push(p1.dist(p2));
        }

        ret.push(v);
    }

    ret
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let filename = &args[1];
        let open_err_message = format!("could not open file: {}", filename);
        let f = File::open(filename).expect(&open_err_message);
    } else {
        let mut buf = String::new();
        from_stdin(&mut buf).expect("failed to read from stdin");

        let points_str = buf.split("\n").collect::<Vec<&str>>();

        println!("from buf: {:?}", points_str);
    }
}

fn from_stdin(mut buffer: &mut String) -> io::Result<()> {
    io::stdin().read_to_string(&mut buffer)?;
    Ok(())
}

// fn calculate_tour(points: Vec<point::Point>) -> Tour {}
