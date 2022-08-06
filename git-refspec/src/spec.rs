use crate::types::Push;
use crate::{Instruction, Mode, Operation, RefSpec, RefSpecRef};
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
        match (self.op, self.mode, self.src, self.dest) {
            (Operation::Push, Mode::Normal | Mode::Force, Some(src), None) => Instruction::Push(Push::Single {
                src,
                dest: src,
                allow_non_fast_forward: matches!(self.mode, Mode::Force),
            }),
            (Operation::Push, Mode::Normal | Mode::Force, None, None) => Instruction::Push(Push::AllMatchingBranches {
                allow_non_fast_forward: matches!(self.mode, Mode::Force),
            }),
            (Operation::Push, Mode::Normal | Mode::Force, Some(src), Some(dest)) if has_pattern(src) => {
                Instruction::Push(Push::MultipleWithGlob {
                    src,
                    dest,
                    allow_non_fast_forward: matches!(self.mode, Mode::Force),
                })
            }
            (Operation::Push, Mode::Normal | Mode::Force, Some(src), Some(dest)) => Instruction::Push(Push::Single {
                src,
                dest,
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
            dest: self.dest.map(ToOwned::to_owned),
        }
    }
}
