use std::{collections::{HashMap, HashSet}, hash::Hash};

use crate::harness::core::{error::HarnessError, process::ProcessID, state_machine::{StateMachineContext, StateMachineEdgeID, StateMachineMessageDestination, StateMachineMessageEnvelope, StateMachineMessageID, StateMachineMessageParticipantID, StateMachineNodeID}};

pub struct StateMachineProductNodeBuilder(StateMachineNodeID, usize);

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct ProductEntry(Vec<StateMachineNodeID>);

pub struct StateMachineProductNode {
    root_node: StateMachineNodeID,
    product_subnodes: HashMap<ProductEntry, StateMachineNodeID>,
    inbound_product_messages: HashMap<usize, HashMap<StateMachineMessageID, StateMachineMessageID>>,
    outbound_response_messages: HashMap<StateMachineEdgeID, usize>
}

impl ProductEntry {
    pub fn new(base: StateMachineNodeID, count: usize) -> ProductEntry {
        let mut content = Vec::new();
        for _ in 0..count {
            content.push(base);
        }
        ProductEntry(content)
    }

    pub fn mnemonic(&self, context: &StateMachineContext) -> Result<String, HarnessError> {
        let mut mnemonic = String::new();
        mnemonic.push('(');
        for (i, subnode) in self.0.iter().enumerate() {
            if i > 0 {
                mnemonic.push_str(", ");
            }
            mnemonic.push_str(context.get_node_mnemonic(*subnode).ok_or(HarnessError::new("Unable to find subnode to build product node mnemonic"))?);
        }
        mnemonic.push(')');
        Ok(mnemonic)
    }
}

impl From<Vec<StateMachineNodeID>> for ProductEntry {
    fn from(value: Vec<StateMachineNodeID>) -> Self {
        ProductEntry(value)
    }
}

impl StateMachineProductNodeBuilder {
    pub fn new(base: StateMachineNodeID, count: usize) -> StateMachineProductNodeBuilder {
        StateMachineProductNodeBuilder(base, count)
    }

    pub fn base_node(&self) -> StateMachineNodeID {
        self.0
    }

    pub fn count(&self) -> usize {
        self.1
    }

    fn get_product_message_mnemonic(index: usize, total: usize, base_message: &str) -> String {
        let mut result = String::new();
        result.push('(');
        for i in 0..total {
            if i > 0 {
                result.push_str(", ");
            }
            if i == index {
                result.push_str(base_message);
            } else {
                result.push('_');
            }
        }
        result.push(')');
        result
    }

    pub fn build(&self, context: &mut StateMachineContext) -> Result<StateMachineProductNode, HarnessError> {
        let mut visited = HashSet::new();
        let mut product_subnodes = HashMap::new();
        let root_entry = ProductEntry::new(self.base_node(), self.count());
        let mut queue = Vec::from([root_entry.clone()]);
        let mut inbound_product_messages = HashMap::new();
        let mut outbound_response_messages = HashMap::new();
        while !queue.is_empty() {
            let entry = queue.pop().unwrap();
            if visited.contains(&entry) {
                continue;
            }
            visited.insert(entry.clone());
            

            let entry_product_node = match product_subnodes.get(&entry) {
                Some(product_node) => *product_node,
                None => {
                    let product_node = context.new_node(entry.mnemonic(context)?)?;
                    product_subnodes.insert(entry.clone(), product_node);
                    product_node
                }
            };

            for i in 0..self.count() {
                inbound_product_messages.entry(i).or_insert(HashMap::new());
                let edges = context.get_edges_from(*entry.0.get(i).unwrap())
                    .ok_or(HarnessError::new("Unable to find edges coming from node for product node construction"))?
                    .collect::<Vec<_>>();
                for edge in edges {
                    let mut new_content = entry.0.clone();
                    new_content[i] = context.get_edge_target(edge).ok_or(HarnessError::new("Unable to find edge target for product node construction"))?;
                    let next_entry: ProductEntry = new_content.into();
                    let next_entry_product_node = match product_subnodes.get(&next_entry) {
                        Some(product_node) => *product_node,
                        None => {
                            let next_entry_mnemonic = next_entry.mnemonic(context)?;
                            let product_node = context.new_node(next_entry_mnemonic)?;
                            product_subnodes.insert(next_entry.clone(), product_node);
                            product_node
                        }
                    };
                    let trigger: Option<StateMachineMessageID> = match context.get_edge_trigger(edge) {
                        Some(msg) => {
                            let product_msg = match inbound_product_messages.get(&i).unwrap().get(&msg){
                                Some(product_msg) => *product_msg,
                                None => {
                                    let product_msg = context.new_message(StateMachineProductNodeBuilder::get_product_message_mnemonic(i, self.count(), context.get_message_mnemonic(msg).ok_or(HarnessError::new("Unable to find message mnemonic for product message construction"))?))?;
                                    inbound_product_messages.get_mut(&i).expect("Expected inbound product message map to exist")
                                        .insert(msg, product_msg);
                                    product_msg
                                }
                            };
                            Some(product_msg)
                        },
                        None => None
                    };
                    let product_edge = context.new_edge(entry_product_node, next_entry_product_node, trigger, context.get_edge_action(edge))?;
                    outbound_response_messages.insert(product_edge, i);
                    queue.push(next_entry);
                }
            };
        };

        let root_node = *product_subnodes.get(&root_entry).expect("Expected node to exist");
        Ok(StateMachineProductNode {
            root_node,
            product_subnodes,
            inbound_product_messages,
            outbound_response_messages
        })
    }
}

