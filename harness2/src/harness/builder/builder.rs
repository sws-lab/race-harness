use std::{collections::{HashMap, HashSet}, env};

use crate::harness::{codegen::template::CodegenTemplate, core::{error::HarnessError, process::{ProcessID, ProcessSet}, state_machine::{StateMachineActionID, StateMachineContext, StateMachineMessageDestination, StateMachineMessageID, StateMachineMessageParticipantID, StateMachineNodeID}}, entities::product_node::{StateMachineProductNode, StateMachineProductNodeBuilder}};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct HarnessBuilderSymbolID(u64);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum HarnessBuilderSymbolType {
    State,
    Message,
    Action,
    Process
}

enum HarnessStateBuilder {
    Primitive {
        mnemonic: String
    },

    Product {
        mnemonic: String,
        base_state: HarnessBuilderSymbolID,
        mapped_processes: Vec<HarnessBuilderSymbolID>
    }
}

struct HarnessMessageBuilder {
    mnemonic: String
}

struct HarnessEdgeBuilder {
    source: HarnessBuilderSymbolID,
    target: HarnessBuilderSymbolID,
    trigger: Option<HarnessBuilderSymbolID>,
    action: Option<HarnessBuilderSymbolID>
}

enum HarnessEnvelopeBuilder {
    Unicast(HarnessBuilderSymbolID, HarnessBuilderSymbolID),
    Multicast(Vec<HarnessBuilderSymbolID>, HarnessBuilderSymbolID),
    Response(HarnessBuilderSymbolID)
}

struct HarnessActionBuilder {
    mnemonic: String,
    envelopes: Vec<HarnessEnvelopeBuilder>,
    content: Option<String>
}

struct HarnessProcessBuilder {
    mnemonic: String,
    entry_state: HarnessBuilderSymbolID,
    parameters: HashMap<String, String>,
    prologue: Option<String>
}

struct HarnessBuilderState {
    states: HashMap<HarnessBuilderSymbolID, StateMachineNodeID>,
    messages: HashMap<HarnessBuilderSymbolID, StateMachineMessageID>,
    actions: HashMap<HarnessBuilderSymbolID, StateMachineActionID>,
    processes: HashMap<HarnessBuilderSymbolID, ProcessID>,
    visited_edges: HashSet<HarnessBuilderSymbolID>
}

pub struct HarnessBuilder {
    next_symbol: u64,
    named_symbols: HashMap<String, (HarnessBuilderSymbolID, HarnessBuilderSymbolType)>,
    states: HashMap<HarnessBuilderSymbolID, HarnessStateBuilder>,
    messages: HashMap<HarnessBuilderSymbolID, HarnessMessageBuilder>,
    edges: HashMap<HarnessBuilderSymbolID, HarnessEdgeBuilder>,
    direct_edges: HashMap<HarnessBuilderSymbolID, Vec<HarnessBuilderSymbolID>>,
    actions: HashMap<HarnessBuilderSymbolID, HarnessActionBuilder>,
    processes: HashMap<HarnessBuilderSymbolID, HarnessProcessBuilder>,
    global_prologue: Option<String>
}

impl HarnessBuilder {
    pub fn new() -> HarnessBuilder {
        HarnessBuilder {
            next_symbol: 0,
            named_symbols: HashMap::new(),
            states: HashMap::new(),
            messages: HashMap::new(),
            edges: HashMap::new(),
            direct_edges: HashMap::new(),
            actions: HashMap::new(),
            processes: HashMap::new(),
            global_prologue: None
        }
    }

    pub fn new_primitive_state(&mut self, mnemonic: &str) -> Result<HarnessBuilderSymbolID, HarnessError> {
        let symbol = self.new_named_symbol(mnemonic, HarnessBuilderSymbolType::State)?;
        self.states.entry(symbol)
            .or_insert(HarnessStateBuilder::Primitive { mnemonic: mnemonic.into() });
        Ok(symbol)
    }

    pub fn new_product_state(&mut self, mnemonic: &str, base_state: HarnessBuilderSymbolID, mapped_processes: impl Iterator<Item = HarnessBuilderSymbolID>) -> Result<HarnessBuilderSymbolID, HarnessError> {
        let symbol = self.new_named_symbol(mnemonic, HarnessBuilderSymbolType::State)?;
        if !self.states.contains_key(&symbol) {
            if !self.states.contains_key(&base_state) {
                return Err(HarnessError::new("Unknown product state base state symbol"));
            }
            let mapped_processes = mapped_processes.collect::<Vec<_>>();
            if mapped_processes.iter().any(| process | !self.processes.contains_key(&process)) {
                return Err(HarnessError::new("Unknown product state mapped process symbol"));
            }

            self.states.insert(symbol, HarnessStateBuilder::Product { mnemonic: mnemonic.into(), base_state, mapped_processes });
        }
        
        Ok(symbol)
    }

