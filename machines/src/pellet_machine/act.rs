use std::time::{Duration, Instant};

use crate::MachineAct;
use crate::MachineMessage;
use crate::MachineValues;

use super::PelletMachine;

impl MachineAct for PelletMachine 
{
    fn act(&mut self, now: Instant) 
    {
        //tracing::error!("ACT RECEIVED");
        
        if let Ok(msg) = self.api_receiver.try_recv() 
        {
            self.act_machine_message(msg);
        };

        // self.update();

        // if self.did_change_state 
        // {
        //     self.emit_state();
        // }

        // more than 33ms have passed since last emit (30 "fps" target)
        if now.duration_since(self.last_measurement_emit) > Duration::from_secs_f64(1.0 / 30.0) 
        {
            self.emit_live_values();
            self.last_measurement_emit = now;
        }
    }

    fn act_machine_message(&mut self, msg: MachineMessage) 
    {
        match msg 
        {
            MachineMessage::SubscribeNamespace(namespace) => 
            {
                self.namespace.namespace = Some(namespace);
                self.emit_state();
            }
            
            MachineMessage::UnsubscribeNamespace => match &mut self.namespace.namespace 
            {
                Some(namespace) => 
                {
                    tracing::info!("UnsubscribeNamespace");
                    namespace.socket_queue_tx.close();
                    namespace.sockets.clear();
                    namespace.events.clear();
                }
                None => todo!(),
            },
            MachineMessage::HttpApiJsonRequest(value) => 
            {
                use crate::MachineApi;
                let _res = self.api_mutate(value);
            }
            MachineMessage::ConnectToMachine(_machine_connection)  => {}
            MachineMessage::DisconnectMachine(_machine_connection) => {}
            MachineMessage::RequestValues(sender) => 
            {
                tracing::error!("REQUESTED VALUES");
                
                let state = serde_json::to_value(self.create_state_event()).expect("Failed to serialize state");
                
                let live_values = serde_json::to_value(self.create_live_values_event()).expect("Failed to serialize live values");

                sender.send_blocking(MachineValues{ state, live_values }).expect("Failed to send values");
                sender.close();
            }
        }
    }
}