pub mod simple;
pub mod veggies;

// Test suite
#[cfg(test)]
mod test_schema_simple {

    use crate::simple::*;

    #[test]
    fn test_simple_struct() {
        let simple_struct_a: Simple = Simple {
            completed: true,
            id: 17.0,
            title: "Test".to_string(),
            user_id: 1997.0,
        };

        let simple_struct_b: Simple = simple_struct_a.clone();

        assert_eq!(simple_struct_a, simple_struct_b);

        let simple_struct_c: Simple = Simple {
            completed: true,
            id: 18.0,
            title: "Test".to_string(),
            user_id: 1997.0,
        };

        assert_ne!(simple_struct_a, simple_struct_c);
    }

    #[test]
    fn test_simple_builder() {
        let simple_build_a: Simple = Simple::builder()
            .completed(true)
            .id(17.0)
            .title("Test".to_string())
            .user_id(1997.0)
            .try_into()
            .unwrap();

        let simple_build_b: Simple = Simple::from(&simple_build_a);

        assert_eq!(simple_build_a, simple_build_b);

        let simple_build_c: Simple = Simple::builder()
            .completed(true)
            .id(18.0)
            .title("Test".to_string())
            .user_id(1997.0)
            .try_into()
            .unwrap();

        assert_ne!(simple_build_a, simple_build_c);
    }
}

#[cfg(test)]
mod test_schema_veggies {
    use crate::veggies::*;
    use serde_json::{Map, Value};

    #[test]
    fn test_struct_veggie() {
        let veg_struct_a: Veggie = Veggie {
            veggie_like: true,
            veggie_name: "carrots".to_string(),
        };

        let veg_struct_b: Veggie = veg_struct_a.clone();

        assert_eq!(veg_struct_a, veg_struct_b);

        let veg_struct_c: Veggie = Veggie {
            veggie_like: true,
            veggie_name: "onion".to_string(),
        };

        assert_ne!(veg_struct_a, veg_struct_c);

        let veg_struct_d: Veggie = Veggie::from(&veg_struct_b);

        assert_eq!(veg_struct_d, veg_struct_b);
    }

    #[test]
    fn test_struct_fruit() {
        let mut list: Map<String, Value> = Map::new();
        list.insert("Lorem".to_string(), "ipsum".into());
        let fruit_struct_a: Fruit = Fruit::from(list);

        // println!("{:?}", fruit_struct_a);

        let fruit_struct_b: Fruit = Fruit::from(&fruit_struct_a);

        assert_eq!(fruit_struct_a, fruit_struct_b);
    }
}
