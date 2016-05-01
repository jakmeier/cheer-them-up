//! Independent module to compute collision between collection of equally shaped objects and other objects. 

use constants::EPS;
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


pub fn find_closest_enemy(x: f64, y: f64, enemies: &Vec<Box<Enemy>>) -> Option<usize> {
	let mut result = None;
	for (i, e) in enemies.iter().enumerate() {
		let (e_x, e_y) = e.get_coordinates();
		let (e_w, e_h) = e.get_size();
		let e_x = e_x + e_w/2.0;
		let e_y = e_y + e_h/2.0;
		let distance = ((e_x-x)*(e_x-x) + (e_y-y)*(e_y-y)).sqrt();
		if let Some((old_distance, _)) = result {
			if old_distance > distance { result = Some ((distance, i)); } 
		}
		else { result = Some((distance, i)); }
	}
	if let Some ((_, index)) = result { Some(index) }
	else {None}
}

/// Segment is represented with the starting point at [x0,y0] and a vector [s,t]. 
/// The end point of the segment is then [x0+s,y0+t].
pub fn enemies_with_segment(collection: &Vec<Box<Enemy>>, x0: f64, y0: f64, s: f64, t: f64) -> Option<usize> {
	for (i, e) in collection.iter().enumerate() {
		let (x,y) = e.get_coordinates();
		let (w,h) = e.get_size();
		// test intersection with all four edges of the enemy ...
		if segment_intersection( [x,y,w,0.0], [x0,y0,s,t] )
			|| segment_intersection( [x,y,0.0,h], [x0,y0,s,t] )
			|| segment_intersection( [x+w, y, 0.0, h ], [x0,y0,s,t] )
			|| segment_intersection( [x, y+h, w, 0.0], [x0,y0,s,t] )
			{ return Some(i); }
		//than check wheter it is contained in the enemy
		if x0 >= x && x0 <= x+w 
			&& x0+s >= x && x0+s <= x+w
			&& y0 >= y && y0 <= y+h
			&& y0+t >= y && y0+t <= y+h
		{ return Some(i); }
	}
	None
}

// Input should be of form [x0,y0,w0,h0], [x1,y1,w1,h1]
fn segment_intersection( a:[f64;4], b:[f64;4] ) -> bool {
	let p = ((b[0]-a[0])*b[2] - (b[1]-a[1])*b[3]).abs();
	let r = ((b[0]-a[0])*a[2] - (b[1]-a[1])*a[3]).abs();
	let s = (a[2]*b[3]-a[3]*b[2]).abs();
	let t = p/s;
	let u = r/s;
	
	if r < EPS && s < EPS { 
		// Collinear
		let q = a[2]*a[2]+a[3]*a[3];
		let t0 = ((b[0]-a[0])*a[2] + (b[1]-a[1])*a[3]) / q;
		let t1 = t0 + (b[2]*a[2] + b[3]*b[3])/q;
		// test [t0,t1] on interval [0,1]
		if  (t0 <= 1.0 && t1 >= 1.0)
			|| (t1 <= 1.0 && t0 >= 1.0)
			|| (t0 <= 0.0 && t1 >= 0.0)
			|| (t1 <= 0.0 && t0 >= 0.0)
			|| (t1 <= 1.0 && t1 >= 0.0)
			|| (t0 <= 1.0 && t0 >= 0.0)
			{ true }
		else {false}
	}
	else if s < EPS {
		// parallel
		false
	}
	else if t >= 0.0 && t <= 1.0 && u >= 0.0 && u <= 1.0 {
		true
	}
	else { false }
}

#[test]
fn segment_intersection_test(){
	assert!( segment_intersection([0.0,0.0,1.0,1.0],[1.0,0.0,-1.0,1.0]) ); // intersecting
	assert!( !segment_intersection([0.0,0.0,1.0,1.0],[1.0,0.0,1.0,1.0]) ); // parallel
	assert!( segment_intersection([0.0,0.0,1.0,1.0],[0.5,0.5,1.0,1.0]) ); // collinear
	assert!( segment_intersection([0.0,0.0,1.0,1.0],[0.5,0.5,-1.0,-1.0]) ); // collinear with different directions
	assert!( !segment_intersection([0.0,0.0,1.0,1.0],[1.5,1.5,1.0,1.0]) ); // collinear but no intersection
	assert!( !segment_intersection([1.0,0.0,1.0,1.0],[2.0,1.5,-1.0,-1.0]) ); // no intersection but not parallel
}
