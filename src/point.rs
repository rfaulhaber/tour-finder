pub struct Point {
	pub name: String,
	x: i32,
	y: i32,
}

impl Point {
	pub fn new(name: String, x: i32, y: i32) -> Point {
		Point { name, x, y }
	}

	pub fn dist(&self, p: &Point) -> f32 {
		(((p.x - self.x).pow(2) + (p.y - self.y).pow(2)) as f32).sqrt()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn dist_calculates_correctly() {
		let p1 = Point { x: 549, y: 990 };
		let p2 = Point { x: 220, y: 28 };

		let result = p1.dist(p2).floor();

		assert_eq!(result, 1016.0);
	}
}
