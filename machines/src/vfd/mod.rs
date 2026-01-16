use crate::AsyncThreadMessage;
use crate::{
    MACHINE_VFD, Machine, MachineMessage, VENDOR_QITECH,
    machine_identification::{MachineIdentification, MachineIdentificationUnique},
};
use api::{LiveValuesEvent, VFDEvents, VFDMachineNamespace, StateEvent, Direction, MotorState};
use control_core::socketio::event::BuildEvent;
use control_core::socketio::namespace::NamespaceCacheingLogic;
use smol::channel::{Receiver, Sender};
use std::time::Instant;
use tracing::info;

pub mod act;
pub mod api;
pub mod new;

pub mod us_3202510;

#[derive(Debug)]
pub struct VFDMachine {
    api_receiver: Receiver<MachineMessage>,
    api_sender: Sender<MachineMessage>,
    machine_identification_unique: MachineIdentificationUnique,
    main_sender: Option<Sender<AsyncThreadMessage>>,

    // socketio
    namespace: VFDMachineNamespace,
    last_measurement_emit: Instant,

    // machine specific
    direction: Direction,
    speed:     f32,

    // State tracking to only emit when values change
    last_emitted_event: Option<StateEvent>,

    /// Will be initialized as false and set to true by emit_state
    /// This way we can signal to the client that the first state emission is a default state
    emitted_default_state: bool
}

impl Machine for VFDMachine {
    fn get_machine_identification_unique(&self) -> MachineIdentificationUnique {
        self.machine_identification_unique.clone()
    }

    fn get_main_sender(&self) -> Option<Sender<AsyncThreadMessage>> {
        self.main_sender.clone()
    }
}

impl VFDMachine {
    pub const MACHINE_IDENTIFICATION: MachineIdentification = MachineIdentification {
        machine: MACHINE_VFD,
        vendor: VENDOR_QITECH,
    };

    /// Emit live values data event with the current sine wave amplitude
    pub fn emit_live_values(&mut self) {
        // let now = Instant::now();

        let live_values = LiveValuesEvent {};

        self.namespace.emit(VFDEvents::LiveValues(live_values.build()));
    }

    /// Emit the current state of the mock machine only if values have changed
    pub fn emit_state(&mut self) {
        info!(
            "Emitting state for MockMachine, is default state: {}",
            !self.emitted_default_state
        );

        let current_state = StateEvent { motor_state: MotorState {} };

        self.namespace.emit(VFDEvents::State(current_state.build()));
        self.last_emitted_event = Some(current_state);
    }

    pub fn set_speed(&mut self, speed: f32)
    {
        self.speed = speed;
        self.emit_state();
    }

    pub fn set_direction(&mut self, direction: Direction)
    {
        self.direction = direction;
        self.emit_state();
    }
}