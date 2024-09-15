pub mod simple;
pub mod veggies;

// Test suite
#[cfg(test)]
mod test_schema_simple {

    use crate::simple::*;

    #[test]
    fn test_struct_simple() {
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
    fn test_builder_simple() {
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

    #[test]
    fn test_struct_veggies() {
        let veg_build: Veggie = Veggie::builder()
            .veggie_name("carrots")
            .veggie_like(true)
            .try_into()
            .unwrap();

        let veg_struct: Veggie = Veggie {
            veggie_like: true,
            veggie_name: "carrots".to_string(),
        };

        let veggies: Veggies = Veggies {
            fruits: vec![String::from("apple"), String::from("mango")],
            vegetables: vec![veg_struct, veg_build],
        };

        println!("{:?}", veggies);
    }
}
