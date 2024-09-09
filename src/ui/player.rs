use crate::data::Data;

use std::time::{Duration, Instant};

use super::{
    player_controls::{PlayerControlsModel, PlayerControlsMsg},
    player_settings::PlayerSettingsModel,
};
use gtk::prelude::*;
use relm4::{
    gtk, Component, ComponentController, ComponentParts, ComponentSender, Controller,
    SimpleComponent,
};

pub struct PlayerModel {
    controls: Controller<PlayerControlsModel>,
    settings: Controller<PlayerSettingsModel>,
    state: PlayerState,
}

#[derive(Debug)]
pub enum PlayerMsg {
    Ignore,
    NewData(Data),
    StepForward,
    StepBackward,
    FirstStep,
    LastStep,
    JumpToTime(f64),
    AnimationPing,
}

#[relm4::component(pub)]
impl SimpleComponent for PlayerModel {
    type Init = Data;
    type Input = PlayerMsg;
    type Output = ();

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,

            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 5,

                model.controls.widget(),

                gtk::Separator {},

                gtk::Label::new(Some("t = ")) {},
                gtk::Entry {
                    set_max_width_chars: 8,
                    set_valign: gtk::Align::Center,
                    #[watch]
                    set_text: &format!("{:.3}", model.state.time()),
                    connect_activate[sender] => move |entry| {
                        if let Ok(time) = entry.buffer().text().parse::<f64>() {
                            _ = sender.input_sender().send(PlayerMsg::JumpToTime(time));
                        }
                    }
                },

                gtk::Separator {},

                model.settings.widget(),
            },

            gtk::Label {
                #[watch]
                set_label: &format!("{} timesteps loaded", model.state.n_steps),
            },

            gtk::Label {
                #[watch]
                set_label: &format!("Showing step {} (time {:.3})", model.state.position, model.state.time()),
            }
        },
    }

    fn init(
        data: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let controls = PlayerControlsModel::builder().launch(()).forward(
            sender.input_sender(),
            |msg| match msg {
                PlayerControlsMsg::StepForward => PlayerMsg::StepForward,
                PlayerControlsMsg::StepBackward => PlayerMsg::StepBackward,
                PlayerControlsMsg::FirstStep => PlayerMsg::FirstStep,
                PlayerControlsMsg::LastStep => PlayerMsg::LastStep,
                _ => PlayerMsg::Ignore,
            },
        );

        let settings = PlayerSettingsModel::builder()
            .launch(())
            .forward(sender.input_sender(), |_| PlayerMsg::Ignore);

        let model = Self {
            controls,
            settings,
            state: PlayerState::from_data(data),
        };

        let widgets = view_output!();

        // Create timeout function for advancing animation
        let sender_clone = sender.clone();
        gtk::glib::source::timeout_add_local(Duration::from_millis(10), move || {
            _ = sender_clone.input_sender().send(PlayerMsg::AnimationPing);
            gtk::glib::ControlFlow::Continue
        });

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            PlayerMsg::NewData(data) => {
                self.state = PlayerState::from_data(data);
            }
            PlayerMsg::StepForward => {
                self.state.step_forward(self.settings.model().step_size);
            }
            PlayerMsg::StepBackward => {
                self.state.step_backward(self.settings.model().step_size);
            }
            PlayerMsg::FirstStep => {
                self.state.first_step();
            }
            PlayerMsg::LastStep => {
                self.state.last_step();
            }
            PlayerMsg::JumpToTime(time) => {
                self.state.jump_to_time(time);
            }
            PlayerMsg::AnimationPing => {
                if self.controls.model().is_playing {
                    let settings = &self.settings.model();
                    self.state.animate(
                        Duration::from_millis(settings.update_interval_ms),
                        settings.step_size,
                    );
                }
            }
            PlayerMsg::Ignore => {}
        }
    }
}

struct PlayerState {
    data: Data,
    n_steps: usize,
    times: Vec<f64>,
    position: usize,
    last_update: Instant,
}

impl PlayerState {
    pub fn from_data(data: Data) -> Self {
        let n_steps = data.dataslices.len();
        let times: Vec<f64> = data.dataslices.iter().map(|s| s.time).collect();
        let position = 0;
        Self {
            data,
            n_steps,
            times,
            position,
            last_update: Instant::now(),
        }
    }

    pub fn time(&self) -> f64 {
        if !self.times.is_empty() {
            self.times[self.position]
        } else {
            0.0
        }
    }

    pub fn step_forward(&mut self, steps: usize) {
        if self.n_steps == 0 {
            return;
        }

        let mut new_position = self.position.saturating_add(steps);

        if new_position >= self.n_steps {
            new_position = self.n_steps - 1;
        }

        self.position = new_position;
    }

    pub fn step_backward(&mut self, steps: usize) {
        if self.n_steps == 0 {
            return;
        }

        let new_position = self.position.saturating_sub(steps);
        self.position = new_position;
    }

    pub fn first_step(&mut self) {
        self.position = 0;
    }

    pub fn last_step(&mut self) {
        if self.n_steps == 0 {
            return;
        }

        self.position = self.n_steps - 1;
    }

    pub fn jump_to_time(&mut self, target_time: f64) {
        if self.times.is_empty() {
            return;
        }

        if target_time >= self.times[self.n_steps - 1] {
            self.last_step();
            return;
        }

        if target_time <= self.times[0] {
            self.first_step();
            return;
        }

        let step_before_target = self
            .times
            .iter()
            .rposition(|t| target_time >= *t)
            .expect("Failed to find target timestep");

        let time_before_target = self.times[step_before_target];
        let time_after_target = self.times[step_before_target + 1];
        let target_step = if (target_time - time_before_target) < (time_after_target - target_time)
        {
            step_before_target
        } else {
            step_before_target + 1
        };

        self.position = target_step;
    }

    pub fn animate(&mut self, update_interval: Duration, step_size: usize) {
        let now = Instant::now();
        let time_since_last_update = now - self.last_update;
        if update_interval < time_since_last_update {
            self.last_update = now;
            self.step_forward(step_size);
            // Message to trigger plot redraw goes here
        }
    }
}
