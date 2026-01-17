use crate::{
    AsyncThreadMessage,
    MACHINE_PELLET, 
    Machine, 
    MachineMessage, 
    VENDOR_QITECH,
    machine_identification::{ 
        MachineIdentification, 
        MachineIdentificationUnique 
    },
};

use api::{
    LiveValuesEvent, 
    PelletEvents, 
    PelletMachineNamespace, 
    StateEvent, 
    MotorState,
    RunState,
};

use control_core::socketio::{
    event::BuildEvent,
    namespace::NamespaceCacheingLogic
};

use smol::channel::{Receiver, Sender};

use std::time::Instant;

use tracing::info;

pub mod act;
pub mod api;
pub mod new;

#[derive(Debug)]
pub struct PelletMachine {
    // TODO: move into something like machine_base or something?
    api_receiver: Receiver<MachineMessage>,
    api_sender: Sender<MachineMessage>,
    machine_identification_unique: MachineIdentificationUnique,
    main_sender: Option<Sender<AsyncThreadMessage>>,

    // TODO: move into something like socketio_base or something?
    // socketio
    namespace: PelletMachineNamespace,
    last_measurement_emit: Instant,

    // machine specific
    run_state: RunState,
    speed: f32,
    acceleration_time: u8,
    deceleration_time: u8,

    // TODO: move into other shared struct perhaps?

    // State tracking to only emit when values change
    last_emitted_event: Option<StateEvent>,

    /// Will be initialized as false and set to true by emit_state
    /// This way we can signal to the client that the first state emission is a default state
    emitted_default_state: bool
}

impl Machine for PelletMachine {
    fn get_machine_identification_unique(&self) -> MachineIdentificationUnique {
        self.machine_identification_unique.clone()
    }

    fn get_main_sender(&self) -> Option<Sender<AsyncThreadMessage>> {
        self.main_sender.clone()
    }
}

impl PelletMachine {
    pub const MACHINE_IDENTIFICATION: MachineIdentification = MachineIdentification {
        machine: MACHINE_PELLET,
        vendor: VENDOR_QITECH,
    };

    pub fn emit_live_values(&mut self) {
        let live_values = LiveValuesEvent {};
        self.namespace.emit(PelletEvents::LiveValues(live_values.build()));
    }

    /// Emit the current state of the mock machine only if values have changed
    pub fn emit_state(&mut self) {
        info!(
            "Emitting state for PelletMachine, is default state: {}",
            !self.emitted_default_state
        );
        
        let state = StateEvent {
            is_default_state: !std::mem::replace(&mut self.emitted_default_state, true),
            motor_state: MotorState {},
        };

        self.namespace.emit(PelletEvents::State(state.build()));
        self.last_emitted_event = Some(state);
    }

    pub fn set_speed(&mut self, speed: f32)
    {
        self.speed = speed;
        self.emit_state();
    }
    
    pub fn set_run_state(&mut self, run_state: RunState)
    {
        self.run_state = run_state;
        self.emit_state();
    }

    pub fn set_acceleration_time(&mut self, acceleration_time: u8)
    {
        self.acceleration_time = acceleration_time;
        self.emit_state();
    }
    
    pub fn set_deceleration_time(&mut self, deceleration_time: u8)
    {
        self.deceleration_time = deceleration_time;
        self.emit_state();
    }
}