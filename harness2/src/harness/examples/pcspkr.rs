use crate::harness::{codegen::{codegen::ControlFlowCodegen, executable::ControlFlowExecutableCodegen, goblint::ControlFlowGoblintCodegen, output::CodegenOutput, template::CodegenTemplate}, core::{error::HarnessError, process::{ProcessID, ProcessSet}, state_machine::{StateMachineActionID, StateMachineContext, StateMachineNodeID}}};

use super::base::HarnessExample;

pub struct PcspkrExample {
    num_of_pcspkr_drivers: usize,
    num_of_i8253_clients: usize
}

#[allow(dead_code)]
pub struct PcspkrModel {
    pcspkr_driver_probe_action: StateMachineActionID,
    pcspkr_driver_send_event_action: StateMachineActionID,
    pcspkr_driver_suspend_action: StateMachineActionID,
    pcspkr_driver_shutdown_action: StateMachineActionID,
    i8253_client_use_action: StateMachineActionID,

    i8253_client_active_state: StateMachineNodeID,

    pcspkr_driver_unloaded_state: StateMachineNodeID,
    pcspkr_driver_active_state: StateMachineNodeID,
    pcspkr_driver_suspended_state: StateMachineNodeID,

    pcspkr_drivers: Vec<ProcessID>,
    i8253_clients: Vec<ProcessID>
}

impl PcspkrExample {
    pub fn new(num_of_pcspkr_drivers: usize, num_of_i8253_clients: usize) -> PcspkrExample {
        PcspkrExample { num_of_pcspkr_drivers, num_of_i8253_clients }
    }
}

impl HarnessExample for PcspkrExample {
    type Model = PcspkrModel;

    fn build_model(&self, context: &mut StateMachineContext, process_set: &mut ProcessSet) -> Result<Self::Model, HarnessError> {
        let pcspkr_driver_probe_action = context.new_action("pcspkr_driver_probe")?;
        let pcspkr_driver_send_event_action = context.new_action("pcspkr_driver_send_event")?;
        let pcspkr_driver_suspend_action = context.new_action("pcspkr_driver_suspend")?;
        let pcspkr_driver_shutdown_action = context.new_action("pcspkr_driver_shutdown")?;
        let i8253_client_use_action = context.new_action("i8253_client_use")?;

        let i8253_client_active_state = context.new_node("i8253_client_active")?;

        let pcspkr_driver_unloaded_state = context.new_node("pcspkr_driver_unloaded")?;
        let pcspkr_driver_active_state = context.new_node("pcspkr_driver_active")?;
        let pcspkr_driver_suspended_state = context.new_node("pcspkr_driver_suspended")?;
        
        let i8253_clients = (0..self.num_of_i8253_clients).map(| client_id |
            process_set.new_process(format!("i8253_client{}", client_id), i8253_client_active_state))
            .collect::<Vec<_>>();
        let pcspkr_drivers = (0..self.num_of_pcspkr_drivers).map(| driver_id |
            process_set.new_process(format!("pcspkr_driver{}", driver_id), pcspkr_driver_unloaded_state))
            .collect::<Vec<_>>();

        context.new_edge(pcspkr_driver_unloaded_state, pcspkr_driver_unloaded_state, None, None)?;
        context.new_edge(pcspkr_driver_unloaded_state, pcspkr_driver_active_state, None, Some(pcspkr_driver_probe_action))?;
        context.new_edge(pcspkr_driver_active_state, pcspkr_driver_active_state, None, Some(pcspkr_driver_send_event_action))?;
        context.new_edge(pcspkr_driver_active_state, pcspkr_driver_unloaded_state, None, Some(pcspkr_driver_shutdown_action))?;
        context.new_edge(pcspkr_driver_active_state, pcspkr_driver_suspended_state, None, Some(pcspkr_driver_suspend_action))?;
        context.new_edge(pcspkr_driver_suspended_state, pcspkr_driver_suspended_state, None, None)?;
        context.new_edge(pcspkr_driver_suspended_state, pcspkr_driver_active_state, None, None)?;
        context.new_edge(pcspkr_driver_suspended_state, pcspkr_driver_unloaded_state, None, Some(pcspkr_driver_shutdown_action))?;
        context.new_edge(i8253_client_active_state, i8253_client_active_state, None, Some(i8253_client_use_action))?;

        Ok(Self::Model {
            pcspkr_driver_probe_action,
            pcspkr_driver_send_event_action,
            pcspkr_driver_suspend_action,
            pcspkr_driver_shutdown_action,
            i8253_client_use_action,
        
            i8253_client_active_state,
        
            pcspkr_driver_unloaded_state,
            pcspkr_driver_active_state,
            pcspkr_driver_suspended_state,
        
            pcspkr_drivers,
            i8253_clients
        })
    }

    fn executable_codegen<Output: CodegenOutput>(&self, model: &Self::Model) -> Result<(CodegenTemplate, impl ControlFlowCodegen<Output>), HarnessError> {
        let mut template = CodegenTemplate::new();
        template.set_global_prologue(Some(r#"
#include <stdlib.h>
#include <stdio.h>
#include <pthread.h>

_Atomic unsigned int activity = 0;
"#)).define_action(model.pcspkr_driver_send_event_action, "activity++;")
    .define_action(model.i8253_client_use_action, "activity++;");

        let codegen = ControlFlowExecutableCodegen::new();
        Ok((template, codegen))
    }

    fn goblint_codegen<Output: CodegenOutput>(&self, model: &Self::Model) -> Result<(CodegenTemplate, impl ControlFlowCodegen<Output>), HarnessError> {
        let mut template = CodegenTemplate::new();
        template.set_global_prologue(Some(r#"
#include <linux/kernel.h>
#include <linux/module.h>
#include <linux/i8253.h>
#include <linux/input.h>
#include <linux/platform_device.h>
#include <linux/timex.h>
#include <linux/io.h>

extern struct platform_driver pcspkr_platform_driver;
"#));
        for driver in &model.pcspkr_drivers {
            template.set_process_prologue(*driver, r#"
struct platform_device platform_dev;
"#);
        }
        template
            .define_action(model.pcspkr_driver_probe_action, "pcspkr_platform_driver.probe(&platform_dev);")
            .define_action(model.pcspkr_driver_send_event_action, "((struct input_dev *) platform_dev.dev.driver_data)->event(((struct input_dev *) platform_dev.dev.driver_data), EV_SND, SND_BELL, 1000);")
            .define_action(model.pcspkr_driver_shutdown_action, "pcspkr_platform_driver.shutdown(&platform_dev);")
            .define_action(model.i8253_client_use_action, r#"
unsigned long flags;
raw_spin_lock_irqsave(&i8253_lock, flags);
outb(inb_p(0x61) & 0xFC, 0x61);
raw_spin_unlock_irqrestore(&i8253_lock, flags);
"#);
        let codegen = ControlFlowGoblintCodegen::new();
        Ok((template, codegen))
    }
}