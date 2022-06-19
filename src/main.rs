use std::alloc::System;
use std::process;
use std::ptr::null;
use windows::core::*;
use windows::Win32::Foundation::*;
use windows::Win32::System::DataExchange::{GlobalAddAtomA, GlobalAddAtomW};
use windows::Win32::UI::Input::KeyboardAndMouse::{RegisterHotKey, MOD_ALT};
use windows::Win32::UI::WindowsAndMessaging::{
    LoadCursorA, LoadIconA, IDC_ARROW, IDI_APPLICATION, MSG, WNDCLASS_STYLES,
};
use windows_sys::Win32::Foundation::HWND;
use windows_sys::Win32::Graphics::Gdi::{GetStockObject, WHITE_BRUSH};
use windows_sys::Win32::UI::WindowsAndMessaging::{FindWindowA, MessageBoxA, MB_ICONERROR};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    RegisterClassA, CS_HREDRAW, CS_VREDRAW, WNDCLASSA,
};
// src\main.rs

// - 获取热键标识：GlobalAddAtom
// - 注册热键：Register HotKey
// - 注销热键：UnregisterHotKey
// - 释放热键标识：GlobalDeleteAtom

// https://github.com/microsoft/windows-rs/search?q=RegisterClass api查询

// c++ 创建托盘程序 https://blog.csdn.net/wangyi463295828/article/details/117442705
// 创建windows系统托盘程序 https://blog.csdn.net/lileiyu1/article/details/79053325
fn main() {
    let mut msg: MSG;
    let sz_app_window_name = "CapsLockSwap";
    let sz_app_class_name = "CapsLockSwap";

    let (hwnd) = unsafe {
        let hwnd: HWND = FindWindowA(0, sz_app_window_name);
        if hwnd != null() {
            MessageBoxA(
                0,
                "Application is already running",
                sz_app_class_name,
                MB_ICONERROR,
            );
            process::exit(1);
        };
        let mut wndclass: WNDCLASSA = WNDCLASSA {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: None,
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: Default::default(),
            hIcon: LoadIconA(null(), IDI_APPLICATION),
            hCursor: LoadCursorA(null(), IDC_ARROW),
            hbrBackground: GetStockObject(WHITE_BRUSH),
            lpszMenuName: null(),
            lpszClassName: sz_app_class_name,
        };

        if RegisterClassA(&wndclass) == 0 {
            MessageBoxA(
                0,
                "Application is already running",
                sz_app_class_name,
                MB_ICONERROR,
            );
            process::exit(1);
        }
    };
}

// unsafe {
// // let hot_key_id = GlobalAddAtomA("swap_caps");
// let hot_key_id = GlobalAddAtomW("swap_caps");
// // https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes vk 枚举
// // https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-hotkey fsmodifiers 枚举
// // https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerhotkey
// let bool = RegisterHotKey(hwnd, hot_key_id as i32, MOD_ALT, 0x4A);
// }
