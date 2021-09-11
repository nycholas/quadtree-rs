mod quadtree_tests {
    use quadtree::*;

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

        let item1 = Item::new(Point::new(10.0, 10.0), &entity1);
        let item2 = Item::new(Point::new(15.0, 15.0), &entity2);
        let item3 = Item::new(Point::new(20.0, 20.0), &entity3);

        let mut q1 = Quadtree::new(Rectangle::new(0.0, 0.0, 100.0, 100.0));
        q1.put(item1);
        q1.put(item2);
        q1.put(item3);

        let items = q1.query(Rectangle::new(5.0, 5.0, 10.0, 10.0));
        assert!(items.len() == 2);
    }
}
