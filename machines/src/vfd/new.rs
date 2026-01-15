use std::time::Instant;

use super::{
    VFDMachine,
    api::{VFDMachineNamespace},
};
use crate::{MachineNewHardware, MachineNewParams, MachineNewTrait};
use anyhow::Error;

use crate::{
    MachineNewHardware, MachineNewParams, MachineNewTrait, get_ethercat_device,
    validate_no_role_dublicates, validate_same_machine_identification_unique,
};

use smol::lock::RwLock;
use std::sync::Arc;

use ethercat_hal::devices::{wago_750_354::{WAGO_750_354_IDENTITY_A, Wago750_354}, wago_modules::wago_750_652::Wago750_652};
use ethercat_hal::devices::{EthercatDevice, downcast_device};

impl MachineNewTrait for VFDMachine {
    fn new<'maindevice, 'subdevices>(
        params: &MachineNewParams<'maindevice, 'subdevices, '_, '_, '_, '_, '_>,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        use crate::{
            MachineNewHardware, MachineNewHardwareEthercat, validate_no_role_dublicates,
            validate_same_machine_identification_unique,
        };

        let device_identification = params.device_group.to_vec();

        validate_same_machine_identification_unique(&device_identification)?;
        validate_no_role_dublicates(&device_identification)?;



        let hardware = match &params.hardware {
            MachineNewHardware::Ethercat(x) => x,
            _ => {
                return Err(anyhow::anyhow!(
                    "[{}::MachineNewTrait/TestMachine::new] MachineNewHardware is not Ethercat",
                    module_path!()
                ));
            }
        };

        smol::block_on(async {
            let _wago_750_354 = get_ethercat_device::<Wago750_354>(
                hardware,
                params,
                0,
                [WAGO_750_354_IDENTITY_A].to_vec(),
            )
            .await?;

            let modules = Wago750_354::initialize_modules(_wago_750_354.1).await?;

            let mut coupler = _wago_750_354.0.write().await;

            for module in modules {
                coupler.set_module(module);
            }

            coupler.init_slot_modules(_wago_750_354.1);

            let dev = coupler.slot_devices.get(1).unwrap().clone().unwrap();

            let wago750_652: Arc<RwLock<Wago750_652>> = downcast_device::<Wago750_652>(dev).await.?;


            let inverter = 
        });

        let now = Instant::now();
        let (sender, receiver) = smol::channel::unbounded();
        let mut vfd_machine = Self {
            main_sender: params.main_thread_channel.clone(),
            api_receiver: receiver,
            api_sender: sender,
            machine_identification_unique: params.get_machine_identification_unique(),
            namespace: VFDMachineNamespace {
                namespace: params.namespace.clone(),
            },
            last_measurement_emit: now,

            direction: super::api::Direction::Forward,
            speed: 0.0,

            emitted_default_state: false,
            last_emitted_event: None,
        };

        vfd_machine.emit_state();

        Ok(vfd_machine)
    }
}
