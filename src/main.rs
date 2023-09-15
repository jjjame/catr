fn main() {
    if let Err(_e) = catr::get_args().and_then(catr::run) {}
}
