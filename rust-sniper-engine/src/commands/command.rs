use crate::settings::settings::SniperSettings;

#[derive(Debug, Clone)]
pub enum Command {
    StartBot,
    StopBot,
    UpdateSettings(SniperSettings),
}
