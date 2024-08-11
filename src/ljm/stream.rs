#[derive(Clone)]
pub(crate) struct LJMStream {
    // Stores the scan rate
    pub(crate) scan_rate: f64,

    // Stores a list of the internal LJM addresses
    pub(crate) scan_list: Vec<i32>,
}