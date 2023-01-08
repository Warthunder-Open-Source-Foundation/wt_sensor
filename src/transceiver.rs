use crate::util::Antenna;

pub enum Transceiver {
	Pulse(Pulse),
	PulseDoppler(Pulse),
}


// Name is WIP
pub struct Pulse {
	side_lobes_attenuation: f64,
	power: f64,
	band: u8,
	rcs: f64,
	range: f64,
	range_max: f64,
	time_gain_control: bool,
	antenna: Antenna,
}