use sample_contract::NameProvider;

pub struct Sample {}

impl NameProvider for Sample {
    fn get_name(&self) -> String {
        String::from("sample")
    }
}
