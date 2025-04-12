// Struktura konfiguracji programu Pandle
// Przechowuje wszystkie kluczowe adresy i ustawienia programu
//
// Pola:
// - admin: Pubkey - adres administratora programu
// - marinade_program: Pubkey - adres programu Marinade
// - msol_mint: Pubkey - adres mint'a msol
// - pandle_program: Pubkey - adres programu Pandle
// - usdc_mint: Pubkey - adres mint'a USDC
// - fee_account: Pubkey - konto na które trafiają opłaty
// - bump: u8 - wartość bump dla PDA
// - fees_enabled: bool - czy opłaty są aktywne
// - fee_rate: u16 - procentowa stawka opłaty (np. 100 = 1%)
//
// - impl ProgramConfig - zawiera stałą LEN określającą rozmiar struktury
//   (32 bajty * 5 pól Pubkey + 1 bajt bump + 1 bajt bool + 2 bajty fee_rate)

use anchor_lang::prelude::*;

#[account]
pub struct ProgramConfig {
    pub admin: Pubkey,
    pub marinade_program: Pubkey,
    pub msol_mint: Pubkey,
    pub pandle_program: Pubkey,
    pub usdc_mint: Pubkey,
    pub fee_account: Pubkey,
    pub bump: u8,
    pub fees_enabled: bool,
    pub fee_rate: u16,
}

impl ProgramConfig {
    pub const LEN: usize = 32 * 5 + 1 + 1 + 2;
}