use socha_plugin_2020::util::{DoubledCoords as Doubled, AxialCoords as Axial};

#[test]
fn doubled_to_axial_coords() {
	assert_eq!(Axial::from(Doubled::new(0, 0)), Axial::new(0, 0));
	assert_eq!(Axial::from(Doubled::new(2, 0)), Axial::new(1, 0));
	assert_eq!(Axial::from(Doubled::new(-1, -1)), Axial::new(0, 1));
	assert_eq!(Axial::from(Doubled::new(3, -1)), Axial::new(2, 1));
	assert_eq!(Axial::from(Doubled::new(0, -4)), Axial::new(2, 4));
	assert_eq!(Axial::from(Doubled::new(-1, -3)), Axial::new(2, 3));
	assert_eq!(Axial::from(Doubled::new(-1, 1)), Axial::new(-1, -1));
	assert_eq!(Axial::from(Doubled::new(-5, -1)), Axial::new(-2, 1));
}

#[test]
fn axial_to_doubled_coords() {
	assert_eq!(Doubled::from(Axial::new(0, 0)), Doubled::new(0, 0));
	assert_eq!(Doubled::from(Axial::new(1, 0)), Doubled::new(2, 0));
	assert_eq!(Doubled::from(Axial::new(0, 1)), Doubled::new(-1, -1));
	assert_eq!(Doubled::from(Axial::new(2, 1)), Doubled::new(3, -1));
	assert_eq!(Doubled::from(Axial::new(2, 4)), Doubled::new(0, -4));
	assert_eq!(Doubled::from(Axial::new(2, 3)), Doubled::new(-1, -3));
	assert_eq!(Doubled::from(Axial::new(-1, -1)), Doubled::new(-1, 1));
	assert_eq!(Doubled::from(Axial::new(-2, 1)), Doubled::new(-5, -1));
}
