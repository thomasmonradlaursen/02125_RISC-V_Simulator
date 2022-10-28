#[derive(Debug)]
    pub struct Fields {
        pub test_int: i32,
    }

pub fn add_one(field: &mut Fields) {
    field.test_int += 1;
}

