use std::fmt::Display;

use super::{error::HarnessError, process::ProcessSet, process_state::{ProcessSetState, ProcessSetStateSpace}, state_machine::StateMachineContext};

pub struct ProcessSetStateFormatter(Vec<(String, String, Vec<(String, String)>)>);

pub struct ProcessSetStateSpaceFormatter(Vec<ProcessSetStateFormatter>);

impl Display for ProcessSetStateFormatter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{{")?;
        for (process, state, inbox) in &self.0 {
            write!(f, "\t{}: {}", process, state)?;
            for (inbox_idx, (origin, message)) in inbox.iter().enumerate() {
                if inbox_idx == 0 {
                    write!(f, " [")?;
                } else {
                    write!(f, ", ")?;
                }
                write!(f, "{} from {}", origin, message)?;
            }
            if !inbox.is_empty() {
                write!(f, "]")?;
            }
            write!(f, "\n")?;
        }
        write!(f, "}}")
    }
}

impl Display for ProcessSetStateSpaceFormatter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (index, entry) in self.0.iter().enumerate() {
            if index > 0 {
                writeln!(f, "")?;
            }
            entry.fmt(f)?;
        }
        Ok(())
    }
}

impl ProcessSetStateFormatter {
    pub fn new(context: &StateMachineContext, process_set: &ProcessSet, process_set_state: &ProcessSetState) -> Result<ProcessSetStateFormatter, HarnessError> {
        Ok(ProcessSetStateFormatter(process_set.iter()
            .map(| process| -> Result<(String, String, Vec<(String, String)>), HarnessError> {
                let process_mnemonic = process_set.get_process_mnemonic(process)
                    .ok_or(HarnessError::new("Unable to retrieve process mnemonic"))?;
                let process_state = process_set_state.get_process_node(process)
                    .ok_or(HarnessError::new("Unable to retrieve process state"))?;
                let process_state_mnemonic = context.get_node_mnemonic(process_state)
                    .ok_or(HarnessError::new("Unable to retrieve node mnemonic"))?;
                let process_inbox = process_set_state.get_process_inbox(process)
                    .ok_or(HarnessError::new("Unable to retrieve process inbox"))?;
                let process_inbox_mnemonics = process_inbox.map(| (origin, message) | {
                    let origin_mnemonic = process_set.get_process_mnemonic(origin)
                        .ok_or(HarnessError::new("Unable to retrieve message sender process mnemonic"))?;
                    let message_mnemonic = context.get_message_mnemonic(message)
                        .ok_or(HarnessError::new("Unable to retrieve message mnemonic"))?;
                    Ok((origin_mnemonic.into(), message_mnemonic.into()))
                }).collect::<Result<_, HarnessError>>()?;
                Ok((process_mnemonic.into(), process_state_mnemonic.into(), process_inbox_mnemonics))
            }).collect::<Result<_, HarnessError>>()?))
    }
}

impl ProcessSetStateSpaceFormatter {
    pub fn new(context: &StateMachineContext, process_set: &ProcessSet, state_space: &ProcessSetStateSpace) -> Result<ProcessSetStateSpaceFormatter, HarnessError> {
        Ok(ProcessSetStateSpaceFormatter(
            state_space.iter()
                .map(| state | ProcessSetStateFormatter::new(context, process_set, state))
                .collect::<Result<Vec<ProcessSetStateFormatter>, HarnessError>>()?
        ))
    }
}