    pub fn new_message(&mut self, mnemonic: &str) -> Result<HarnessBuilderSymbolID, HarnessError> {
        let symbol = self.new_named_symbol(mnemonic, HarnessBuilderSymbolType::Message)?;
        self.messages.entry(symbol)
            .or_insert(HarnessMessageBuilder { mnemonic: mnemonic.into() });
        Ok(symbol)
    }

    pub fn new_action(&mut self, mnemonic: &str) -> Result<HarnessBuilderSymbolID, HarnessError> {
        let symbol = self.new_named_symbol(mnemonic, HarnessBuilderSymbolType::Action)?;
        self.actions.entry(symbol)
            .or_insert(HarnessActionBuilder { mnemonic: mnemonic.into(), envelopes: Vec::new(), content: None });
        Ok(symbol)
    }

    pub fn new_process(&mut self, mnemonic: &str, entry_state: HarnessBuilderSymbolID) -> Result<HarnessBuilderSymbolID, HarnessError> {
        if !self.states.contains_key(&entry_state) {
            return Err(HarnessError::new("Unknown process entry state symbol"));
        }
        let symbol = self.new_named_symbol(mnemonic, HarnessBuilderSymbolType::Process)?;
        self.processes.entry(symbol)
            .or_insert(HarnessProcessBuilder { mnemonic: mnemonic.into(), entry_state, parameters: HashMap::new(), prologue: None });
        Ok(symbol)
    }

    pub fn new_edge(&mut self, source: HarnessBuilderSymbolID, target: HarnessBuilderSymbolID, trigger: Option<HarnessBuilderSymbolID>, action: Option<HarnessBuilderSymbolID>) -> Result<HarnessBuilderSymbolID, HarnessError> {
        if !self.states.contains_key(&source) {
            return Err(HarnessError::new("Unknown edge source state symbol"));
        }
        if !self.states.contains_key(&target) {
            return Err(HarnessError::new("Unknown edge target state symbol"));
        }
        if let Some(trigger) = trigger {
            if !self.messages.contains_key(&trigger) {
                return Err(HarnessError::new("Unknown edge trigger message symbol"));
            }
        }
        if let Some(action) = action {
            if !self.actions.contains_key(&action) {
                return Err(HarnessError::new("Unknown edge action symbol"));
            }
        }

        let symbol = self.new_symbol();
        self.edges.insert(symbol, HarnessEdgeBuilder { source, target, trigger, action });
        self.direct_edges.entry(source)
            .or_insert(Vec::new())
            .push(symbol);
        Ok(symbol)
    }

    pub fn set_global_prologue(&mut self, prologue: String) {
        match &mut self.global_prologue {
            Some(head) => {
                head.push_str(&prologue);
            }

            None => self.global_prologue = Some(prologue)
        }
    }

    pub fn set_process_prologue(&mut self, process: HarnessBuilderSymbolID, prologue: String) -> Result<(), HarnessError> {
        self.processes.get_mut(&process)
            .ok_or(HarnessError::new("Unknown process symbol"))?
            .prologue = Some(prologue);
        Ok(())
    }

    pub fn set_process_parameters(&mut self, process: HarnessBuilderSymbolID, key: String, value: String) -> Result<(), HarnessError> {
        self.processes.get_mut(&process)
            .ok_or(HarnessError::new("Unknown process symbol"))?
            .parameters.insert(key, value);
        Ok(())
    }

    pub fn set_action_content(&mut self, action: HarnessBuilderSymbolID, content: String) -> Result<(), HarnessError> {
        let action = match self.actions.get_mut(&action) {
            Some(action) => action,
            None => return Err(HarnessError::new("Unknown action symbol"))
        };
        action.content = Some(content);
        Ok(())
    }

    pub fn new_unicast_envelope(&mut self, action: HarnessBuilderSymbolID, destination: HarnessBuilderSymbolID, message: HarnessBuilderSymbolID) -> Result<(), HarnessError> {
        let action = match self.actions.get_mut(&action) {
            Some(action) => action,
            None => return Err(HarnessError::new("Unknown action symbol"))
        };
        if !self.processes.contains_key(&destination) {
            return Err(HarnessError::new("Unknown envelope destination process symbol"));
        }
        if !self.messages.contains_key(&message) {
            return Err(HarnessError::new("Unknown envelope message symbol"));
        }

        action.envelopes.push(HarnessEnvelopeBuilder::Unicast(destination, message));
        Ok(())
    }

