use sample_contract::NameProvider;

pub struct Sample {}

impl NameProvider for Sample {
    fn name(&self) -> &'static str {
        "sample"
    }
}
