use async_std::task;
use gtk::prelude::*;
use gtk::glib::{self, clone};
use systemstat::{Platform, System};
use crate::electrodes::{DEFAULT_POLLING_DURATION, Electrode, make_icon};

pub struct CpuTemperature;

impl Electrode for CpuTemperature {
    fn setup(parent: &gtk::Box) {
        let (box_, label) = make_icon(parent, "");
        box_.style_context().add_class("electrode");

        glib::MainContext::default().spawn_local(clone!(@weak label => async move {
            let system = System::new();

            loop {
                let cpu_temperature = system.cpu_temp()
                    .expect("could not measure CPU temperature");

                let text = format!("{}°C", cpu_temperature.ceil());
                label.set_label(&text);

                task::sleep(DEFAULT_POLLING_DURATION).await;
            }
        }));
    }
}
