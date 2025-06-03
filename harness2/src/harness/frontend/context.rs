use std::collections::{BTreeMap, HashMap};

use crate::harness::{codegen::template::CodegenTemplate, core::{error::HarnessError, process::{ProcessID, ProcessSet}, state_machine::{StateMachineActionID, StateMachineContext, StateMachineMessageDestination, StateMachineMessageEnvelopeBehavior, StateMachineMessageID, StateMachineMessageParticipantID, StateMachineNodeID}}, entities::product_node::{StateMachineProductNode, StateMachineProductNodeBuilder}};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct HarnessContextSymbolID(u64);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum HarnessContextSymbol {
    State(HarnessContextSymbolID),
    Edge(HarnessContextSymbolID),
    Message(HarnessContextSymbolID),
    Action(HarnessContextSymbolID),
    Process(HarnessContextSymbolID)
}

#[derive(Clone)]
enum HarnessContextStateBuilder {
    Primitive {
        mnemonic: String
    },

    Product {
        base_state: HarnessContextSymbol,
        mapped_processes: Vec<HarnessContextSymbol>
    }
}

#[derive(Clone)]
struct HarnessContextMessageBuilder {
    mnemonic: String
}

#[derive(Clone)]
struct HarnessContextEdgeBuilder {
    target: HarnessContextSymbol,
    trigger: Option<HarnessContextSymbol>,
    action: Option<HarnessContextSymbol>
}

#[derive(Clone)]
enum HarnessContextEnvelopeBuilder {
    Unicast(HarnessContextSymbol, StateMachineMessageEnvelopeBehavior, HarnessContextSymbol),
    Multicast(Vec<HarnessContextSymbol>, StateMachineMessageEnvelopeBehavior, HarnessContextSymbol),
    Response(StateMachineMessageEnvelopeBehavior, HarnessContextSymbol)
}

#[derive(Clone)]
struct HarnessContextActionBuilder {
    mnemonic: String,
    envelopes: Vec<HarnessContextEnvelopeBuilder>,
    content: Option<String>
}

#[derive(Clone)]
struct HarnessContextProcessBuilder {
    mnemonic: String,
    entry_state: HarnessContextSymbol,
    parameters: HashMap<String, String>,
    prologue: Option<String>
}

pub struct HarnessContextBuild {
    states: HashMap<HarnessContextSymbol, StateMachineNodeID>,
    messages: HashMap<HarnessContextSymbol, StateMachineMessageID>,
    actions: HashMap<HarnessContextSymbol, StateMachineActionID>,
    processes: HashMap<HarnessContextSymbol, ProcessID>,
    pending_product_mappings: Vec<(HarnessContextSymbol, StateMachineProductNode, Vec<HarnessContextSymbol>)>,
    template: CodegenTemplate
}

#[derive(Clone)]
pub struct HarnessContext {
    next_symbol: u64,
    named_symbols: HashMap<String, HarnessContextSymbol>,
    states: HashMap<HarnessContextSymbol, HarnessContextStateBuilder>,
    messages: HashMap<HarnessContextSymbol, HarnessContextMessageBuilder>,
    edges: HashMap<HarnessContextSymbol, HarnessContextEdgeBuilder>,
    direct_edges: HashMap<HarnessContextSymbol, Vec<HarnessContextSymbol>>,
    actions: HashMap<HarnessContextSymbol, HarnessContextActionBuilder>,
    processes: BTreeMap<HarnessContextSymbol, HarnessContextProcessBuilder>,
    global_prologue: Option<String>,
    executable: bool
}

impl HarnessContextBuild {
    pub fn get_template(&self) -> &CodegenTemplate {
        &self.template
    }

    pub fn get_state(&self, symbol: HarnessContextSymbol) -> Option<StateMachineNodeID> {
        self.states.get(&symbol).map(| x | *x)
    }

    pub fn get_message(&self, symbol: HarnessContextSymbol) -> Option<StateMachineMessageID> {
        self.messages.get(&symbol).map(| x | *x)
    }

