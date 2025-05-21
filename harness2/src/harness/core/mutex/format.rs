use std::fmt::Display;

use crate::harness::core::{error::HarnessError, process::ProcessSet, state_machine::StateMachineContext};

use super::segment::MutualExclusionSegment;

pub struct MutualExclusionSegmentFormatter(Vec<(String, String)>);

impl Display for MutualExclusionSegmentFormatter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{{")?;
        for (process, state) in self.0.iter() {
            writeln!(f, "\t{}: {}", process, state)?;
        }
        write!(f, "}}")
    }
}

impl MutualExclusionSegmentFormatter {
    pub fn new(context: &StateMachineContext, process_set: &ProcessSet, segment: &MutualExclusionSegment) -> Result<MutualExclusionSegmentFormatter, HarnessError> {
        Ok(MutualExclusionSegmentFormatter(
            segment.iter().map(| (process, state) | -> Result<(String, String), HarnessError> {
                let process_mnemonic = process_set.get_process_mnemonic(process)
                    .ok_or(HarnessError::new("Unable to retrieve process mnemonic"))?;
                let state_mnemonic = context.get_node_mnemonic(state)
                    .ok_or(HarnessError::new("Unable to retrieve node mnemonic"))?;
                Ok((process_mnemonic.into(), state_mnemonic.into()))
            }).collect::<Result<Vec<(String, String)>, HarnessError>>()?
        ))
    }
}
