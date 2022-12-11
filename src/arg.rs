

pub struct Arg{
    pub field: String,
    pub require: bool,
    pub has_value: bool
}

impl Arg {
    pub fn new(
        field: String,
        require: bool,
        has_value: bool
    ) -> Self { Self { field, require, has_value } }
}