    pub fn get_action(&self, symbol: HarnessContextSymbol) -> Option<StateMachineActionID> {
        self.actions.get(&symbol).map(| x | *x)
    }

    pub fn get_process(&self, symbol: HarnessContextSymbol) -> Option<ProcessID> {
        self.processes.get(&symbol).map(| x | *x)
    }
}

impl HarnessContext {
    pub fn new() -> HarnessContext {
        HarnessContext {
            next_symbol: 0,
            named_symbols: HashMap::new(),
            states: HashMap::new(),
            messages: HashMap::new(),
            edges: HashMap::new(),
            direct_edges: HashMap::new(),
            actions: HashMap::new(),
            processes: BTreeMap::new(),
            global_prologue: None,
            executable: false
        }
    }

    pub fn set_executable(&mut self, executable: bool) {
        self.executable = executable;
    }

    pub fn is_executable(&self) -> bool {
        self.executable
    }

    pub fn new_primitive_state(&mut self, mnemonic: &str) -> Result<HarnessContextSymbol, HarnessError> {
        let symbol = self.new_named_state_symbol(mnemonic)?;
        self.states.entry(symbol)
            .or_insert(HarnessContextStateBuilder::Primitive { mnemonic: mnemonic.into() });
        Ok(symbol)
    }

    pub fn new_product_state(&mut self, mnemonic: &str, base_state: HarnessContextSymbol, mapped_processes: impl Iterator<Item = HarnessContextSymbol>) -> Result<HarnessContextSymbol, HarnessError> {
        let symbol = self.new_named_state_symbol(mnemonic)?;
        if !self.states.contains_key(&symbol) {
            if !self.states.contains_key(&base_state) {
                return Err(HarnessError::new("Unknown product state base state symbol"));
            }
            let mapped_processes = mapped_processes.collect::<Vec<_>>();
            if mapped_processes.iter().any(| process | !self.processes.contains_key(&process)) {
                return Err(HarnessError::new("Unknown product state mapped process symbol"));
            }

            self.states.insert(symbol, HarnessContextStateBuilder::Product { base_state, mapped_processes });
        }
        
        Ok(symbol)
    }

    pub fn new_message(&mut self, mnemonic: &str) -> Result<HarnessContextSymbol, HarnessError> {
        let symbol = self.new_named_message_symbol(mnemonic)?;
        self.messages.entry(symbol)
            .or_insert(HarnessContextMessageBuilder { mnemonic: mnemonic.into() });
        Ok(symbol)
    }

    pub fn new_action(&mut self, mnemonic: &str) -> Result<HarnessContextSymbol, HarnessError> {
        let symbol = self.new_named_action_symbol(mnemonic)?;
        self.actions.entry(symbol)
            .or_insert(HarnessContextActionBuilder { mnemonic: mnemonic.into(), envelopes: Vec::new(), content: None });
        Ok(symbol)
    }

    pub fn new_process(&mut self, mnemonic: &str, entry_state: HarnessContextSymbol) -> Result<HarnessContextSymbol, HarnessError> {
        if !self.states.contains_key(&entry_state) {
            return Err(HarnessError::new("Unknown process entry state symbol"));
        }
        let symbol = self.new_named_process_symbol(mnemonic)?;
        self.processes.entry(symbol)
            .or_insert(HarnessContextProcessBuilder { mnemonic: mnemonic.into(), entry_state, parameters: HashMap::new(), prologue: None });
        Ok(symbol)
    }

    pub fn new_edge(&mut self, source: HarnessContextSymbol, target: HarnessContextSymbol, trigger: Option<HarnessContextSymbol>, action: Option<HarnessContextSymbol>) -> Result<HarnessContextSymbol, HarnessError> {
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

        let symbol = HarnessContextSymbol::Edge(self.new_symbol_id());
        self.edges.insert(symbol, HarnessContextEdgeBuilder { target, trigger, action });
        self.direct_edges.entry(source)
            .or_insert(Vec::new())
            .push(symbol);
        Ok(symbol)
    }

