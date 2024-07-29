use crate::{data::Data, types::ListArgs};

pub fn action(args: ListArgs) {
    match args.name {
        Some(name) => doc(name),
        None => all(),
    }
}

pub fn doc(name: String) {
    let data = Data::default();
    let doc = match data.get(&name) {
        Some(doc) => doc,
        None => panic!("not found document: {}", name),
    };
    println!("{:#?}", doc);
}

pub fn all() {
    println!("{:#?}", Data::default().all());
}
