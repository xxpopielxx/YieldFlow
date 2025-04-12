// Główny moduł instrukcji programu
//
// Eksportuje wszystkie instrukcje programu pogrupowane w moduły:
// - admin: Operacje administracyjne programu
// - claim: Logika wypłat dywidend
// - initialize: Inicjalizacja kont użytkowników
//
// Każdy podmoduł zawiera:
// - Struktury Accounts z wymaganymi kontami
// - Funkcje handlerów wykonujące logikę instrukcji
// - Powiązane typy danych i walidacje
pub mod admin;
pub mod claim;
pub mod initialize;

pub use admin::*;
pub use claim::*;
pub use initialize::*;