impl StateMachineProductNode {
    pub fn get_root_node(&self) -> StateMachineNodeID {
        self.root_node
    }

    pub fn get_product_subnode_for<T: Into<Vec<StateMachineNodeID>>>(&self, subnodes: T) -> Option<StateMachineNodeID> {
        self.product_subnodes.get(&ProductEntry(subnodes.into())).map(| node | *node)
    }

    pub fn get_inbound_message_mapping(&self, participants: impl Iterator<Item = StateMachineMessageParticipantID>) -> Result<impl Fn(ProcessID, StateMachineMessageID) -> Option<StateMachineMessageID> + 'static, HarnessError> {
        let participants = participants.collect::<Vec<StateMachineMessageParticipantID>>();
        for i in 0..participants.len() {
            if !self.inbound_product_messages.contains_key(&i) {
                return Err(HarnessError::new("Unable to match product node inbound messages to provided participant list"));
            }
        }

        let inbound_product_messages = self.inbound_product_messages.clone();
        Ok(move | sender, message | {
            for (i, &participant) in participants.iter().enumerate() {
                let sender_participant = Into::<StateMachineMessageParticipantID>::into(sender);
                if sender_participant == participant {
                    let sender_mapping = inbound_product_messages.get(&i).expect("Expected sender message mapping to exist");
                    match sender_mapping.get(&message) {
                        Some(mapped_message) => return Some(*mapped_message),
                        None => ()
                    }
                }
            }

            None
        })
    }

    pub fn get_outbound_message_mapping(&self, participants: impl Iterator<Item = StateMachineMessageParticipantID>) -> Result<impl for<'a> Fn(StateMachineEdgeID, &'a StateMachineMessageEnvelope) -> Option<StateMachineMessageEnvelope> + 'static, HarnessError> {
        let participants = participants.collect::<Vec<StateMachineMessageParticipantID>>();
        let outbound_response_messages = self.outbound_response_messages.clone();
        for &participant_idx in outbound_response_messages.values() {
            if participant_idx >= participants.len() {
                return Err(HarnessError::new("Unable to match product node outbound messages to provided participant list"));
            }
        }
    
        Ok(move | edge, envelope: &StateMachineMessageEnvelope | {
            match envelope.get_destination() {
                StateMachineMessageDestination::Response => if let Some(&participant_idx) = outbound_response_messages.get(&edge) {
                    let participant = participants.get(participant_idx).expect("Expected participant to exist");
                    return Some(envelope.redirect(StateMachineMessageDestination::Unicast(*participant)))
                },
                _ => ()
            }

            None
        })
    }
}
