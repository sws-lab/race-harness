use std::collections::BTreeMap;

use super::{process_state::{ProcessSetState, ProcessSetStateSpace}, error::HarnessError, state_machine::{StateMachineContext, StateMachineEdgeID, StateMachineMessageEnvelope, StateMachineMessageID, StateMachineMessageParticipantID, StateMachineNodeID}};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct ProcessID(u64);

struct Process {
    mnemonic: String,
    entry_node: StateMachineNodeID,
    inbound_message_mappings: Vec<Box<dyn Fn(ProcessID, StateMachineMessageID) -> Option<StateMachineMessageID> + 'static>>,
    outbound_message_mappings: Vec<Box<dyn Fn(StateMachineEdgeID, &StateMachineMessageEnvelope) -> Option<StateMachineMessageEnvelope> + 'static>>
}

pub struct ProcessSet {
    processes: BTreeMap<ProcessID, Process>
}

impl From<ProcessID> for u64 {
    fn from(value: ProcessID) -> Self {
        value.0
    }
}

impl From<ProcessID> for StateMachineMessageParticipantID {
    fn from(value: ProcessID) -> Self {
        StateMachineMessageParticipantID(value.0)
    }
}

impl ProcessSet {
    pub fn new() -> ProcessSet {
        ProcessSet {
            processes: BTreeMap::new()
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = ProcessID> {
        self.processes.keys().map(| process_id | *process_id)
    }

    pub fn new_process(&mut self, mnemonic: String, entry_node: StateMachineNodeID) -> ProcessID {
        let process_id = ProcessID(self.processes.len() as u64);
        let process = Process {
            mnemonic,
            entry_node,
            inbound_message_mappings: Vec::new(),
            outbound_message_mappings: Vec::new()
        };
        self.processes.insert(process_id, process);
        process_id
    }

    pub fn new_inbound_message_mapping(&mut self, process_id: ProcessID, mapping: impl Fn(ProcessID, StateMachineMessageID) -> Option<StateMachineMessageID> + 'static) -> Result<(), HarnessError> {
        self.processes.get_mut(&process_id)
            .ok_or(HarnessError::new("Unable to find process to add inbound message mapping"))?
            .inbound_message_mappings.push(Box::new(mapping));
        Ok(())
    }

    pub fn new_outbound_message_mapping(&mut self, process_id: ProcessID, mapping: impl Fn(StateMachineEdgeID, &StateMachineMessageEnvelope) -> Option<StateMachineMessageEnvelope> + 'static) -> Result<(), HarnessError> {
        self.processes.get_mut(&process_id)
            .ok_or(HarnessError::new("Unable to find process to add outbound message mapping"))?
            .outbound_message_mappings.push(Box::new(mapping));
        Ok(())
    }

    pub fn get_process_mnemonic(&self, process_id: ProcessID) -> Option<&str> {
        self.processes.get(&process_id).map(| process | process.mnemonic.as_str())
    }

    pub fn get_process_entry_node(&self, process_id: ProcessID) -> Option<StateMachineNodeID> {
        self.processes.get(&process_id).map(| process | process.entry_node)
    }

    pub fn get_processes(&self) -> impl Iterator<Item = ProcessID> {
        self.processes.iter().map(| (k, _) | *k)
    }

    pub fn map_inbound_message(&self, receiver_id: ProcessID, sender_id: ProcessID, message: StateMachineMessageID) -> Result<StateMachineMessageID, HarnessError> {
        let mappings = &self.processes.get(&receiver_id).ok_or(HarnessError::new("Unable to find a process to map inbound message"))?.inbound_message_mappings;
        for mapping in mappings {
            match mapping(sender_id, message) {
                Some(msg) => return Ok(msg),
                None => ()
            };
        }
        Ok(message)
    }

    pub fn map_outbound_message(&self, sender_id: ProcessID, origin_edge: StateMachineEdgeID, envelope: &StateMachineMessageEnvelope) -> Result<StateMachineMessageEnvelope, HarnessError> {
        let mappings = &self.processes.get(&sender_id).ok_or(HarnessError::new("Unable to find a process to map outbound message"))?.outbound_message_mappings;
        for mapping in mappings {
            match mapping(origin_edge, &envelope) {
                Some(env) => return Ok(env),
                None => ()
            }
        }
        Ok(envelope.clone())
    }

    pub fn get_initial_state(&self) -> ProcessSetState {
        ProcessSetState::new(self)
    }

    pub fn get_state_space(&self, context: &StateMachineContext) -> Result<ProcessSetStateSpace, HarnessError> {
        self.get_initial_state().get_reachable_state_space(context, self)
    }
}
