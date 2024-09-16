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

// #[cfg(test)]
// mod test_schema_misc {
//     use std::io;

//     fn parse_data(input: i32) -> Result<i32, io::Error> {
//         match input {
//             0 => Ok(0),
//             x => Err(io::Error::new(
//                 io::ErrorKind::InvalidData,
//                 format!("unexpected number {}", x),
//             )),
//         }
//     }

//     #[test]
//     fn test_parsing_wrong_data() {
//         let result = parse_data(1).map_err(|e| e.kind());
//         let expected = Err(io::ErrorKind::InvalidData);
//         assert_eq!(expected, result);
//     }
// }

#[cfg(test)]
mod test_schema_veggies {
    use crate::veggies::*;

    #[test]
    fn test_struct_veggies() {
        let veg_struct_a: Veggie = Veggie {
            veggie_like: true,
            veggie_name: "carrots".to_string(),
        };

        let veg_struct_b: Veggie = veg_struct_a.clone();

        assert_eq!(veg_struct_a, veg_struct_b);

        let veggies_struct_a: Veggies = Veggies {
            fruits: vec![String::from("apple"), String::from("mango")],
            vegetables: vec![veg_struct_a, veg_struct_b],
        };

        let veggies_struct_b: Veggies = veggies_struct_a.clone();

        assert_eq!(veggies_struct_a, veggies_struct_b);
    }
}
