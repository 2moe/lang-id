use core::fmt;
use tinystr::TinyAsciiStr;
pub mod description;
pub mod max;
pub mod min;

type PhfMap<'a> = phf::Map<&'a str, &'a str>;

/// Function to convert a string slice to a TinyAsciiStr with a given length N
pub const fn as_tiny<const N: usize>(s: &str) -> TinyAsciiStr<N> {
    match TinyAsciiStr::from_str(s) {
        Ok(x) => x,
        _ => panic!("Failed to convert as tinystr"),
    }
}

/// Struct representing a language identification code with tiny string components
#[derive(Debug)]
pub struct TinyID {
    pub language: TinyAsciiStr<4>, 
    pub script: TinyAsciiStr<4>,
    pub region: TinyAsciiStr<4>, 
}

/// Implementing the Display trait to enable printing of a TinyID object
impl fmt::Display for TinyID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}-{}", self.language, self.script, self.region)
    }
}

/// Implementation block for the TinyID struct
impl TinyID {

    /// Constructor function for the TinyID struct
    pub const fn new(language: &str, script: &str, region: &str) -> Self {
        Self {
            language: as_tiny::<4>(language), 
            script: as_tiny::<4>(script), 
            region: as_tiny::<4>(region), 
        }
    }
}
