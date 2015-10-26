extern crate polydraw;

use polydraw::geom::point::Point;
use polydraw::geom::ring::Ring;
use polydraw::geom::lineinter::{h_multi_intersect, h_multi_intersect_fast};

fn main() {
   let mut inters = Ring::new(10_000);
   let p1 = Point::new(2135, 2476);
   let p2 = Point::new(16753, 72398);

   println!("SLOW");

   h_multi_intersect(p1, p2, 1000, &mut inters);

   for x in inters[..].iter() {
      println!("X : {}", x);
   }

   inters.consume();

   println!("FAST");

   h_multi_intersect_fast(p1, p2, 1000, &mut inters);

   for x in inters[..].iter() {
      println!("X : {}", x);
   }
}
