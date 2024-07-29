use crate::{data::Data, types::DelArgs};

pub fn action(args: DelArgs) {
    let mut data = Data::default();

    if !data.del(&args.name) {
        panic!("not found document: {}", args.name);
    }

    data.save();
}
