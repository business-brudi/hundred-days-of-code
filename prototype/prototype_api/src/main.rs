use sample_contract::NameProvider;
use registration::Registration;

fn main() {
    let name_provider: Box<dyn NameProvider> = Box::new(registration::sample::NameProvider {});
    let name = name_provider.print_name();
    println!("{}", name);
}