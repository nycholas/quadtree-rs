#![crate_type = "lib"]
#![crate_name = "quadtree"]

use std::fmt;
use std::ops::Deref;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl PartialEq<Point> for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}

pub trait Position {
    fn position(&self) -> Point;
}

#[derive(Debug)]
pub struct Item<'a, T> {
    point: Point,
    data: &'a T,
}

impl<'a, T> Item<'a, T>
where
    T: 'a,
{
    pub fn new(point: Point, data: &'a T) -> Self {
        Self { point, data }
    }

    pub fn data(&self) -> &'a T {
        self.data
    }
}

impl<'a, T> Deref for Item<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.data
    }
}

impl<'a, T> Position for Item<'a, T> {
    fn position(&self) -> Point {
        self.point
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

impl Rectangle {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}, {}, {}, {})",
            self.x,
            self.y,
            self.x + self.width,
            self.y + self.height
        )
    }
}

pub struct Options {
    pub max_items: usize,
    pub max_depth: u8,
    pub depth: u8,
}

impl Default for Options {
    fn default() -> Options {
        Options {
            max_items: 20,
            max_depth: 3,
            depth: 0,
        }
    }
}

pub struct Quadtree<T> {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    items: Vec<T>,
    children: Option<[Box<Quadtree<T>>; 4]>,
    options: Options,
}

impl<T: Position> Quadtree<T> {
    pub fn new(boundary: Rectangle) -> Self {
        Self::with_options(
            boundary,
            Options {
                ..Default::default()
            },
        )
    }

    pub fn with_options(boundary: Rectangle, options: Options) -> Self {
        Self {
            x: boundary.x,
            y: boundary.y,
            width: boundary.width,
            height: boundary.height,
            items: Vec::new(),
            children: None,
            options,
        }
    }

    pub fn put(&mut self, item: T) {
        if !self.contains(&item) {
            return;
        }

        if self.children.is_none()
            && self.items.len() < self.options.max_items
            && self.options.depth < self.options.max_depth
        {
            self.items.push(item);
            return;
        }

        match self.children {
            Some(ref mut children) => {
                for child in children {
                    if child.contains(&item) {
                        child.items.push(item);
                        break;
                    }
                }
            }
            None => {
                self.items.push(item);
                let mut children = self.subdivide();
                while let Some(it) = self.items.pop() {
                    for child in children.iter_mut() {
                        if child.contains(&it) {
                            child.items.push(it);
                            break;
                        }
                    }
                }
                self.children = Some(children)
            }
        }
    }

    pub fn query(&self, range: Rectangle) -> Vec<&T> {
        match self.children {
            Some(ref children) => {
                let mut items = Vec::<&T>::new();
                if self.intersects(&range, &self.bounds()) {
                    for child in children {
                        items.extend(child.query(range));
                    }
                }
                items
            }
            None => {
                let mut items = Vec::<&T>::new();
                for item in &self.items {
                    if self._contains(&item.position(), &range) {
                        items.push(item);
                    }
                }
                items
            }
        }
    }

    fn contains(&self, item: &T) -> bool {
        self._contains(&item.position(), &self.bounds())
    }

    fn _contains(&self, point: &Point, boundary: &Rectangle) -> bool {
        point.x >= boundary.x
            && point.x <= boundary.x + boundary.width
            && point.y >= boundary.y
            && point.y <= boundary.y + boundary.height
    }

    fn intersects(&self, rectangle: &Rectangle, boundary: &Rectangle) -> bool {
        rectangle.x < boundary.x + boundary.width
            && rectangle.x + rectangle.width > boundary.x
            && rectangle.y < boundary.y + boundary.height
            && rectangle.y + rectangle.height > boundary.y
    }

    fn subdivide(&self) -> [Box<Quadtree<T>>; 4] {
        let w = self.width / 2.0;
        let h = self.height / 2.0;
        [
            Box::new(Quadtree::with_options(
                Rectangle::new(self.x, self.y, w, h),
                Options {
                    max_items: self.options.max_items,
                    max_depth: self.options.max_depth,
                    depth: self.options.depth + 1,
                },
            )),
            Box::new(Quadtree::with_options(
                Rectangle::new(self.x + w, self.y, w, h),
                Options {
                    max_items: self.options.max_items,
                    max_depth: self.options.max_depth,
                    depth: self.options.depth + 1,
                },
            )),
            Box::new(Quadtree::with_options(
                Rectangle::new(self.x + w, self.y + h, w, h),
                Options {
                    max_items: self.options.max_items,
                    max_depth: self.options.max_depth,
                    depth: self.options.depth + 1,
                },
            )),
            Box::new(Quadtree::with_options(
                Rectangle::new(self.x, self.y + h, w, h),
                Options {
                    max_items: self.options.max_items,
                    max_depth: self.options.max_depth,
                    depth: self.options.depth + 1,
                },
            )),
        ]
    }