    pub fn append_global_prologue(&mut self, prologue: String) {
        match &mut self.global_prologue {
            Some(head) => {
                head.push_str(&prologue);
            }

            None => self.global_prologue = Some(prologue)
        }
    }

    pub fn append_process_prologue(&mut self, process: HarnessContextSymbol, prologue: String) -> Result<(), HarnessError> {
        let process_builder = self.processes.get_mut(&process)
            .ok_or(HarnessError::new("Unknown process symbol"))?;
        match &mut process_builder.prologue {
            Some(head) => {
                head.push_str(&prologue);
            }

            None => process_builder.prologue = Some(prologue)
        }
        Ok(())
    }

    pub fn set_process_parameter(&mut self, process: HarnessContextSymbol, key: String, value: String) -> Result<(), HarnessError> {
        self.processes.get_mut(&process)
            .ok_or(HarnessError::new("Unknown process symbol"))?
            .parameters.insert(key, value);
        Ok(())
    }

    pub fn set_action_content(&mut self, action: HarnessContextSymbol, content: String) -> Result<(), HarnessError> {
        let action = match self.actions.get_mut(&action) {
            Some(action) => action,
            None => return Err(HarnessError::new("Unknown action symbol"))
        };
        action.content = Some(content);
        Ok(())
    }

    pub fn new_unicast_envelope(&mut self, action: HarnessContextSymbol, destination: HarnessContextSymbol, behavior: StateMachineMessageEnvelopeBehavior, message: HarnessContextSymbol) -> Result<(), HarnessError> {
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

        action.envelopes.push(HarnessContextEnvelopeBuilder::Unicast(destination, behavior, message));
        Ok(())
    }

    pub fn new_multicast_envelope(&mut self, action: HarnessContextSymbol, destinations: impl Iterator<Item = HarnessContextSymbol>, behavior: StateMachineMessageEnvelopeBehavior, message: HarnessContextSymbol) -> Result<(), HarnessError> {
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

        action.envelopes.push(HarnessContextEnvelopeBuilder::Multicast(destinations, behavior, message));
        Ok(())
    }

    pub fn new_response_envelope(&mut self, action: HarnessContextSymbol, behavior: StateMachineMessageEnvelopeBehavior, message: HarnessContextSymbol) -> Result<(), HarnessError> {
        let action = match self.actions.get_mut(&action) {
            Some(action) => action,
            None => return Err(HarnessError::new("Unknown action symbol"))
        };
        if !self.messages.contains_key(&message) {
            return Err(HarnessError::new("Unknown envelope message symbol"));
        }

        action.envelopes.push(HarnessContextEnvelopeBuilder::Response(behavior, message));
        Ok(())
    }

    fn new_symbol_id(&mut self) -> HarnessContextSymbolID {
        let symbol = HarnessContextSymbolID(self.next_symbol);
        self.next_symbol += 1;
        symbol
    }

    fn new_named_state_symbol(&mut self, name: &str) -> Result<HarnessContextSymbol, HarnessError> {
        let existing_symbol = self.named_symbols.get(name);
        if let Some(HarnessContextSymbol::State(_)) = existing_symbol {
            Ok(*existing_symbol.unwrap())
        } else if let Some(_) = existing_symbol {
            Err(HarnessError::new("Another identically named symbol with mismatching type already exists"))
        } else {
            let symbol = HarnessContextSymbol::State(self.new_symbol_id());
            self.named_symbols.insert(name.into(), symbol);
            Ok(symbol)
        }
    }

    fn new_named_message_symbol(&mut self, name: &str) -> Result<HarnessContextSymbol, HarnessError> {
        let existing_symbol = self.named_symbols.get(name);
        if let Some(HarnessContextSymbol::Message(_)) = existing_symbol {
            Ok(*existing_symbol.unwrap())
        } else if let Some(_) = existing_symbol {
            Err(HarnessError::new("Another identically named symbol with mismatching type already exists"))
        } else {
            let symbol = HarnessContextSymbol::Message(self.new_symbol_id());
            self.named_symbols.insert(name.into(), symbol);
            Ok(symbol)
        }
    }

