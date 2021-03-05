pub fn extractfrom_slice(data_array: &[u8]) -> Version {
    Version {
        build: (data_array[0] as u16) | data_array[1] as u16,
        minor: data_array[2],
        major: data_array[3],
    }
}