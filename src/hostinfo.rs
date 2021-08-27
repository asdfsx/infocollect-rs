#[derive(Debug)]
pub struct HostInfo {
	pub Hostname:      String,
	pub CPUCores:      f64,
	pub MEMSize:       f64,
	pub GPUCardNum:    f64,
	pub HardwareAddrs: Vec<String>,
}
