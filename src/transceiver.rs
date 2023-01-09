use std::str::FromStr;
use wt_blk::WTBlk;
use crate::util::Antenna;
use serde_json::Value;


// Name is WIP
#[derive(Debug)]
pub struct Transceiver {
	trans_type: TransceiverType,
	side_lobes_attenuation: f64,
	power: f64,
	band: u8,
	rcs: f64,
	range: f64,
	range_max: f64,
	time_gain_control: bool,
	antenna: Antenna,
}

impl Transceiver {
	pub fn from_value(value: &WTBlk, trans_type: &str) -> Option<Self> {
		let side_lobes_attenuation = value.float("/sideLobesAttenuation").ok()?;

		let power = value.float("/power").ok()?;

		let band = value.int("/band").ok()? as u8;

		let rcs = value.float("/rcs").ok()?;

		let range = value.float("/range").ok()?;

		let range_max = value.float("/rangeMax").ok()?;

		let time_gain_control = value.bool("/timeGainControl").ok()?;

		let angle_half_sens = value.float("/antenna/angleHalfSens").ok()?;

		let side_lobes_sensitivity = value.float("/antenna/sideLobesSensitivity").ok()?;

		Some(Self {
			trans_type: TransceiverType::from_str(trans_type).ok()?,
			side_lobes_attenuation,
			power,
			band,
			rcs,
			range,
			range_max,
			time_gain_control,
			antenna: Antenna {
				angle_half_sens,
				side_lobes_sensitivity,
			},
		})
	}
}

#[derive(Debug)]
pub enum TransceiverType {
	Pulse,
	PulseDoppler,
}

impl FromStr for TransceiverType {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"pulse" => Ok(TransceiverType::Pulse),
			"pulseDoppler" => Ok(TransceiverType::PulseDoppler),
			_ => {
				Err(())
			}
		}
	}
}