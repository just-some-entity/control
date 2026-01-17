use crate::{MachineApi, MachineMessage};

use super::PelletMachine;
use control_core::socketio::{
    event::{Event, GenericEvent},
    namespace::{
        CacheFn, CacheableEvents, Namespace, NamespaceCacheingLogic, cache_first_and_last_event,
    },
};
use control_core_derive::BuildEvent;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use smol::channel::Sender;
use std::sync::Arc;
use tracing::instrument;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub enum RunState {
    Stopped,
    Forward,
    Reverse,
}

#[derive(Serialize, Debug, Clone)]
pub struct LiveValuesEvent { }

impl LiveValuesEvent {
    pub fn build(&self) -> Event<Self> {
        Event::new("LiveValuesEvent", self.clone())
    }
}

#[derive(Serialize, Debug, Clone, PartialEq, BuildEvent)]
pub struct StateEvent {
    pub is_default_state: bool,
    
    pub motor_state: MotorState,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct MotorState { }

pub enum PelletEvents {
    LiveValues(Event<LiveValuesEvent>),
    State(Event<StateEvent>),
}

#[derive(Debug)]
pub struct PelletMachineNamespace {
    pub namespace: Option<Namespace>,
}

#[derive(Deserialize, Serialize)]
/// Mutation for controlling the Pellet machine
enum Mutation {
    SetRunState(RunState),
    SetSpeed(f32),
    SetAccelerationTime(u8),
    SetDecelerationTime(u8),
}

impl MachineApi for PelletMachine {
    fn api_get_sender(&self) -> Sender<MachineMessage> {
        self.api_sender.clone()
    }

    fn api_mutate(&mut self, request_body: Value) -> Result<(), anyhow::Error> {
        let mutation: Mutation = serde_json::from_value(request_body)?;
        match mutation {
            
            Mutation::SetRunState(state) => {
                self.set_run_state(state);
            },
            
            Mutation::SetDirection(reverse) => {
                self.set_direction(if reverse { Direction::Reverse } else { Direction::Forward });
            }
            Mutation::SetSpeed(speed) => {
                self.set_speed(speed);
            }
        }
        Ok(())
    }

    fn api_event_namespace(&mut self) -> Option<Namespace> {
        self.namespace.namespace.clone()
    }
}

//TODO; rename NamespaceCacheingLogic to NamespaceCachingLogic
impl NamespaceCacheingLogic<PelletEvents> for PelletMachineNamespace {
    #[instrument(skip_all)]
    fn emit(&mut self, events: PelletEvents) {
        let event = Arc::new(events.event_value());
        let buffer_fn = events.event_cache_fn();

        match &mut self.namespace {
            Some(ns) => ns.emit(event, &buffer_fn),
            None => (),
        }
    }
}

impl CacheableEvents<Self> for PelletEvents {
    fn event_value(&self) -> GenericEvent {
        match self {
            Self::LiveValues(event) => event.into(),
            Self::State(event) => event.into(),
        }
    }

    fn event_cache_fn(&self) -> CacheFn {
        let cache_first_and_last = cache_first_and_last_event();

        match self {
            Self::LiveValues(_) => cache_first_and_last,
            Self::State(_) => cache_first_and_last,
        }
    }
}