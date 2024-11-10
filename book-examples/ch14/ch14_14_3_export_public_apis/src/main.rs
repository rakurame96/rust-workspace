use ch14_14_3_export_public_apis::kinds::PrimaryColor;
use ch14_14_3_export_public_apis::utils::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}
