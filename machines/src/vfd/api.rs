use crate::{MachineApi, MachineMessage};

use super::VFDMachine;
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

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub enum Direction {
    Forward,
    Reverse,
}

#[derive(Serialize, Debug, Clone)]
pub struct LiveValuesEvent {

}

impl LiveValuesEvent {
    pub fn build(&self) -> Event<Self> {
        Event::new("LiveValuesEvent", self.clone())
    }
}

#[derive(Serialize, Debug, Clone, PartialEq, BuildEvent)]
pub struct StateEvent {
    pub motor_state: MotorState,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct MotorState {
    // pub direction: Direction,
    // pub speed:     f32,
}

pub enum VFDEvents {
    LiveValues(Event<LiveValuesEvent>),
    State(Event<StateEvent>),
}

#[derive(Debug)]
pub struct VFDMachineNamespace {
    pub namespace: Option<Namespace>,
}

impl CacheableEvents<Self> for VFDEvents {
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

#[derive(Deserialize, Serialize)]
/// Mutation for controlling the VFD machine
enum Mutation {
    SetDirection(bool),
    SetSpeed(f32),
}

//TODO; rename NamespaceCacheingLogic to NamespaceCachingLogic
impl NamespaceCacheingLogic<VFDEvents> for VFDMachineNamespace {
    #[instrument(skip_all)]
    fn emit(&mut self, events: VFDEvents) {
        let event = Arc::new(events.event_value());
        let buffer_fn = events.event_cache_fn();

        match &mut self.namespace {
            Some(ns) => ns.emit(event, &buffer_fn),
            None => (),
        }
    }
}

impl MachineApi for VFDMachine {
    fn api_get_sender(&self) -> Sender<MachineMessage> {
        self.api_sender.clone()
    }

    fn api_mutate(&mut self, request_body: Value) -> Result<(), anyhow::Error> {
        let mutation: Mutation = serde_json::from_value(request_body)?;
        match mutation {
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