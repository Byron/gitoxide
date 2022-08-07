use crate::instruction::{Fetch, Push};
use crate::{parse::Operation, types::Mode, Instruction, RefSpec, RefSpecRef};

/// Conversion. Use the [RefSpecRef][RefSpec::to_ref()] type for more usage options.
impl RefSpec {
    /// Return ourselves as reference type.
    pub fn to_ref(&self) -> RefSpecRef<'_> {
        RefSpecRef {
            mode: self.mode,
            op: self.op,
            src: self.src.as_ref().map(|b| b.as_ref()),
            dst: self.dst.as_ref().map(|b| b.as_ref()),
        }
    }
}

mod impls {
    use crate::{RefSpec, RefSpecRef};

    impl From<RefSpecRef<'_>> for RefSpec {
        fn from(v: RefSpecRef<'_>) -> Self {
            v.to_owned()
        }
    }
}

/// Access
impl RefSpecRef<'_> {
    /// Transform the state of the refspec into an instruction making clear what to do with it.
    pub fn instruction(&self) -> Instruction<'_> {
        match self.op {
            Operation::Fetch => match (self.mode, self.src, self.dst) {
                (Mode::Normal | Mode::Force, Some(src), None) => Instruction::Fetch(Fetch::Only { src }),
                (Mode::Normal | Mode::Force, Some(src), Some(dst)) => Instruction::Fetch(Fetch::AndUpdate {
                    src,
                    dst,
                    allow_non_fast_forward: matches!(self.mode, Mode::Force),
                }),
                (Mode::Negative, Some(src), None) => Instruction::Fetch(Fetch::Exclude { src }),
                (mode, src, dest) => {
                    unreachable!(
                        "BUG: fetch instructions with {:?} {:?} {:?} are not possible",
                        mode, src, dest
                    )
                }
            },
            Operation::Push => match (self.mode, self.src, self.dst) {
                (Mode::Normal | Mode::Force, Some(src), None) => Instruction::Push(Push::Matching {
                    src,
                    dst: src,
                    allow_non_fast_forward: matches!(self.mode, Mode::Force),
                }),
                (Mode::Normal | Mode::Force, None, Some(dst)) => {
                    Instruction::Push(Push::Delete { ref_or_pattern: dst })
                }
                (Mode::Normal | Mode::Force, None, None) => Instruction::Push(Push::AllMatchingBranches {
                    allow_non_fast_forward: matches!(self.mode, Mode::Force),
                }),
                (Mode::Normal | Mode::Force, Some(src), Some(dst)) => Instruction::Push(Push::Matching {
                    src,
                    dst,
                    allow_non_fast_forward: matches!(self.mode, Mode::Force),
                }),
                (mode, src, dest) => {
                    unreachable!(
                        "BUG: push instructions with {:?} {:?} {:?} are not possible",
                        mode, src, dest
                    )
                }
            },
        }
    }
}

/// Conversion
impl RefSpecRef<'_> {
    /// Convert this ref into a standalone, owned copy.
    pub fn to_owned(&self) -> RefSpec {
        RefSpec {
            mode: self.mode,
            op: self.op,
            src: self.src.map(ToOwned::to_owned),
            dst: self.dst.map(ToOwned::to_owned),
        }
    }
}
