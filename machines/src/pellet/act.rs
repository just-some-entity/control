use super::PelletMachine;
use crate::{MachineAct, MachineMessage};
use std::time::{Duration, Instant};

impl MachineAct for PelletMachine {
    fn act(&mut self, now: Instant) {
        let msg = self.api_receiver.try_recv();
        match msg {
            Ok(msg) => { self.act_machine_message(msg); }
            Err(_)  => (),
        };

        let target_ups: Duration = Duration::from_secs_f64(1.0 / 30.0);
        if now.duration_since(self.last_measurement_emit) > target_ups {
            self.emit_live_values();
            self.last_measurement_emit = now;
        }
    }

    fn act_machine_message(&mut self, msg: MachineMessage) {
        match msg {
            MachineMessage::SubscribeNamespace(namespace) => {
                self.namespace.namespace = Some(namespace);
                self.emit_state();
            }
            
            MachineMessage::UnsubscribeNamespace => {
                match &mut self.namespace.namespace {
                    Some(namespace) => {
                        tracing::info!("UnsubscribeNamespace");
                        namespace.socket_queue_tx.close();
                        namespace.sockets.clear();
                        namespace.events.clear();
                    }
                    None => todo!(),
                }
                
                self.namespace.namespace = None;
            },
            MachineMessage::HttpApiJsonRequest(value) => {
                use crate::MachineApi;
                let _res = self.api_mutate(value);
            }
            MachineMessage::ConnectToMachine(_machine_connection)  => (),
            MachineMessage::DisconnectMachine(_machine_connection) => ()
        }
    }
}
