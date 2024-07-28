use crate::{data::Data, types::ListArgs};

pub fn action(args:ListArgs) {
    println!("{:#?}", Data::default().all());
}
