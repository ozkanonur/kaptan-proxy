use payload::HttpQualifications;

pub mod payload;

pub trait ParseRawTcpBuffer {
    /// Reads qualifications of Http(method, path, version, host)
    /// and returns HttpQualifications as instace.
    ///
    /// # Panics
    /// No panic scenario implemented yet. However, logs
    ///
    /// # Usage
    /// ```
    /// let http_qualifications = HttpQualifications::parse(buffer);
    /// ```
    fn parse(buffer: &String) -> HttpQualifications;
}
