#[derive(Clone)]
#[cfg(feature = "stream")]
pub struct LJMStream {
    // Stores the scan rate
    pub(crate) scans_per_read: i32,

    // Stores a list of the internal LJM addresses
    pub(crate) scan_list: Vec<i32>,
}