    fn new_named_action_symbol(&mut self, name: &str) -> Result<HarnessContextSymbol, HarnessError> {
        let existing_symbol = self.named_symbols.get(name);
        if let Some(HarnessContextSymbol::Action(_)) = existing_symbol {
            Ok(*existing_symbol.unwrap())
        } else if let Some(_) = existing_symbol {
            Err(HarnessError::new("Another identically named symbol with mismatching type already exists"))
        } else {
            let symbol = HarnessContextSymbol::Action(self.new_symbol_id());
            self.named_symbols.insert(name.into(), symbol);
            Ok(symbol)
        }
    }

    fn new_named_process_symbol(&mut self, name: &str) -> Result<HarnessContextSymbol, HarnessError> {
        let existing_symbol = self.named_symbols.get(name);
        if let Some(HarnessContextSymbol::Process(_)) = existing_symbol {
            Ok(*existing_symbol.unwrap())
        } else if let Some(_) = existing_symbol {
            Err(HarnessError::new("Another identically named symbol with mismatching type already exists"))
        } else {
            let symbol = HarnessContextSymbol::Process(self.new_symbol_id());
            self.named_symbols.insert(name.into(), symbol);
            Ok(symbol)
        }
    }

    pub fn build(&self, context: &mut StateMachineContext, process_set: &mut ProcessSet) -> Result<HarnessContextBuild, HarnessError> {
        let mut build = HarnessContextBuild {
            states: HashMap::new(),
            messages: HashMap::new(),
            actions: HashMap::new(),
            processes: HashMap::new(),
            pending_product_mappings: Vec::new(),
            template: CodegenTemplate::new()
        };

        self.build_processes(context, process_set, &mut build)?;
        self.build_envelopes(context, &mut build)?;
        self.build_product_mappings(process_set, &mut build)?;

        if let Some(prologue) = &self.global_prologue {
            build.template.set_global_prologue(Some(prologue));
        }

        Ok(build)
    }

    fn build_state(&self, context: &mut StateMachineContext, process_set: &mut ProcessSet, build: &mut HarnessContextBuild, process: HarnessContextSymbol, symbol: HarnessContextSymbol) -> Result<StateMachineNodeID, HarnessError> {
        let node = if let Some(&node) = build.states.get(&symbol) {
            return Ok(node);
        } else {
            let state_builder = self.states.get(&symbol).expect("Expected state to exist");
            match state_builder {
                HarnessContextStateBuilder::Primitive { mnemonic } => {
                    let node = context.new_node(mnemonic)?;
                    build.states.insert(symbol, node);
                    node
                }
                
                HarnessContextStateBuilder::Product { base_state, mapped_processes } => {
                    let base_node = self.build_state(context, process_set, build, process, *base_state)?;
                    let product_builder = StateMachineProductNodeBuilder::new(base_node, mapped_processes.len());
                    let product_node = product_builder.build(context)?;
                    let node = product_node.get_root_node();
                    build.pending_product_mappings.push((process, product_node, mapped_processes.clone()));
                    build.states.insert(symbol, node);
                    node
                }
            }
        };

        if let Some(edges) = self.direct_edges.get(&symbol) {
            for edge in edges {
                let edge = self.edges.get(&edge).expect("Expected edge to exist");
                let target_node = self.build_state(context, process_set, build, process, edge.target)?;
                let trigger = edge.trigger.map(| trigger | self.build_message(context, build, trigger))
                    .map_or(Ok(None), | v | v.map(Some))?;
                let action = edge.action.map(| action | self.build_action(context, build, action))
                    .map_or(Ok(None), | v | v.map(Some))?;
                context.new_edge(node, target_node, trigger, action)?;
            }
        }

        Ok(node)
    }

    fn build_message(&self, context: &mut StateMachineContext, build: &mut HarnessContextBuild, symbol: HarnessContextSymbol) -> Result<StateMachineMessageID, HarnessError> {
        if let Some(&message) = build.messages.get(&symbol) {
            Ok(message)
        } else {
            let message_builder = self.messages.get(&symbol).expect("Expected message to exist");
            let message = context.new_message(message_builder.mnemonic.clone())?;
            build.messages.insert(symbol, message);
            Ok(message)
        }
    }

