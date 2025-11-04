
use bitflags::bitflags;

bitflags! {
    /// Represents one or more modalities in a multimodal system.
    ///
    /// Each variant is a power of two, allowing bitwise composition.
    ///
    /// # Example
    /// ```
    /// use modalities::Modality;
    ///
    /// let combo = Modality::Audio | Modality::Text;
    /// assert!(combo.contains(Modality::Audio));
    /// assert!(combo.contains(Modality::Text));
    /// assert!(!combo.contains(Modality::Image));
    /// ```
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Modality: u32 {
        const NONE  = 0;
        const AUDIO = 1 << 0;
        const IMAGE = 1 << 1;
        const TEXT  = 1 << 2;
        const VIDEO = 1 << 3;
        const OTHER = 1 << 4;
        const ALL   = Self::AUDIO.bits()
                     | Self::IMAGE.bits()
                     | Self::TEXT.bits()
                     | Self::VIDEO.bits()
                     | Self::OTHER.bits();
    }
}

impl Modality {
    pub fn to_names(self) -> Vec<&'static str> {
        let mut names = Vec::new();
        if self.contains(Modality::AUDIO) {
            names.push("audio");
        }
        if self.contains(Modality::IMAGE) {
            names.push("image");
        }
        if self.contains(Modality::TEXT) {
            names.push("text");
        }
        if self.contains(Modality::VIDEO) {
            names.push("video");
        }
        if self.contains(Modality::OTHER) {
            names.push("other");
        }
        names
    }

    pub fn from_names(names: &[&str]) -> Result<Self, String> {
        let mut bits = Modality::NONE;
        for name in names {
            bits |= match *name {
                "audio" => Modality::AUDIO,
                "image" => Modality::IMAGE,
                "text" => Modality::TEXT,
                "video" => Modality::VIDEO,
                "other" => Modality::OTHER,
                _ => return Err(format!("Invalid modality name: {}", name)),
            };
        }
        Ok(bits)
    }
}

#[cfg(feature = "python")]
pub mod python_modality {
    use super::Modality;
    use pyo3::prelude::*;

    #[pyclass(module = "modalities")]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct PyModality {
        bits: u32,
    }

    #[pymethods]
    impl PyModality {
        #[new]
        fn new() -> Self {
            PyModality { bits: 0 }
        }

        #[classattr]
        const NONE: Self = Self {
            bits: Modality::NONE.bits(),
        };
        #[classattr]
        const AUDIO: Self = Self {
            bits: Modality::AUDIO.bits(),
        };
        #[classattr]
        const IMAGE: Self = Self {
            bits: Modality::IMAGE.bits(),
        };
        #[classattr]
        const TEXT: Self = Self {
            bits: Modality::TEXT.bits(),
        };
        #[classattr]
        const VIDEO: Self = Self {
            bits: Modality::VIDEO.bits(),
        };
        #[classattr]
        const OTHER: Self = Self {
            bits: Modality::OTHER.bits(),
        };
        #[classattr]
        const ALL: Self = Self {
            bits: Modality::ALL.bits(),
        };

        fn __or__(&self, rhs: &Self) -> Self {
            Self {
                bits: self.bits | rhs.bits,
            }
        }

        fn __and__(&self, rhs: &Self) -> Self {
            Self {
                bits: self.bits & rhs.bits,
            }
        }

        fn __contains__(&self, other: &Self) -> bool {
            (self.bits & other.bits) == other.bits
        }

        fn names(&self) -> Vec<String> {
            let m = Modality::from_bits_truncate(self.bits);
            m.to_names().into_iter().map(|s| s.to_string()).collect()
        }

        fn __str__(&self) -> String {
            let names = self.names().join(" | ");
            if names.is_empty() {
                "none".into()
            } else {
                names
            }
        }
    }
}

#[cfg(feature = "python")]
pub use python_modality::PyModality;
#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
#[pymodule]
fn modalities(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyModality>()?;
    Ok(())
}

#[cfg(test)]
mod modality_tests {
    use super::*;
    #[test]
    fn test_single_flags_bits() {
        assert_eq!(Modality::AUDIO.bits(), 1);
        assert_eq!(Modality::IMAGE.bits(), 1 << 1);
        assert_eq!(Modality::TEXT.bits(), 1 << 2);
        assert_eq!(Modality::VIDEO.bits(), 1 << 3);
        assert_eq!(Modality::OTHER.bits(), 1 << 4);
        assert_eq!(Modality::NONE.bits(), 0);
    }

    #[test]
    fn test_basic_combinations() {
        let combo = Modality::AUDIO | Modality::TEXT;

        assert!(combo.contains(Modality::AUDIO));
        assert!(combo.contains(Modality::TEXT));
        assert!(!combo.contains(Modality::IMAGE));
        assert_eq!(combo.bits(), Modality::AUDIO.bits() | Modality::TEXT.bits());
    }

    #[test]
    fn test_bitwise_operations_are_inverse() {
        let a = Modality::AUDIO;
        let b = Modality::TEXT;
        let c = a | b;

        // ensure intersection works
        assert_eq!(c & a, a);
        assert_eq!(c & b, b);
        assert_eq!(a & b, Modality::NONE);
    }

    #[test]
    fn test_to_names_single_and_multi() {
        let single = Modality::IMAGE;
        assert_eq!(single.to_names(), vec!["image"]);

        let multi = Modality::AUDIO | Modality::VIDEO;
        let names = multi.to_names();
        assert!(names.contains(&"audio"));
        assert!(names.contains(&"video"));
        assert_eq!(names.len(), 2);
    }

    #[test]
    fn test_from_names_valid() {
        let combo = Modality::from_names(&["audio", "text"]).unwrap();
        assert!(combo.contains(Modality::AUDIO));
        assert!(combo.contains(Modality::TEXT));
        assert!(!combo.contains(Modality::IMAGE));

        let all = Modality::from_names(&["audio", "image", "text", "video", "other"]).unwrap();
        assert_eq!(all, Modality::ALL);
    }

    #[test]
    fn test_from_names_invalid() {
        let err = Modality::from_names(&["nonsense"]);
        assert!(err.is_err());
        assert!(err.unwrap_err().contains("Invalid modality name"));
    }

    #[test]
    fn test_none_and_all_constants() {
        assert_eq!(Modality::NONE.bits(), 0);
        assert_eq!(
            Modality::ALL.bits(),
            Modality::AUDIO.bits()
                | Modality::IMAGE.bits()
                | Modality::TEXT.bits()
                | Modality::VIDEO.bits()
                | Modality::OTHER.bits()
        );
    }

    #[test]
    fn test_display_and_debug_consistency() {
        // Ensures debug printing works and names aren't empty for valid combos
        let m = Modality::AUDIO | Modality::TEXT;
        let names = m.to_names();
        assert!(!format!("{:?}", m).is_empty());
        assert_eq!(names.len(), 2);
    }
}
