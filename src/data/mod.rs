use include_dir::{include_dir, Dir};
use std::{fmt::Debug, str::FromStr};

static DATA: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/data");

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum Proposal {
    Annotations,
    CustomPageSizes,
    ExceptionHandling,
    ExtendedConst,
    FunctionReferences,
    GC,
    Memory64,
    MultiMemory,
    RelaxedSimd,
    TailCall,
    Threads,
    WideArithmetic,
}

impl Proposal {
    pub fn all() -> &'static [Proposal] {
        &[
            Proposal::Annotations,
            Proposal::CustomPageSizes,
            Proposal::ExceptionHandling,
            Proposal::ExtendedConst,
            Proposal::FunctionReferences,
            Proposal::GC,
            Proposal::Memory64,
            Proposal::MultiMemory,
            Proposal::RelaxedSimd,
            Proposal::TailCall,
            Proposal::Threads,
            Proposal::WideArithmetic,
        ]
    }
}

impl From<Proposal> for &'static str {
    fn from(proposal: Proposal) -> &'static str {
        match proposal {
            Proposal::Annotations => "annotations",
            Proposal::CustomPageSizes => "custom-page-sizes",
            Proposal::ExceptionHandling => "exception-handling",
            Proposal::ExtendedConst => "extended-const",
            Proposal::FunctionReferences => "function-references",
            Proposal::GC => "gc",
            Proposal::Memory64 => "memory64",
            Proposal::MultiMemory => "multi-memory",
            Proposal::RelaxedSimd => "relaxed-simd",
            Proposal::TailCall => "tail-call",
            Proposal::Threads => "threads",
            Proposal::WideArithmetic => "wide-arithmetic",
        }
    }
}

impl FromStr for Proposal {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "annotations" => Proposal::Annotations,
            "custom-page-sizes" => Proposal::CustomPageSizes,
            "exception-handling" => Proposal::ExceptionHandling,
            "extended-const" => Proposal::ExtendedConst,
            "function-references" => Proposal::FunctionReferences,
            "gc" => Proposal::GC,
            "memory64" => Proposal::Memory64,
            "multi-memory" => Proposal::MultiMemory,
            "relaxed-simd" => Proposal::RelaxedSimd,
            "tail-call" => Proposal::TailCall,
            "threads" => Proposal::Threads,
            "wide-arithmetic" => Proposal::WideArithmetic,
            _ => return Err(()),
        })
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum SpecVersion {
    V1,
    V2,
    V3,
    Latest,
}

impl SpecVersion {
    fn name(self) -> &'static str {
        match self {
            Self::V1 => "wasm-v1",
            Self::V2 => "wasm-v2",
            Self::V3 => "wasm-v3",
            Self::Latest => "wasm-latest",
        }
    }

    pub fn all() -> &'static [SpecVersion] {
        &[
            SpecVersion::V1,
            SpecVersion::V2,
            SpecVersion::V3,
            SpecVersion::Latest,
        ]
    }
}

/// Get all test files associated with a proposal
pub fn proposal(name: &Proposal) -> impl Iterator<Item = TestFile<'static>> {
    let name: &'static str = (*name).into();
    let tests = DATA
        .get_dir(format!("proposals/{name}"))
        .expect("spec dir should always exist");

    tests.files().map(|file| TestFile {
        parent: name.to_string(),
        name: file
            .path()
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string(),
        contents: file.contents_utf8().expect("file should be utf8"),
    })
}

/// Get all test files associated with a spec version
pub fn spec(version: &SpecVersion) -> impl Iterator<Item = TestFile<'static>> {
    let name = version.name();
    let tests = DATA.get_dir(name).expect("spec dir should always exist");

    tests.files().map(|file| TestFile {
        parent: name.to_string(),
        name: file
            .path()
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string(),
        contents: file.contents_utf8().expect("file should be utf8"),
    })
}

/// A test file
#[derive(Debug)]
pub struct TestFile<'a> {
    pub parent: String,
    pub name: String,
    pub contents: &'a str,
}

impl<'a> TestFile<'a> {
    /// Get the name of the test file
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the parent of the test file (either a proposal or a spec version)
    pub fn parent(&self) -> &str {
        &self.parent
    }

    /// Get the raw contents of the test file
    pub fn raw(&self) -> &'a str {
        self.contents
    }

    #[cfg(feature = "wast")]
    /// Parse the contents of the test file
    pub fn wast(&self) -> wast::parser::Result<WastBuffer<'a>> {
        let mut lexer = wast::lexer::Lexer::new(self.contents);
        lexer.allow_confusing_unicode(true);
        let parse_buffer = wast::parser::ParseBuffer::new_with_lexer(lexer)?;

        Ok(WastBuffer {
            buffer: parse_buffer,
        })
    }
}

/// A wrapper around [`wast::parser::ParseBuffer`]
pub struct WastBuffer<'a> {
    // this wrapper struct is neccecary to not use self-referential structs as WastDirective
    // contains a reference to the buffer which contains a reference to the original file data.
    buffer: wast::parser::ParseBuffer<'a>,
}

impl Debug for WastBuffer<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WastBuffer").finish()
    }
}

impl<'a> WastBuffer<'a> {
    // Get the wast directives from the buffer
    pub fn directives(&'a self) -> wast::parser::Result<Vec<wast::WastDirective<'a>>> {
        Ok(wast::parser::parse::<wast::Wast<'a>>(&self.buffer)?.directives)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proposals() {
        for p in Proposal::all() {
            for test in proposal(p) {
                test.wast()
                    .expect("Failed to lex wast")
                    .directives()
                    .expect("Failed to parse wast");
            }
        }
    }

    #[test]
    fn test_spec_versions() {
        for v in SpecVersion::all() {
            for test in spec(v) {
                test.wast()
                    .expect("Failed to lex wast")
                    .directives()
                    .expect("Failed to parse wast");
            }
        }
    }
}