    fn bounds(&self) -> Rectangle {
        Rectangle::new(self.x, self.y, self.width, self.height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_points() {
        let p1 = Point::new(10.0, 5.0);
        assert_eq!(p1.x, 10.0);
        assert_eq!(p1.y, 5.0);
        assert_eq!(p1, Point::new(10.0, 5.0));
        assert_eq!(format!("Point: {}", p1), "Point: (10, 5)");
    }

    #[test]
    fn test_items() {
        let data1 = String::from("data1");
        let item1 = Item::new(Point::new(10.0, 5.0), &data1);
        assert_eq!(item1.point, Point::new(10.0, 5.0));
        assert_eq!(item1.position(), Point::new(10.0, 5.0));
        assert_eq!(item1.deref(), &data1);
    }

    #[test]
    fn test_rectangles() {
        let rec1 = Rectangle::new(0.0, 1.0, 10.0, 5.0);
        assert_eq!(rec1.x, 0.0);
        assert_eq!(rec1.y, 1.0);
        assert_eq!(rec1.width, 10.0);
        assert_eq!(rec1.height, 5.0);
        assert_eq!(format!("Rectangle: {}", rec1), "Rectangle: (0, 1, 10, 6)");
    }

    #[test]
    fn test_contains() {
        let q1 = Quadtree::<Item<String>>::new(Rectangle::new(0.0, 0.0, 100.0, 100.0));
        let data1 = String::from("item1");
        let item1 = Item::new(Point::new(10.0, 10.0), &data1);
        assert!(q1.contains(&item1));

        let data2 = String::from("item2");
        let item2 = Item::new(Point::new(-10.0, -10.0), &data2);
        assert!(!q1.contains(&item2));
    }

    #[test]
    fn test_contains_() {
        let q1 = Quadtree::<Item<String>>::new(Rectangle::new(0.0, 0.0, 100.0, 100.0));
        assert!(q1._contains(
            &Point::new(10.0, 10.0),
            &Rectangle::new(0.0, 0.0, 100.0, 100.0)
        ));
        assert!(q1._contains(
            &Point::new(100.0, 100.0),
            &Rectangle::new(0.0, 0.0, 100.0, 100.0)
        ));
        assert!(!q1._contains(
            &Point::new(101.0, 100.0),
            &Rectangle::new(0.0, 0.0, 100.0, 100.0)
        ));
        assert!(!q1._contains(
            &Point::new(100.0, 101.0),
            &Rectangle::new(0.0, 0.0, 100.0, 100.0)
        ));
        assert!(!q1._contains(
            &Point::new(101.0, 101.0),
            &Rectangle::new(0.0, 0.0, 100.0, 100.0)
        ));
        assert!(!q1._contains(
            &Point::new(10.0, 10.0),
            &Rectangle::new(100.0, 100.0, 100.0, 100.0)
        ));
        assert!(q1._contains(
            &Point::new(110.0, 10.0),
            &Rectangle::new(100.0, 0.0, 100.0, 100.0)
        ));
        assert!(q1._contains(
            &Point::new(110.0, 110.0),
            &Rectangle::new(100.0, 100.0, 100.0, 100.0)
        ));
        assert!(!q1._contains(
            &Point::new(110.0, 110.0),
            &Rectangle::new(0.0, 0.0, 100.0, 100.0)
        ));
        assert!(q1._contains(
            &Point::new(10.0, 110.0),
            &Rectangle::new(0.0, 100.0, 100.0, 100.0)
        ));
        assert!(q1._contains(
            &Point::new(100.0, 100.0),
            &Rectangle::new(100.0 - 3.0, 100.0 - 3.0, 2.0 * 3.0, 2.0 * 3.0)
        ));
        assert!(!q1._contains(
            &Point::new(150.0, 150.0),
            &Rectangle::new(100.0 - 3.0, 100.0 - 3.0, 2.0 * 3.0, 2.0 * 3.0)
        ));
        assert!(!q1._contains(
            &Point::new(152.0, 152.0),
            &Rectangle::new(100.0 - 3.0, 100.0 - 3.0, 2.0 * 3.0, 2.0 * 3.0)
        ));
    }

    #[test]
    fn test_intersects() {
        let q1 = Quadtree::<Item<String>>::new(Rectangle::new(0.0, 0.0, 100.0, 100.0));
        assert!(q1.intersects(
            &Rectangle::new(5.0, 5.0, 50.0, 50.0),
            &Rectangle::new(20.0, 10.0, 10.0, 10.0)
        ));
        assert!(q1.intersects(
            &Rectangle::new(5.0, 5.0, 50.0, 50.0),
            &Rectangle::new(5.0, 5.0, 50.0, 50.0)
        ));
        assert!(!q1.intersects(
            &Rectangle::new(5.0, 5.0, 50.0, 50.0),
            &Rectangle::new(55.0, 55.0, 50.0, 50.0)
        ));
    }

    #[test]
    fn test_subdivide() {
        let q1 = Quadtree::<Item<String>>::new(Rectangle::new(0.0, 0.0, 100.0, 100.0));
        let children = q1.subdivide();
        assert_eq!(children.len(), 4);

        let north_east = children[0].deref();
        let north_east_pos = (
            north_east.x,
            north_east.y,
            north_east.width,
            north_east.height,
        );
        assert_eq!(north_east.options.max_items, 20);
        assert_eq!(north_east.options.max_depth, 3);
        assert_eq!(north_east.options.depth, 1);
        assert_eq!(north_east_pos, (0.0, 0.0, 50.0, 50.0));

        let north_west = children[1].deref();
        let north_west_pos = (
            north_west.x,
            north_west.y,
            north_west.width,
            north_west.height,
        );
        assert_eq!(north_west.options.max_items, 20);
        assert_eq!(north_west.options.max_depth, 3);
        assert_eq!(north_west.options.depth, 1);
        assert_eq!(north_west_pos, (50.0, 0.0, 50.0, 50.0));

        let south_west = children[2].deref();
        let south_west_pos = (
            south_west.x,
            south_west.y,
            south_west.width,
            south_west.height,
        );
        assert_eq!(south_west.options.max_items, 20);
        assert_eq!(south_west.options.max_depth, 3);
        assert_eq!(south_west.options.depth, 1);
        assert_eq!(south_west_pos, (50.0, 50.0, 50.0, 50.0));

        let south_east = children[3].deref();
        let south_east_pos = (
            south_east.x,
            south_east.y,
            south_east.width,
            south_east.height,
        );
        assert_eq!(south_east.options.max_items, 20);
        assert_eq!(south_east.options.max_depth, 3);
        assert_eq!(south_east.options.depth, 1);
        assert_eq!(south_east_pos, (0.0, 50.0, 50.0, 50.0));
    }

    #[test]
    fn test_put_and_subdivide() {
        let entity = ();

        let mut qt = Quadtree::with_options(
            Rectangle::new(0.0, 0.0, 200.0, 200.0),
            Options {
                max_items: 1,
                ..Default::default()
            },
        );
        qt.put(Item::new(Point::new(10.0, 10.0), &entity));
        qt.put(Item::new(Point::new(110.0, 10.0), &entity));
        qt.put(Item::new(Point::new(110.0, 110.0), &entity));
        qt.put(Item::new(Point::new(10.0, 110.0), &entity));
        match qt.children {
            Some([ref c1, ref c2, ref c3, ref c4]) => {
                assert_eq!(c1.x, 0.0);
                assert_eq!(c1.y, 0.0);
                assert_eq!(c1.width, 100.0);
                assert_eq!(c1.height, 100.0);
                assert_eq!(c1.options.max_items, 1);
                assert_eq!(c1.options.max_depth, 3);
                assert_eq!(c1.options.depth, 1);
                assert_eq!(c1.items.len(), 1);
                assert_eq!(c1.items[0].point, Point::new(10.0, 10.0));
                assert!(c1.children.is_none());

                assert_eq!(c2.x, 100.0);
                assert_eq!(c2.y, 0.0);
                assert_eq!(c2.width, 100.0);
                assert_eq!(c2.height, 100.0);
                assert_eq!(c2.options.max_items, 1);
                assert_eq!(c2.options.max_depth, 3);
                assert_eq!(c2.options.depth, 1);
                assert_eq!(c2.items.len(), 1);
                assert_eq!(c2.items[0].point, Point::new(110.0, 10.0));
                assert!(c2.children.is_none());

                assert_eq!(c3.x, 100.0);
                assert_eq!(c3.y, 100.0);
                assert_eq!(c3.width, 100.0);
                assert_eq!(c3.height, 100.0);
                assert_eq!(c3.options.max_items, 1);
                assert_eq!(c3.options.max_depth, 3);
                assert_eq!(c3.options.depth, 1);
                assert_eq!(c3.items.len(), 1);
                assert_eq!(c3.items[0].point, Point::new(110.0, 110.0));
                assert!(c3.children.is_none());

                assert_eq!(c4.x, 0.0);
                assert_eq!(c4.y, 100.0);
                assert_eq!(c4.width, 100.0);
                assert_eq!(c4.height, 100.0);
                assert_eq!(c4.options.max_items, 1);
                assert_eq!(c4.options.max_depth, 3);
                assert_eq!(c4.options.depth, 1);
                assert_eq!(c4.items.len(), 1);
                assert_eq!(c4.items[0].point, Point::new(10.0, 110.0));
                assert!(c4.children.is_none());
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_bounds() {
        let q1 = Quadtree::<Item<String>>::new(Rectangle::new(0.0, 0.0, 100.0, 100.0));
        let bounds = q1.bounds();
        assert_eq!(
            (bounds.x, bounds.y, bounds.width, bounds.height),
            (0.0, 0.0, 100.0, 100.0)
        );
    }
}
