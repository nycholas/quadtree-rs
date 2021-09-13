mod quadtree_tests {
    use quadtree::*;

    #[derive(Debug)]
    struct Entity {
        id: i64,
        nickname: String,
    }

    impl Entity {
        fn new(id: i64, nickname: String) -> Self {
            Self { id, nickname }
        }
    }

    #[test]
    fn test_it_works() {
        let entity1 = Entity::new(1, String::from("nickname1"));
        let entity2 = Entity::new(2, String::from("nickname2"));
        let entity3 = Entity::new(3, String::from("nickname3"));

        let mut qt = Quadtree::new(Rectangle::new(0.0, 0.0, 100.0, 100.0));
        qt.put(Item::new(Point::new(10.0, 10.0), &entity1));
        qt.put(Item::new(Point::new(15.0, 15.0), &entity2));
        qt.put(Item::new(Point::new(20.0, 20.0), &entity3));

        let items = qt.query(Rectangle::new(5.0, 5.0, 10.0, 10.0));
        assert!(items.len() == 2);
        assert_eq!(items[0].data().id, 1);
        assert_eq!(items[0].data().nickname, String::from("nickname1"));
        assert_eq!(items[1].data().id, 2);
        assert_eq!(items[1].data().nickname, String::from("nickname2"));
    }

    #[test]
    fn test_query() {
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

        let items1 = qt.query(Rectangle::new(0.0, 0.0, 200.0, 200.0));
        let points1: Vec<Point> = items1.iter().map(|&it| { it.position() }).collect();
        assert_eq!(items1.len(), 4);
        assert!(points1.contains(&Point::new(10.0, 10.0)));
        assert!(points1.contains(&Point::new(110.0, 10.0)));
        assert!(points1.contains(&Point::new(110.0, 110.0)));
        assert!(points1.contains(&Point::new(10.0, 110.0)));

        let items2 = qt.query(Rectangle::new(10.0, 0.0, 20.0, 20.0));
        let points2: Vec<Point> = items2.iter().map(|&it| { it.position() }).collect();
        assert_eq!(items2.len(), 1);
        assert!(points2.contains(&Point::new(10.0, 10.0)));

        let items3 = qt.query(Rectangle::new(0.0, 0.0, 200.0, 100.0));
        let points3: Vec<Point> = items3.iter().map(|&it| { it.position() }).collect();
        assert_eq!(items3.len(), 2);
        assert!(points3.contains(&Point::new(10.0, 10.0)));
        assert!(points3.contains(&Point::new(110.0, 10.0)));

        let items4 = qt.query(Rectangle::new(0.0, 100.0, 200.0, 100.0));
        let points4: Vec<Point> = items4.iter().map(|&it| { it.position() }).collect();
        assert_eq!(items4.len(), 2);
        assert!(points4.contains(&Point::new(110.0, 110.0)));
        assert!(points4.contains(&Point::new(10.0, 110.0)));

        let items5 = qt.query(Rectangle::new(200.0, 200.0, 200.0, 200.0));
        assert_eq!(items5.len(), 0);

        let items6 = qt.query(Rectangle::new(0.0, 0.0, 0.0, 0.0));
        assert_eq!(items6.len(), 0);

        let items7 = qt.query(Rectangle::new(100.0, 100.0, 100.0, 100.0));
        let points7: Vec<Point> = items7.iter().map(|&it| { it.position() }).collect();
        assert_eq!(items7.len(), 1);
        assert!(points7.contains(&Point::new(110.0, 110.0)));
    }
}
