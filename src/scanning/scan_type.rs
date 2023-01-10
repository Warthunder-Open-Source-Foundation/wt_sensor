use std::str::FromStr;

#[derive(Debug)]
pub enum Pattern {
	Pyramide,
	Cone,
	No, // IDK what this does or means, is related to Track
}

impl FromStr for Pattern {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"pyramide" => Ok(Self::Pyramide),
			"cone" => Ok(Self::Cone),
			"no" => Ok(Self::No),
			_ => Err(())
		}
	}
}