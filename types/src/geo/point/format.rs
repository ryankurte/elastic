use serde::{ Serializer, Deserializer };
use georust::Point;

/// A format used for parsing and formatting geo points.
pub trait GeoPointFormat
where Self : Default + Copy {
	/// Parses a `geo::Point`.
	///
	/// This requires access to the full `serde` deserializer because geo points can be serialised as
	/// different kinds of complex objects.
	fn parse<D>(deserializer: &mut D) -> Result<Point, D::Error> where
	D: Deserializer;

	/// Formats a `geo::Point`.
	///
	/// This requires access to the full `serde` serializer because geo points can be serialised as
	/// different kinds of complex objects.
	///
	/// Formatting also has access to the mapping type, which could be needed to build the structure
	/// properly.
	fn format<S, M>(point: &Point, serializer: &mut S) -> Result<(), S::Error> where
	M: ::geo::point::mapping::ElasticGeoPointMapping<Self>,
	S: Serializer;
}
