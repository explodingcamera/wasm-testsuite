///! Access to raw `.wast` test files
use std::str::FromStr;

use include_dir::{include_dir, Dir, File};
static DATA: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/data");

#[derive(Debug)]
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

impl Into<&'static str> for Proposal {
    fn into(self) -> &'static str {
        match self {
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

#[derive(Debug)]
pub enum SpecVersion {
    V1,
    V2,
    V3,
}

impl SpecVersion {
    fn name(&self) -> &'static str {
        match self {
            Self::V1 => "wasm-v1",
            Self::V2 => "wasm-v2",
            Self::V3 => "wasm-v3",
        }
    }

    pub fn all() -> &'static [SpecVersion] {
        &[SpecVersion::V1, SpecVersion::V2, SpecVersion::V3]
    }
}

/// Get all test files associated with a proposal
pub fn proposal(name: Proposal) -> Option<impl Iterator<Item = &'static File<'static>> + 'static> {
    let name: &'static str = name.into();
    let tests = DATA.get_dir(format!("proposals/{}", name))?;
    Some(tests.files())
}

/// Get all test files associated with a spec version
pub fn spec(
    version: SpecVersion,
) -> Option<impl Iterator<Item = &'static File<'static>> + 'static> {
    let name = version.name();
    let tests = DATA.get_dir(format!("{}", name))?;
    Some(tests.files())
}
