extern crate sdl2;

pub mod triangle {
    use sdl2::rect::Point;
    use sdl2::render::Canvas;
    use sdl2::video::Window;

    fn fill_bottom_flat_triangle(points: &[Point; 3], canvas: &mut Canvas<Window>) {
        //println!("fill_bottom_flat_triangle");
        let invslope1: f64 = (points[1].x as f64 - points[0].x as f64) / (points[1].y as f64 - points[0].y as f64);
        let invslope2: f64 = (points[2].x as f64 - points[0].x as f64) / (points[2].y as f64 - points[0].y as f64);
        //println!("btf {:?}", points);
        //println!("btf ({}-{})/({}-{})={}", points[2].x, points[1].x, points[2].y, points[1].y, invslope2);

        let mut curx1: f64 = points[0].x as f64;
        let mut curx2: f64 = points[0].x as f64;
        for scanline_y in points[0].y-1..points[1].y {
            let p1 = Point::new(curx1 as i32, scanline_y);
            let p2 = Point::new(curx2 as i32, scanline_y);
            //println!("bft {:?} -> {:?}", p1, p2);
            canvas.draw_line(p1, p2).unwrap();
            curx1 += invslope1;
            curx2 += invslope2;
        }
    }

    fn fill_top_flat_triangle(points: &[Point; 3], canvas: &mut Canvas<Window>) {
        //println!("fill_top_flat_triangle");
        let invslope1: f64 = (points[2].x as f64 - points[0].x as f64) / (points[2].y as f64 - points[0].y as f64);
        let invslope2: f64 = (points[2].x as f64 - points[1].x as f64) / (points[2].y as f64 - points[1].y as f64);
        //println!("ftf {:?}", points);
        //println!("ftf ({}-{})/({}-{})={}", points[2].x, points[1].x, points[2].y, points[1].y, invslope2);

        let mut curx1: f64 = points[2].x as f64;
        let mut curx2: f64 = points[2].x as f64;
        for scanline_y in (points[0].y..points[2].y+1).rev() {
            let p1 = Point::new(curx1 as i32, scanline_y);
            let p2 = Point::new(curx2 as i32, scanline_y);
            //println!("ftf {:?} -> {:?}", p1, p2);
            canvas.draw_line(p1, p2).unwrap();
            curx1 -= invslope1;
            curx2 -= invslope2;
        }
    }

    pub fn fill_triangle(points: &[Point; 3], canvas: &mut Canvas<Window>) {
        // http://www.sunshine2k.de/coding/java/TriangleRasterization/TriangleRasterization.html
        // sort the points for filling
        //println!("- {:?}", points);
        let mut points_sorted = points.clone();
        // pdqsort sorts in decending order
        pdqsort::sort_by(&mut points_sorted, |a, b| b.y.cmp(&a.y));
        // .. we need them in ascending order
        points_sorted.reverse();
        //println!("+ {:?}", points_sorted);

        if points_sorted[1].y == points_sorted[2].y {
            // bottom-flat triangle
            fill_bottom_flat_triangle(&points_sorted, canvas);
        } else if points_sorted[0].y == points_sorted[1].y {
            // top-flat triangle
            fill_top_flat_triangle(&points_sorted, canvas);
        } else {
            // general case, we need to split the triangle in half
            let half_point: Point = Point::new(
                points_sorted[0].x + (((points_sorted[1].y - points_sorted[0].y) as f64 /
                    (points_sorted[2].y - points_sorted[0].y) as f64 ) as f64 *
                    (points_sorted[2].x - points_sorted[0].x) as f64) as i32,
                points_sorted[1].y);
            //println!("h {:?}", half_point);
            fill_bottom_flat_triangle(&[points_sorted[0], points_sorted[1], half_point], canvas);
            fill_top_flat_triangle(&[points_sorted[1], half_point, points_sorted[2]], canvas);
        }
    }

    pub fn outline_triangle(points: &[Point; 3], canvas: &mut Canvas<Window>) {
        // draw triangle (as lines) to backbuffer
        for i in 0..3 {
            // end of this line is the beginning of the next..
            let mut j = i + 1;
            // .. unless the next line is actually the first one
            j = if j == 3 { 0 } else { j };
            // draw the line
            //println!("{:?}, {:?}", points[i], points[j]);
            canvas.draw_line(points[i], points[j]).unwrap();
        }
    }
}

/*#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}*/
