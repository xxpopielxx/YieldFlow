// Moduł state - główny moduł przechowujący struktury danych programu
//
// Zawiera:
// - config - moduł z konfiguracją programu (ProgramConfig)
// - user_stake - moduł ze strukturami związanymi ze stakingiem użytkowników
//
// Udostępnia publicznie:
// - wszystkie struktury z modułu config
// - wszystkie struktury z modułu user_stake
//
// Służy jako główny punkt eksportu struktur stanu programu

pub mod config;
pub mod user_stake;

pub use config::*;
pub use user_stake::*;