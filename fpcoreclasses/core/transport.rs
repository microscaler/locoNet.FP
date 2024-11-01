use std::collections::HashMap;

/// Represents the physical transport protocol like serial COM, Bluetooth, HTTP, etc.
pub trait Transport {
    /// Returns the transport protocol name, e.g., "http", "com" (for COM port serial), "bt" (Bluetooth), etc.
    fn transport_name(&self) -> &str;

    /// Returns all (usually local) addresses, which can have connected fiscal printers.
    /// The returned pairs are in the form (address, description).
    fn get_available_addresses(&self) -> Vec<(String, String)> {
        Vec::new() // Default implementation returning an empty vector
    }

    /// Opens a channel to the specified address.
    ///
    /// # Arguments
    ///
    /// * `address` - The address to connect to.
    ///
    /// # Returns
    ///
    /// A newly created channel implementing the `Channel` trait.
    fn open_channel(&self, address: &str) -> Box<dyn Channel>;

    /// Drops a channel. If it is needed, it also closes it.
    ///
    /// # Arguments
    ///
    /// * `channel` - The channel to be dropped.
    fn drop(&self, channel: Box<dyn Channel>);
}
