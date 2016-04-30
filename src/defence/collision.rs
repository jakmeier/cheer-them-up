//! Independent module to compute collision between collection of equally shaped objects and other objects. 

use super::tower::Tower;
use super::enemy::Enemy;

/*pub trait RectangleShape {
	fn get_rectangle_shape(&self) -> [f64;4];
}*/

pub fn towers_with_rectangle(collection: &Vec<Box<Tower>>, x: f64, y: f64, w: f64, h: f64) -> bool {
	for t in collection.iter() {
		let (x0,y0) = t.get_coordinates();
		let (w0,h0) = t.get_tower_size();
		if !((x+w) < x0 || x > (x0 + w0) || (y+h) < y0 || y > (y0 + h0))
			{ return true; }
	}
	false
}

pub fn enemies_with_rectangle(collection: &Vec<Box<Enemy>>, x: f64, y: f64, w: f64, h: f64) -> bool {
	for t in collection.iter() {
		let (x0,y0) = t.get_coordinates();
		let (w0,h0) = t.get_size();
		if !((x+w) < x0 || x > (x0 + w0) || (y+h) < y0 || y > (y0 + h0))
			{ return true; }
	}
	false
}