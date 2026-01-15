use crate::machine_identification::{MachineIdentification, MachineIdentificationUnique};
use crate::wago_750_530_example::api::{StateEvent, WAGO_750_530_ExampleMachineEvents};
use crate::{AsyncThreadMessage, Machine, MachineMessage};
use control_core::socketio::namespace::NamespaceCacheingLogic;
use ethercat_hal::io::digital_output::DigitalOutput;
use smol::channel::{Receiver, Sender};
use std::time::Instant;
pub mod act;
pub mod api;
pub mod new;
use crate::wago_750_530_example::api::WAGO_750_530_ExampleMachineNamespace;
use crate::{TEST_MACHINE, VENDOR_QITECH};

#[derive(Debug)]
pub struct WAGO_750_530_ExampleMachine {
    pub api_receiver: Receiver<MachineMessage>,
    pub api_sender: Sender<MachineMessage>,
    pub machine_identification_unique: MachineIdentificationUnique,
    pub namespace: WAGO_750_530_ExampleMachineNamespace,
    pub last_state_emit: Instant,
    pub led_on: [bool; 4],
    pub main_sender: Option<Sender<AsyncThreadMessage>>,
    pub douts: [DigitalOutput; 4],
}

impl Machine for WAGO_750_530_ExampleMachine {
    fn get_machine_identification_unique(&self) -> MachineIdentificationUnique {
        self.machine_identification_unique.clone()
    }

    fn get_main_sender(&self) -> Option<Sender<AsyncThreadMessage>> {
        self.main_sender.clone()
    }
}

impl WAGO_750_530_ExampleMachine {
    pub const MACHINE_IDENTIFICATION: MachineIdentification = MachineIdentification {
        vendor: VENDOR_QITECH,
        machine: TEST_MACHINE,
    };
}

impl WAGO_750_530_ExampleMachine {
    pub fn emit_state(&mut self) {
        let event = StateEvent {
            led_on: self.led_on,
        }
        .build();

        self.namespace.emit(WAGO_750_530_ExampleMachineEvents::State(event));
    }

    /// Set the state of a specific LED
    pub fn set_led(&mut self, index: usize, on: bool) {
        if index < self.led_on.len() {
            self.led_on[index] = on;
            self.emit_state();
        }
    }

    /// Set all LEDs at once
    pub fn set_all_leds(&mut self, on: bool) {
        self.led_on = [on; 4];
        self.emit_state();
    }
}
