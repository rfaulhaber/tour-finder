extern crate rand;
use rand::Rng;
use std::collections::HashMap;
use std::time::Instant;

#[derive(Clone, Debug)]
pub struct Point {
	name: String,
	x: i32,
	y: i32,
}

impl Point {
	pub fn new(name: &str, x: i32, y: i32) -> Point {
		let struct_name = String::from(name);
		Point {
			name: struct_name,
			x,
			y,
		}
	}

	pub fn dist(&self, p: &Point) -> f32 {
		(((p.x - self.x).pow(2) + (p.y - self.y).pow(2)) as f32).sqrt()
	}

	pub fn get_name(&self) -> String {
		self.name.clone()
	}
}

pub struct PointSet {
	points: Vec<Point>,
	map: HashMap<String, Point>, // map between point name and point
	distance_table: Vec<Vec<f32>>,
}

impl PointSet {
	pub fn new(points: Vec<Point>) -> PointSet {
		let distance_table = calculate_distance_table(points.clone());

		let mut map: HashMap<String, Point> = HashMap::new();

		for point in points.clone() {
			map.insert(point.get_name(), point);
		}

		PointSet {
			points,
			map,
			distance_table,
		}
	}

	pub fn dist_between(&self, first: String, second: String) -> f32 {
		let first_point = self.map.get(&first).unwrap();
		let second_point = self.map.get(&second).unwrap();

		return first_point.dist(second_point);
	}
}

pub struct Route {
	distance: f32,
	ordered_point_names: Vec<String>,
}

impl Route {
	fn from_path(path: Path) -> Route {
		let mut names: Vec<String> = Vec::new();

		for point in &path.points {
			names.push(point.get_name());
		}

		Route {
			distance: path.distance(),
			ordered_point_names: names,
		}
	}

	pub fn get_result_string(&self) -> String {
		let mut result = String::new();

		let distance_str = format!("distance: {}", self.distance);
		let point_route = self.ordered_point_names.join(" -> ");

		result.push_str(distance_str.as_str());
		result.push_str("\n");
		result.push_str(point_route.as_str());

		return result;
	}
}

pub enum SolveMethod {
	SimulatedAnnealing,
	NearestNeighbor,
}

pub fn solve(set: PointSet, method: SolveMethod) -> Route {
	match method {
		SolveMethod::NearestNeighbor => nearest_neighbor(set),
		SolveMethod::SimulatedAnnealing => simulated_annealing(set),
	}
}

fn simulated_annealing(set: PointSet) -> Route {
	println!("staring annealing");
	let now = Instant::now();
	let mut route = Path::new(random_route(set.points.clone()));

	let mut current_dist = route.distance();

	while now.elapsed().as_secs() < 1 {
		// imposes this limit based on the problem criteria
		let start = rand::thread_rng().gen_range(1, set.points.len());
		let end = rand::thread_rng().gen_range(1, set.points.len());
		route = Path::new(neighboring_tour(route.points.clone(), start, end));

		let dist = route.distance();

		if dist < current_dist {
			current_dist = dist;
			println!("lowering temp, current distance: {}", current_dist);
		}
	}

	return Route::from_path(route);
}

fn random_route(set: Vec<Point>) -> Vec<Point> {
	let mut queue = set.clone();
	let first = queue.remove(0);
	let last = queue.remove(queue.len() - 1);
	let mut current = queue.remove(rand::thread_rng().gen_range(0, queue.len()));

	let mut path: Vec<Point> = Vec::new();

	path.push(current.clone());

	while queue.len() > 0 {
		let random_index = rand::thread_rng().gen_range(0, queue.len());
		current = queue.remove(random_index);
		path.push(current.clone());
	}

	let mut new_vec: Vec<Point> = Vec::new();

	new_vec.push(first);
	new_vec.append(&mut path);
	new_vec.push(last);

	return new_vec;
}

fn neighboring_tour(set: Vec<Point>, start: usize, end: usize) -> Vec<Point> {
	let mut new_subroute: Vec<Point> = Vec::new();
	let mut range_count = 0; // there's a better way to do this, right?

	for index in 0..set.len() {
		if index >= start && index <= end {
			// math is hard, especially when you write code late at night
			let item = set.get(end - range_count).unwrap().clone();
			new_subroute.push(item);
			range_count = range_count + 1;
		} else {
			let item = set.get(index).unwrap().clone();
			new_subroute.push(item);
		}
	}

	return new_subroute;
}

struct Path {
	points: Vec<Point>,
}

impl Path {
	fn new(points: Vec<Point>) -> Path {
		Path { points }
	}

	fn distance(&self) -> f32 {
		let mut distance: f32 = 0.0;

		for i in (0..self.points.len() - 2).step_by(2) {
			let this = &self.points[i];
			let next = &self.points[i + 2];

			distance += this.dist(&next);
		}

		distance
	}
}

