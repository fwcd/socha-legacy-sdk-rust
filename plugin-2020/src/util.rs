use arrayvec::ArrayVec;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign};
use socha_client_base::xml_node::XmlNode;
use socha_client_base::hashmap;

/// Axial coordinates on the hex grid.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct AxialCoords {
	x: i32,
	y: i32
}

/// Cube coordinates on the hex grid.
/// These are used by the protocol internally.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct CubeCoords {
	x: i32,
	y: i32,
	z: i32
}

/// An iterator that returns coordinates on
/// a straight line.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct LineIter {
	current: CubeCoords,
	destination: CubeCoords,
	step: CubeCoords
}

pub trait LineFormable {
	/// Tests whether `self` and `rhs` form a
	/// straight line.
	fn forms_line_with(self, rhs: Self) -> bool;
	
	/// Fetches the elements _between_ `self` and `rhs`
	/// in cube coordinates.
	fn line_iter(self, rhs: Self) -> LineIter;
}

pub trait Adjacentable {
	/// Tests whether `self` and `rhs` are neighbors.
	fn is_adjacent_to(self, rhs: Self) -> bool;
}

impl AxialCoords {
	/// Creates new axial coordinates.
	#[inline]
	pub fn new(x: i32, y: i32) -> Self { Self { x: x, y: y } }
	
	/// Fetches the x-coordinate
	#[inline]
	pub fn x(self) -> i32 { self.x }
	
	/// Fetches the y-coordinate
	#[inline]
	pub fn y(self) -> i32 { self.y }

	/// Fetches all 6 neighbors, regardless of any board
	/// boundaries.
	#[inline]
	pub fn coord_neighbors(self) -> ArrayVec<[AxialCoords; 6]> {
		ArrayVec::from([
			self + AxialCoords::new(-1, 0),
			self + AxialCoords::new(0, 1),
			self + AxialCoords::new(1, 1),
			self + AxialCoords::new(1, 0),
			self + AxialCoords::new(0, -1),
			self + AxialCoords::new(-1, -1)
		])
	}
}

impl CubeCoords {
	/// Creates new (unvalidated) cube coordinates.
	#[inline]
	pub fn new(x: i32, y: i32, z: i32) -> Self {
		Self { x: x, y: y, z: z }
	}

	/// Creates new cube coordinates if they are valid.
	#[inline]
	pub fn new_valid(x: i32, y: i32, z: i32) -> Option<Self> {
		if (x + y + z) == 0 {
			Some(CubeCoords { x: x, y: y, z: z })
		} else {
			None
		}
	}
	
	/// Fetches the x-coordinate
	#[inline]
	pub fn x(self) -> i32 { self.x }
	
	/// Fetches the y-coordinate
	#[inline]
	pub fn y(self) -> i32 { self.y }
	
	/// Fetches the z-coordinate
	#[inline]
	pub fn z(self) -> i32 { self.z }
}

impl<C> LineFormable for C where C: Into<CubeCoords> {
	fn forms_line_with(self, rhs: Self) -> bool {
		let lhs_cube = self.into();
		let rhs_cube = rhs.into();
		lhs_cube.x == rhs_cube.x || lhs_cube.y == rhs_cube.y || lhs_cube.z == rhs_cube.z
	}
	
	fn line_iter(self, rhs: Self) -> LineIter {
		let lhs_cube = self.into();
		let rhs_cube = rhs.into();
		let diff = rhs_cube - lhs_cube;
		let step = CubeCoords::new(diff.x().signum(), diff.y().signum(), diff.z().signum());
		LineIter::new(lhs_cube + step, step, rhs_cube)
	}
}

impl LineIter {
	pub fn new(start: CubeCoords, step: CubeCoords, destination: CubeCoords) -> Self {
		Self { current: start, step: step, destination: destination }
	}
}

impl Iterator for LineIter {
	type Item = CubeCoords;
	
	fn next(&mut self) -> Option<CubeCoords> {
		if self.current == self.destination {
			None
		} else {
			let pos = self.current;
			self.current += self.step;
			Some(pos)
		}
	}
}

impl<C> Adjacentable for C where C: Into<AxialCoords> {
	fn is_adjacent_to(self, rhs: Self) -> bool {
		let lhs_axial = self.into();
		let rhs_axial = rhs.into();
		lhs_axial.coord_neighbors().iter().any(|&c| c == rhs_axial)
	}
}

impl Add for AxialCoords {
	type Output = Self;

	fn add(self, rhs: Self) -> Self { Self { x: self.x + rhs.x, y: self.y + rhs.y } }
}

impl Sub for AxialCoords {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self { Self { x: self.x - rhs.x, y: self.y - rhs.y } }
}

impl<R> Mul<R> for AxialCoords where R: Into<i32> {
	type Output = Self;
	
	fn mul(self, rhs: R) -> Self {
		let other = rhs.into();
		Self { x: self.x * other, y: self.y * other }
	}
}

impl AddAssign for AxialCoords {
	fn add_assign(&mut self, rhs: Self) {
		self.x += rhs.x;
		self.y += rhs.y;
	}
}

impl SubAssign for AxialCoords {
	fn sub_assign(&mut self, rhs: Self) {
		self.x -= rhs.x;
		self.y -= rhs.y;
	}
}

impl<R> MulAssign<R> for AxialCoords where R: Into<i32> {
	fn mul_assign(&mut self, rhs: R) {
		let r = rhs.into();
		self.x *= r;
		self.y *= r;
	}
}

impl Add for CubeCoords {
	type Output = Self;

	fn add(self, rhs: Self) -> Self { Self { x: self.x + rhs.x, y: self.y + rhs.y, z: self.y + rhs.z } }
}

impl Sub for CubeCoords {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self { Self { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z } }
}

impl<R> Mul<R> for CubeCoords where R: Into<i32> {
	type Output = Self;
	
	fn mul(self, rhs: R) -> Self {
		let other = rhs.into();
		Self { x: self.x * other, y: self.y * other, z: self.z * other }
	}
}

impl AddAssign for CubeCoords {
	fn add_assign(&mut self, rhs: Self) {
		self.x += rhs.x;
		self.y += rhs.y;
		self.z += rhs.z;
	}
}

impl SubAssign for CubeCoords {
	fn sub_assign(&mut self, rhs: Self) {
		self.x -= rhs.x;
		self.y -= rhs.y;
		self.z -= rhs.z;
	}
}

impl<R> MulAssign<R> for CubeCoords where R: Into<i32> {
	fn mul_assign(&mut self, rhs: R) {
		let r = rhs.into();
		self.x *= r;
		self.y *= r;
		self.z += r;
	}
}

impl From<CubeCoords> for AxialCoords {
	fn from(coords: CubeCoords) -> Self { Self { x: coords.x, y: coords.y } }
}

impl From<AxialCoords> for CubeCoords {
	fn from(coords: AxialCoords) -> Self { Self { x: coords.x, y: coords.y, z: -(coords.x + coords.y) } }
}

impl From<CubeCoords> for XmlNode {
	fn from(coords: CubeCoords) -> Self {
		Self::new(
			"CubeCoordinates",
			"",
			hashmap!["x".to_owned() => coords.x().to_string(), "y".to_owned() => coords.y().to_string(), "z".to_owned() => coords.z().to_string()],
			vec![]
		)
	}
}