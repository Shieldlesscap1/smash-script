#![feature(proc_macro_hygiene)]
#![feature(asm)]
#[macro_use]
pub mod macros;

pub use lua_macro::*;
type ScriptBootstrapperFunc = unsafe extern "C" fn(&mut smash::lua2cpp::L2CAgentBase, &mut smash::lib::utility::Variadic);
type SysLineControlFunc = unsafe extern "C" fn(&mut smash::lua2cpp::L2CFighterCommon) -> smash::lib::L2CValue;
type SysLineCallbackFunc = unsafe fn(&mut smash::lua2cpp::L2CFighterCommon);
type SysLineWeaponControlFunc = unsafe extern "C" fn(&mut smash::lua2cpp::L2CFighterBase) -> smash::lib::L2CValue;
type SysLineWeaponCallbackFunc = unsafe fn(&mut smash::lua2cpp::L2CFighterBase);

#[macro_export]
macro_rules! replace_scripts {
    ($($func:ident),* $(,)?) => {
        $(
            $crate::replace_script!($func);
        )*
    };
}

#[macro_export]
macro_rules! replace_fighter_frames {
    ($($func:ident),* $(,)?) => {
        $(
            $crate::replace_fighter_frame!($func);
        )*
    };
}

#[macro_export]
macro_rules! replace_weapon_frames {
    ($($func:ident),* $(,)?) => {
        $(
            $crate::replace_weapon_frame!($func);
        )*
    };
}

#[macro_export]
macro_rules! add_fighter_frame_callbacks {
    ($($func:ident),* $(,)?) => {
        $(
            unsafe { $crate::add_sys_line_fighter_callback($func); }
        )*
    };
}

#[macro_export]
macro_rules! add_weapon_frame_callbacks {
    ($($func:ident),* $(,)?) => {
        $(
            unsafe { $crate::add_sys_line_weapon_callback($func); }
        )*
    };
}

extern "Rust" {
    pub fn replace_lua_script(fighter: &'static str, script: smash::phx::Hash40, func: ScriptBootstrapperFunc, is_new: bool);
    pub fn replace_sys_line_fighter_script(agent: smash::lib::LuaConst, func: SysLineControlFunc);
    pub fn replace_sys_line_weapon_script(agent: smash::lib::LuaConst, func: SysLineWeaponControlFunc);
    pub fn add_sys_line_fighter_callback(func: SysLineCallbackFunc);
    pub fn add_sys_line_weapon_callback(func: SysLineWeaponCallbackFunc);
}