fn nearest_neighbor(set: PointSet) -> Route {
	let mut queue = set.points.clone();
	let mut total_distance: f32 = 0.0;
	let mut current = queue.remove(0);

	let mut route_order: Vec<String> = Vec::new();

	route_order.push(current.get_name());

	while queue.len() > 0 {
		let mut min_dist = std::f32::MAX;
		let mut min_dist_index = -1;

		for (index, point) in queue.iter().enumerate() {
			let dist = current.dist(point);
			if dist < min_dist {
				min_dist = dist;
				min_dist_index = index as i32;
			}
		}

		total_distance += min_dist;

		let next_point = queue.remove(min_dist_index as usize);

		route_order.push(next_point.get_name());

		current = next_point;
	}

	let first = set.points.get(0).unwrap();

	total_distance += current.dist(&first);
	route_order.push(first.get_name());

	Route {
		distance: total_distance,
		ordered_point_names: route_order,
	}
}

fn calculate_distance_table(points: Vec<Point>) -> Vec<Vec<f32>> {
	let mut ret: Vec<Vec<f32>> = Vec::new();

	for p1 in points.clone() {
		let mut v: Vec<f32> = Vec::new();

		for p2 in points.clone() {
			v.push(p1.dist(&p2));
		}

		ret.push(v);
	}

	ret
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn dist_calculates_correctly() {
		let p1 = Point {
			x: 549,
			y: 990,
			name: String::from("p1"),
		};
		let p2 = Point {
			x: 220,
			y: 28,
			name: String::from("p2"),
		};

		let result = p1.dist(&p2).floor();

		assert_eq!(result, 1016.0);
	}

	#[test]
	fn calculate_distance_table_correctly_builds_table() {
		let p1 = Point::new("p1", 0, 0);
		let p2 = Point::new("p2", 100, 100);
		let p3 = Point::new("p3", 50, 200);

		let table = calculate_distance_table(vec![p1, p2, p3]);

		let expected_table: Vec<Vec<f32>> = vec![
			vec![0.0, 141.421, 206.155],
			vec![141.421, 0.0, 111.803],
			vec![206.155, 111.803, 0.0],
		];

		for x in 0..table.len() {
			for y in 0..table.get(x).unwrap().len() {
				let actual = *table.get(x).unwrap().get(y).unwrap();
				let expected = *expected_table.get(x).unwrap().get(y).unwrap();

				let result = (actual - expected).abs();

				assert!(result < 0.001)
			}
		}
	}

	#[test]
	fn nearest_neighbor_test() {
		let p1 = Point::new("p1", 0, 0);
		let p2 = Point::new("p2", 100, 100);
		let p3 = Point::new("p3", 50, 200);

		let set = PointSet::new(vec![p1, p2, p3]);

		let result = nearest_neighbor(set);

		assert_eq!(result.ordered_point_names, vec!("p1", "p2", "p3", "p1"));
		assert_eq!(result.distance.floor(), 459.0); // we get the idea
	}

	#[test]
	fn get_result_string_test() {
		let test_result = Route {
			distance: 30.0,
			ordered_point_names: vec![String::from("p1"), String::from("p2"), String::from("p3")],
		};

		let result = test_result.get_result_string();

		assert_eq!(result, "distance: 30\np1 -> p2 -> p3");
	}

	#[test]
	fn random_route_creates_new_route() {
		let points = vec![
			Point::new("p1", 0, 0),
			Point::new("p2", 100, 100),
			Point::new("p3", 50, 200),
			Point::new("p4", 60, 40),
			Point::new("p5", 420, 69),
			Point::new("p6", 1, 2),
			Point::new("p7", 9, 80),
			Point::new("p8", 6, 70),
		];

		let result = random_route(points);

		assert_eq!(result.len(), 8);
		assert_eq!(result.get(0).unwrap().get_name(), "p1");
		assert_eq!(result.get(7).unwrap().get_name(), "p8");
	}

	#[test]
	fn neighboring_route_creates_new_route() {
		let points = vec![
			Point::new("p1", 0, 0),
			Point::new("p2", 100, 100),
			Point::new("p3", 50, 200),
			Point::new("p4", 60, 40),
			Point::new("p5", 420, 69),
			Point::new("p6", 1, 2),
			Point::new("p7", 9, 80),
			Point::new("p8", 6, 70),
		];

		let result = neighboring_tour(points, 1, 4);

		assert_eq!(result.len(), 8);
		assert_eq!(result.get(0).unwrap().get_name(), "p1");
		assert_eq!(result.get(1).unwrap().get_name(), "p5");
		assert_eq!(result.get(2).unwrap().get_name(), "p4");
		assert_eq!(result.get(3).unwrap().get_name(), "p3");
		assert_eq!(result.get(4).unwrap().get_name(), "p2");
		assert_eq!(result.get(5).unwrap().get_name(), "p6");
		assert_eq!(result.get(6).unwrap().get_name(), "p7");
		assert_eq!(result.get(7).unwrap().get_name(), "p8");
	}
}
