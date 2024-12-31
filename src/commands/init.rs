use super::{Init, Run};

impl Run for Init {
    fn run(self) -> anyhow::Result<()> {
        println!(
            "{}",
            include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/alias.sh"))
        );
        Ok(())
    }
}
