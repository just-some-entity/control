
use crate::pellet_machine::PelletMachine;

use crate::pellet_machine::api::{LiveValuesEvent, MotorState, PelletMachineEvents, StateEvent};

use control_core::helpers::hasher_serializer::hash_with_serde_model;

use control_core::socketio::event::BuildEvent;

use control_core::socketio::namespace::NamespaceCacheingLogic;

use units::angular_velocity::AngularVelocity;

use units::pressure::{Pressure, bar};

use units::thermodynamic_temperature::ThermodynamicTemperature;

use units::{angular_velocity::revolution_per_minute, thermodynamic_temperature::degree_celsius};


impl PelletMachine
{
    pub fn emit_state(&mut self) 
    {
        let event = self.create_live_values_event().build();
        self.namespace.emit(PelletMachineEvents::State(event));
        self.emitted_default_state = true;
    }

    pub fn emit_live_values(&mut self)
    {
        let event = self.create_live_values_event().build();
        self.namespace.emit(PelletMachineEvents::LiveValues(event));
    }

    pub fn create_state_event(&self) -> StateEvent 
    {
        StateEvent 
        {
            is_default_state: !self.emitted_default_state,
            motor_state: MotorState {
                run_state: self.run_state,
                frequency: self.frequency_target,
                acceleration_time: self.acceleration_time_step,
                deceleration_time: self.deceleration_time_step,
            },
        }
    }

    pub fn create_live_values_event(&mut self) -> LiveValuesEvent
    {
        LiveValuesEvent {
            run_state: self.run_state,
            voltage: self.voltage,
            current: self.current,
            temperature: self.temperature,
            system_status: self.system_status,
            error_code: self.error_code,
            frequency: self.frequency_actual,
            acceleration_time: self.acceleration_time_step,
            deceleration_time: self.acceleration_time_step,
        }
    }
}