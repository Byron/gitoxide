use crate::types::Push;
use crate::{Fetch, Instruction, Mode, Operation, RefSpec, RefSpecRef};
use bstr::BStr;

/// Access
impl RefSpecRef<'_> {
    /// Return the refspec mode.
    pub fn mode(&self) -> Mode {
        self.mode
    }

    /// Transform the state of the refspec into an instruction making clear what to do with it.
    pub fn instruction(&self) -> Instruction<'_> {
        fn has_pattern(item: &BStr) -> bool {
            item.contains(&b'*')
        }
        match (self.op, self.mode, self.src, self.dst) {
            (Operation::Fetch, Mode::Normal | Mode::Force, Some(src), None) => Instruction::Fetch(Fetch::Only { src }),
            (Operation::Fetch, Mode::Normal | Mode::Force, Some(src), Some(dst)) => {
                Instruction::Fetch(Fetch::AndUpdateSingle {
                    src,
                    dst,
                    allow_non_fast_forward: matches!(self.mode, Mode::Force),
                })
            }
            (Operation::Push, Mode::Normal | Mode::Force, Some(src), None) => Instruction::Push(Push::Single {
                src,
                dst: src,
                allow_non_fast_forward: matches!(self.mode, Mode::Force),
            }),
            (Operation::Push, Mode::Normal | Mode::Force, None, Some(dst)) => {
                Instruction::Push(Push::Delete { ref_or_pattern: dst })
            }
            (Operation::Push, Mode::Normal | Mode::Force, None, None) => Instruction::Push(Push::AllMatchingBranches {
                allow_non_fast_forward: matches!(self.mode, Mode::Force),
            }),
            (Operation::Fetch, Mode::Normal | Mode::Force, None, None) => {
                Instruction::Fetch(Fetch::AllMatchingBranches {
                    allow_non_fast_forward: matches!(self.mode, Mode::Force),
                })
            }
            (Operation::Push, Mode::Normal | Mode::Force, Some(src), Some(dst)) if has_pattern(src) => {
                Instruction::Push(Push::MultipleWithGlob {
                    src,
                    dst,
                    allow_non_fast_forward: matches!(self.mode, Mode::Force),
                })
            }
            (Operation::Push, Mode::Normal | Mode::Force, Some(src), Some(dst)) => Instruction::Push(Push::Single {
                src,
                dst,
                allow_non_fast_forward: matches!(self.mode, Mode::Force),
            }),
            (Operation::Push, Mode::Negative, Some(src), None) if has_pattern(src) => {
                Instruction::Push(Push::ExcludeMultipleWithGlob { src })
            }
            (Operation::Push, Mode::Negative, Some(src), None) => Instruction::Push(Push::ExcludeSingle { src }),
            (op, mode, src, dest) => {
                unreachable!(
                    "BUG: instructions with {:?} {:?} {:?} {:?} are not possible",
                    op, mode, src, dest
                )
            }
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