    pub fn new_multicast_envelope(&mut self, action: HarnessBuilderSymbolID, destinations: impl Iterator<Item = HarnessBuilderSymbolID>, message: HarnessBuilderSymbolID) -> Result<(), HarnessError> {
        let action = match self.actions.get_mut(&action) {
            Some(action) => action,
            None => return Err(HarnessError::new("Unknown action symbol"))
        };
        let destinations = destinations.collect::<Vec<_>>();
        if destinations.iter().any(| process | !self.processes.contains_key(&process)) {
            return Err(HarnessError::new("Unknown envelope destination process symbol"));
        }
        if !self.messages.contains_key(&message) {
            return Err(HarnessError::new("Unknown envelope message symbol"));
        }

        action.envelopes.push(HarnessEnvelopeBuilder::Multicast(destinations, message));
        Ok(())
    }

    pub fn new_response_envelope(&mut self, action: HarnessBuilderSymbolID, message: HarnessBuilderSymbolID) -> Result<(), HarnessError> {
        let action = match self.actions.get_mut(&action) {
            Some(action) => action,
            None => return Err(HarnessError::new("Unknown action symbol"))
        };
        if !self.messages.contains_key(&message) {
            return Err(HarnessError::new("Unknown envelope message symbol"));
        }

        action.envelopes.push(HarnessEnvelopeBuilder::Response(message));
        Ok(())
    }

    fn new_symbol(&mut self) -> HarnessBuilderSymbolID {
        let symbol = HarnessBuilderSymbolID(self.next_symbol);
        self.next_symbol += 1;
        symbol
    }

    fn new_named_symbol(&mut self, name: &str, symbol_type: HarnessBuilderSymbolType) -> Result<HarnessBuilderSymbolID, HarnessError> {
        if let Some(&(symbol, existing_type)) = self.named_symbols.get(name) {
            if existing_type != symbol_type {
                Err(HarnessError::new("Another named symbol with mismatching type already exists"))
            } else {
                Ok(symbol)
            }
        } else {
            let symbol = self.new_symbol();
            self.named_symbols.insert(name.into(), (symbol, symbol_type));
            Ok(symbol)
        }
    }

    pub fn build(&self, context: &mut StateMachineContext, process_set: &mut ProcessSet) -> Result<CodegenTemplate, HarnessError> {
        let mut template = CodegenTemplate::new();
        let mut builder = HarnessBuilderState {
            states: HashMap::new(),
            messages: HashMap::new(),
            actions: HashMap::new(),
            processes: HashMap::new(),
            visited_edges: HashSet::new()
        };

        let mut state_queue = Vec::new();
        for (&symbol, process_builder) in &self.processes {
            let entry_node = self.build_entry_state(context, &mut builder, process_builder.entry_state)?;
            let process = process_set.new_process(process_builder.mnemonic.clone(), entry_node);
            if let Some(prologue) = &process_builder.prologue {
                template.set_process_prologue(process, prologue);
            }
            for (key, value) in &process_builder.parameters {
                template.set_process_parameter(process, &key, value);
            }
            builder.processes.insert(symbol, process);
            state_queue.push((process, process_builder.entry_state));
        }

        for (process, symbol) in state_queue {
            self.build_state(context, process_set, &mut template, &mut builder, process, symbol)?;
        }

        if let Some(prologue) = &self.global_prologue {
            template.set_global_prologue(Some(prologue));
        }

        Ok(template)
    }

    fn build_entry_state(&self, context: &mut StateMachineContext, builder: &mut HarnessBuilderState, symbol: HarnessBuilderSymbolID) -> Result<StateMachineNodeID, HarnessError> {
        if let Some(&node) = builder.states.get(&symbol) {
            return Ok(node);
        }

        let state_builder = self.states.get(&symbol).expect("Expected state to exist");
        match state_builder {
            HarnessStateBuilder::Primitive { mnemonic } => {
                let node = context.new_node(mnemonic)?;
                builder.states.insert(symbol, node);
                Ok(node)
            }
            
            HarnessStateBuilder::Product { .. } => Err(HarnessError::new("Product state cannot be process entry state"))
        }
    }

