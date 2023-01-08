use crate::scanning::scan_type::ScanType;
use crate::util::Limits;

pub enum ScanPattern {
	// Search
	SearchNarrow,
	SearchMedium,
	SearchWide,

	// I assume these are ACM
	HudLock,
	VerticalLock,
	BoresightLock,

	// Listed under scan patterns?
	Track,
}

pub struct Submode {
	scan_type: ScanType,
	limits: Limits,
	roll_stab_limit: Option<f64>,
	pitch_stab_limit: Option<f64>,
	period: f64,
	width: f64,
	bar_height: Option<f64>,
	bars_count: Option<u8>,
	row_major: Option<bool>,
	center_elevation: Option<f64>,
	indicate: Option<bool>,
}