pub trait Quirk: Send + Sync {
    fn get_description(&self) -> &str;
    fn get_name(&self) -> &str;
    // TODO
}