    fn build_state(&self, context: &mut StateMachineContext, process_set: &mut ProcessSet, template: &mut CodegenTemplate, builder: &mut HarnessBuilderState, process: ProcessID, symbol: HarnessBuilderSymbolID) -> Result<StateMachineNodeID, HarnessError> {
        let node = if let Some(&node) = builder.states.get(&symbol) {
            node
        } else {
            let state_builder = self.states.get(&symbol).expect("Expected state to exist");
            match state_builder {
                HarnessStateBuilder::Primitive { mnemonic } => {
                    let node = context.new_node(mnemonic)?;
                    builder.states.insert(symbol, node);
                    node
                }
                
                HarnessStateBuilder::Product { mnemonic: _, base_state, mapped_processes } => {
                    let base_node = self.build_state(context, process_set, template, builder, process, *base_state)?;
                    let processes = mapped_processes.iter()
                        .map(| process | *builder.processes.get(process).expect("Expected process to exist"))
                        .map(| process | Into::<StateMachineMessageParticipantID>::into(process))
                        .collect::<Vec<StateMachineMessageParticipantID>>();

                    let product_builder = StateMachineProductNodeBuilder::new(base_node, processes.len());
                    let product_node = product_builder.build(context)?;
                    let inbound_mapping = product_node.get_inbound_message_mapping(processes.clone().into_iter())?;
                    let outbound_mapping = product_node.get_outbound_message_mapping(processes.clone().into_iter())?;
                    process_set.new_inbound_message_mapping(process, inbound_mapping)?;
                    process_set.new_outbound_message_mapping(process, outbound_mapping)?;
                    builder.states.insert(symbol, product_node.get_root_node());
                    product_node.get_root_node()
                }
            }
        };

        if let Some(edges) = self.direct_edges.get(&symbol) {
            for edge in edges {
                if builder.visited_edges.contains(edge) {
                    continue;
                }
                builder.visited_edges.insert(*edge);
                let edge = self.edges.get(&edge).expect("Expected edge to exist");
                let target_node = self.build_state(context, process_set, template, builder, process, edge.target)?;
                let trigger = edge.trigger.map(| trigger | self.build_message(context, builder, trigger))
                    .map_or(Ok(None), | v | v.map(Some))?;
                let action = edge.action.map(| action | self.build_action(context, template, builder, action))
                    .map_or(Ok(None), | v | v.map(Some))?;
                context.new_edge(node, target_node, trigger, action)?;
            }
        }

        Ok(node)
    }

    fn build_message(&self, context: &mut StateMachineContext, builder: &mut HarnessBuilderState, symbol: HarnessBuilderSymbolID) -> Result<StateMachineMessageID, HarnessError> {
        if let Some(&message) = builder.messages.get(&symbol) {
            Ok(message)
        } else {
            let message_builder = self.messages.get(&symbol).expect("Expected message to exist");
            let message = context.new_message(message_builder.mnemonic.clone())?;
            builder.messages.insert(symbol, message);
            Ok(message)
        }
    }

    fn build_action(&self, context: &mut StateMachineContext, template: &mut CodegenTemplate, builder: &mut HarnessBuilderState, symbol: HarnessBuilderSymbolID) -> Result<StateMachineActionID, HarnessError> {
        if let Some(&action) = builder.actions.get(&symbol) {
            Ok(action)
        } else {
            let action_builder = self.actions.get(&symbol).expect("Expected action to exist");
            let action = context.new_action(action_builder.mnemonic.clone())?;
            builder.actions.insert(symbol, action);

            for envelope in &action_builder.envelopes {
                match envelope {
                    HarnessEnvelopeBuilder::Unicast(process_symbol, message_symbol) => {
                        let process = *builder.processes.get(process_symbol).expect("Expected process to exist");
                        let message = self.build_message(context, builder, *message_symbol)?;
                        context.add_envelope(action, StateMachineMessageDestination::Unicast(process.into()), message)?;
                    }

                    HarnessEnvelopeBuilder::Multicast(process_symbols, message_symbol) => {
                        let processess = process_symbols.iter()
                            .map(| process_symbol | (*builder.processes.get(process_symbol).expect("Expected process to exist")).into())
                            .collect();
                        let message = self.build_message(context, builder, *message_symbol)?;
                        context.add_envelope(action, StateMachineMessageDestination::Multicast(processess), message)?;
                    }

                    HarnessEnvelopeBuilder::Response(message_symbol) => {
                        let message = self.build_message(context, builder, *message_symbol)?;
                        context.add_envelope(action, StateMachineMessageDestination::Response, message)?;
                    }
                }
            }

            if let Some(content) = &action_builder.content {
                template.define_action(action, content);
            }
            Ok(action)
        }

    }
}
