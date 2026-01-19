use serde::{Deserialize, Serialize};

#[cfg(not(feature = "mock-machine"))]
use crate::{
    MACHINE_PELLET, MachineMessage, VENDOR_QITECH,
    extruder1::{
        api::ExtruderV2Namespace, 
    },
    machine_identification::{MachineIdentification, MachineIdentificationUnique},
};

use control_core::socketio::namespace::NamespaceCacheingLogic;
use smol::{
    channel::{Receiver, Sender},
    lock::RwLock,
};
use socketioxide::extract::SocketRef;
use units::Length;

use crate::{AsyncThreadMessage, pellet_machine::api::{LiveValuesEvent, MotorState, RunState, StateEvent}};

use std::{sync::Arc, time::Instant};

pub mod act;
pub mod api;
pub mod new;
pub mod emit;

use crate::Machine;

use api::{PelletMachineEvents, PelletMachineNamespace};

#[derive(Debug)]
pub struct PelletMachine 
{
    // stuff for every machine
    api_receiver: Receiver<MachineMessage>,
    api_sender: Sender<MachineMessage>,
    main_sender: Option<Sender<AsyncThreadMessage>>,
    machine_identification_unique: MachineIdentificationUnique,

    emitted_default_state: bool,

    // socketio
    namespace: PelletMachineNamespace,
    last_measurement_emit: Instant,

    // machine specific
    run_state: RunState,

    voltage: f64,

    current: f64,

    temperature: f64,

    system_status: f64,

    error_code: u64,

    frequency_target: f64,
    
    frequency_actual: f64,

    acceleration_time_step: f64,

    deceleration_time_step: f64,
}

impl Machine for PelletMachine
{
    fn get_machine_identification_unique(&self) -> MachineIdentificationUnique {
        self.machine_identification_unique.clone()
    }

    fn get_main_sender(&self) -> Option<Sender<AsyncThreadMessage>> {
        self.main_sender.clone()
    }
}

impl PelletMachineNamespace {
    pub async fn disconnect_all(&self) {
        for socket in self.connected_sockets().await {
            let _ = socket.disconnect();
        }
    }

    async fn connected_sockets(&self) -> Vec<SocketRef> {
        if self.namespace.is_none() {
            return vec![];
        }
        let sockets = self.namespace.clone().unwrap().sockets.clone();
        sockets
    }
}

// idk?
impl Drop for PelletMachine {
    fn drop(&mut self) {
        tracing::info!(
            "[LaserMachine::{:?}] Dropping machine and disconnecting clients...",
            self.machine_identification_unique
        );
        smol::block_on(self.namespace.disconnect_all());
    }
}

impl PelletMachine
{
    pub const MACHINE_IDENTIFICATION: MachineIdentification = MachineIdentification {
        vendor: VENDOR_QITECH,
        machine: MACHINE_PELLET,
    };

    pub fn set_frequency(&mut self, frequency: f64)
    {
        //...
    }
}