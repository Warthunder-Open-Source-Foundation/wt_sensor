use std::str::FromStr;
#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;


#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect, bevy_reflect::FromReflect))]
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