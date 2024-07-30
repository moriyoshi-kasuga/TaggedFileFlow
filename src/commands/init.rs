use anyhow::Context;

use crate::asset::Asset;

use super::{Init, Run};

impl Run for Init {
    fn run(&self) -> anyhow::Result<()> {
        println!("{}", String::from_utf8_lossy(&Asset::get("alias.sh").with_context(|| "not found alias.sh")?.data));
        Ok(())
    }
}