    fn build_action(&self, context: &mut StateMachineContext, build: &mut HarnessContextBuild, symbol: HarnessContextSymbol) -> Result<StateMachineActionID, HarnessError> {
        if let Some(&action) = build.actions.get(&symbol) {
            Ok(action)
        } else {
            let action_builder = self.actions.get(&symbol).expect("Expected action to exist");
            let action = context.new_action(action_builder.mnemonic.clone())?;
            build.actions.insert(symbol, action);


            if let Some(content) = &action_builder.content {
                build.template.define_action(action, content);
            }
            Ok(action)
        }
    }

    fn build_processes(&self, context: &mut StateMachineContext, process_set: &mut ProcessSet, build: &mut HarnessContextBuild) -> Result<(), HarnessError> {
        for (&symbol, process_builder) in &self.processes {
            let entry_node = self.build_state(context, process_set, build, symbol, process_builder.entry_state)?;
            let process = process_set.new_process(process_builder.mnemonic.clone(), entry_node);
            if let Some(prologue) = &process_builder.prologue {
                build.template.set_process_prologue(process, prologue);
            }
            for (key, value) in &process_builder.parameters {
                build.template.set_process_parameter(process, &key, value);
            }
            build.processes.insert(symbol, process);
        }

        Ok(())        
    }

    fn build_product_mappings(&self, process_set: &mut ProcessSet, build: &mut HarnessContextBuild) -> Result<(), HarnessError> {
        for (process, product_node, mapped_processes) in &build.pending_product_mappings {
            let process = *build.processes.get(&process).expect("Expected process to exist");
            let processes = mapped_processes.iter()
                .map(| process | *build.processes.get(process).expect("Expected process to exist"))
                .map(| process | Into::<StateMachineMessageParticipantID>::into(process))
                .collect::<Vec<StateMachineMessageParticipantID>>();
            let inbound_mapping = product_node.get_inbound_message_mapping(processes.clone().into_iter())?;
            let outbound_mapping = product_node.get_outbound_message_mapping(processes.into_iter())?;
            process_set.new_inbound_message_mapping(process, inbound_mapping)?;
            process_set.new_outbound_message_mapping(process, outbound_mapping)?;
        }
        build.pending_product_mappings.clear();
        Ok(())
    }

    fn build_envelopes(&self, context: &mut StateMachineContext, build: &mut HarnessContextBuild) -> Result<(), HarnessError> {
        let actions = build.actions.iter()
            .map(| (symbol, action) | (*action, self.actions.get(&symbol).expect("Expected action to exist")))
            .collect::<Vec<_>>();

        for (action, action_builder) in actions {
            for envelope in &action_builder.envelopes {
                match envelope {
                    HarnessContextEnvelopeBuilder::Unicast(process_symbol, behavior, message_symbol) => {
                        let process = *build.processes.get(process_symbol).expect("Expected process to exist");
                        let message = self.build_message(context, build, *message_symbol)?;
                        context.add_envelope(action, StateMachineMessageDestination::Unicast(process.into()), *behavior, message)?;
                    }
    
                    HarnessContextEnvelopeBuilder::Multicast(process_symbols, behavior, message_symbol) => {
                        let processess = process_symbols.iter()
                            .map(| process_symbol | (*build.processes.get(process_symbol).expect("Expected process to exist")).into())
                            .collect();
                        let message = self.build_message(context, build, *message_symbol)?;
                        context.add_envelope(action, StateMachineMessageDestination::Multicast(processess), *behavior, message)?;
                    }
    
                    HarnessContextEnvelopeBuilder::Response(behavior, message_symbol) => {
                        let message = self.build_message(context, build, *message_symbol)?;
                        context.add_envelope(action, StateMachineMessageDestination::Response, *behavior, message)?;
                    }
                }
            }
        }

        Ok(())
    }
}
