extern crate winapi_typedefs;

use std::os::raw::*;
use winapi_typedefs::winuser::*;
use winapi_typedefs::windef::*;
use winapi_typedefs::intsafe::*;
use winapi_typedefs::winnt::*;
use winapi_typedefs::basetsd::*;

// Import declarations
#[allow(non_snake_case)]
#[link(name="user32")]
extern "system"
{
    fn RegisterClassExA(wndClass: *const WNDCLASSEXA) -> ATOM;
    fn RegisterClassExW(wndClass: *const WNDCLASSEXW) -> ATOM;
    fn CreateWindowExA(
        dwExStyle: DWORD,
        lpClassName: LPCSTR,
        lpWindowName: LPCSTR,
        dwStyle: DWORD,
        x: c_uint,
        y: c_uint,
        nWidth: c_uint,
        nHeight: c_uint,
        hWndParent: HWND,
        hMenu: HMENU,
        hInstance: HINSTANCE,
        lpParam: LPVOID) -> HWND;
    fn CreateWindowExW(
        dwExStyle: DWORD,
        lpClassName: LPCWSTR,
        lpWindowName: LPCWSTR,
        dwStyle: DWORD,
        x: c_uint,
        y: c_uint,
        nWidth: c_uint,
        nHeight: c_uint,
        hWndParent: HWND,
        hMenu: HMENU,
        hInstance: HINSTANCE,
        lpParam: LPVOID) -> HWND;
    fn GetMessageA(lpmsg: LPMSG, hWnd: HWND, wMsgFilterMin: UINT, wMsgFilterMax: UINT) -> BOOL;
    fn GetMessageW(lpmsg: LPMSG, hWnd: HWND, wMsgFilterMin: UINT, wMsgFilterMax: UINT) -> BOOL;
    fn TranslateMessage(lpmsg: *const MSG) -> BOOL;
    fn DispatchMessageA(lpmsg: *const MSG) -> LRESULT;
    fn DispatchMessageW(lpmsg: *const MSG) -> LRESULT;
    fn LoadIconA(hInstance: HINSTANCE, lpIconName: LPCSTR) -> HICON;
    fn LoadIconW(hInstance: HINSTANCE, lpIconName: LPCWSTR) -> HICON;
    fn LoadCursorA(hInstance: HINSTANCE, lpCursorName: LPCSTR) -> HCURSOR;
    fn LoadCursorW(hInstance: HINSTANCE, lpCursorName: LPCWSTR) -> HCURSOR;
    fn GetSysColorBrush(nIndex: c_int) -> HBRUSH;
    fn DefWindowProcA(hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
    fn DefWindowProcW(hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
    fn ShowWindow(hWnd: HWND, nCmdShow: c_int) -> BOOL;
    fn SetWindowPos(hwnd: HWND, hwndInsertAfter: HWND, x: c_int, y: c_int, cx: c_int, cy: c_int, uFlags: UINT) -> BOOL;
    fn MessageBoxA(hWnd: HWND, lpText: LPCSTR, lpCaption: LPCSTR, uType: UINT) -> c_int;
    fn GetWindowLongPtrA(hWnd: HWND, nIndex: c_int) -> LONG_PTR;
    fn SetWindowLongPtrA(hWnd: HWND, nIndex: c_int, dwNewLong: LONG_PTR);
    fn GetDC(hwnd: HWND) -> HDC;
    fn BeginPaint(hwnd: HWND, lpPaint: LPPAINTSTRUCT) -> HDC;
    fn EndPaint(hwnd: HWND, lpPaint: *const PAINTSTRUCT) -> BOOL;
    fn UpdateWindow(hwnd: HWND) -> BOOL;
}

// Safe interfaces
/// https://msdn.microsoft.com/en-us/library/windows/desktop/ms633587(v=vs.85).aspx
pub fn register_class_ex_a(wnd_class: WNDCLASSEXA) -> ATOM
{
    unsafe
    {
        let wnd_class_ptr = &wnd_class as *const WNDCLASSEXA;
        RegisterClassExA(wnd_class_ptr)
    }
}

/// https://msdn.microsoft.com/en-us/library/windows/desktop/ms633587(v=vs.85).aspx
pub fn register_class_ex_w(wnd_class: WNDCLASSEXW) -> ATOM
{
    unsafe
    {
        let wnd_class_ptr = &wnd_class as *const WNDCLASSEXW;
        RegisterClassExW(wnd_class_ptr)
    }
}

/// https://msdn.microsoft.com/de-de/library/windows/desktop/ms632680(v=vs.85).aspx
pub fn create_window_ex_a(
    dw_ex_style: DWORD,
    lp_class_name: LPCSTR,
    lp_window_name: LPCSTR,
    dw_style: DWORD,
    x: c_uint,
    y: c_uint,
    n_width: c_uint,
    n_height: c_uint,
    h_wnd_parent: HWND,
    h_menu: HMENU,
    h_instance: HINSTANCE,
    lp_param: LPVOID) -> Option<HWND>
{
    unsafe
    {
        let hwnd = CreateWindowExA(dw_ex_style, lp_class_name, lp_window_name, dw_style,
            x, y, n_width, n_height, h_wnd_parent, h_menu, h_instance, lp_param);
        if hwnd.is_null()
        {
            return None;
        }
        else
        {
            return Some(hwnd);
        }
    }
}

/// https://msdn.microsoft.com/de-de/library/windows/desktop/ms632680(v=vs.85).aspx
pub fn create_window_ex_w(
    dw_ex_style: DWORD,
    lp_class_name: LPCWSTR,
    lp_window_name: LPCWSTR,
    dw_style: DWORD,
    x: c_uint,
    y: c_uint,
    n_width: c_uint,
    n_height: c_uint,
    h_wnd_parent: HWND,
    h_menu: HMENU,
    h_instance: HINSTANCE,
    lp_param: LPVOID) -> Option<HWND>
{
    unsafe
    {
        let hwnd = CreateWindowExW(dw_ex_style, lp_class_name, lp_window_name, dw_style,
            x, y, n_width, n_height, h_wnd_parent, h_menu, h_instance, lp_param);
        if hwnd.is_null()
        {
            return None;
        }
        else
        {
            return Some(hwnd);
        }
    }
}

/// https://msdn.microsoft.com/de-de/library/windows/desktop/ms644936(v=vs.85).aspx
pub fn get_message_a(
    lp_msg: LPMSG,
    h_wnd: HWND,
    w_msg_filter_min: UINT,
    w_msg_filter_max: UINT) -> bool
{
    unsafe
    {
        if GetMessageA(lp_msg, h_wnd, w_msg_filter_min, w_msg_filter_max) == TRUE
        {
            return true;
        }
        else
        {
            return false;
        }
    }
}

/// https://msdn.microsoft.com/de-de/library/windows/desktop/ms644936(v=vs.85).aspx
pub fn get_message_w(
    lp_msg: LPMSG,
    h_wnd: HWND,
    w_msg_filter_max: UINT,
    w_msg_filter_min: UINT) -> bool
{
    unsafe
    {
        if GetMessageW(lp_msg, h_wnd, w_msg_filter_max, w_msg_filter_min) == TRUE
        {
            return true;
        }
        else
        {
            return false;
        }
    }
}

/// https://msdn.microsoft.com/de-de/library/windows/desktop/ms644955(v=vs.85).aspx
pub fn translate_message(lp_msg: *const MSG) -> bool
{
    unsafe
    {
        if TranslateMessage(lp_msg) == TRUE
        {
            return true;
        }
        else
        {
            return false;
        }
    }
}

/// https://msdn.microsoft.com/de-de/library/windows/desktop/ms644934(v=vs.85).aspx
pub fn dispatch_message_a(lp_msg: *const MSG) -> LRESULT
{
    unsafe
    {
        DispatchMessageA(lp_msg)
    }
}

/// https://msdn.microsoft.com/de-de/library/windows/desktop/ms644934(v=vs.85).aspx
pub fn dispatch_message_w(lp_msg: *const MSG) -> LRESULT
{
    unsafe
    {
        DispatchMessageW(lp_msg)
    }
}

/// https://msdn.microsoft.com/de-de/library/windows/desktop/ms648072(v=vs.85).aspx
pub fn load_icon_a(h_instance: HINSTANCE, lp_icon_name: LPCSTR) -> Option<HICON>
{
    unsafe
    {

        let hicon = LoadIconA(h_instance, lp_icon_name);
        if hicon.is_null()
        {
            return None;
        }
        else
        {
            return Some(hicon);
        }
    }
}

/// https://msdn.microsoft.com/de-de/library/windows/desktop/ms648072(v=vs.85).aspx
pub fn load_icon_w(h_instance: HINSTANCE, lp_icon_name: LPCWSTR) -> Option<HICON>
{
    unsafe
    {
        let hicon = LoadIconW(h_instance, lp_icon_name);
        if hicon.is_null()
        {
            return None;
        }
        else
        {
            return Some(hicon);
        }
    }
}

/// https://msdn.microsoft.com/de-de/library/windows/desktop/ms648391(v=vs.85).aspx
pub fn load_cursor_a(h_instance: HINSTANCE, lp_cursor_name: LPCSTR) -> Option<HCURSOR>
{
    unsafe
    {
        let hcursor = LoadCursorA(h_instance, lp_cursor_name);
        if hcursor.is_null()
        {
            return None;
        }
        else
        {
            return Some(hcursor);
        }
    }
}

/// https://msdn.microsoft.com/de-de/library/windows/desktop/ms648391(v=vs.85).aspx
pub fn load_cursor_w(h_instance: HINSTANCE, lp_cursor_name: LPCWSTR) -> Option<HCURSOR>
{
    unsafe
    {
        let hcursor = LoadCursorW(h_instance, lp_cursor_name);
        if hcursor.is_null()
        {
            return None;
        }
        else
        {
            return Some(hcursor);
        }
    }
}

/// https://msdn.microsoft.com/de-de/library/windows/desktop/dd144927(v=vs.85).aspx
pub fn get_sys_color_brush(n_index: c_int) -> Option<HBRUSH>
{
    unsafe
    {
        let hbrush = GetSysColorBrush(n_index);
        if hbrush.is_null()
        {
            return None;
        }
        else
        {
            return Some(hbrush);
        }
    }
}

/// https://msdn.microsoft.com/de-de/library/windows/desktop/ms633572(v=vs.85).aspx
pub extern "system" fn def_window_proc_a(
    h_wnd: HWND,
    msg: UINT,
    w_param: WPARAM,
    l_param: LPARAM) -> LRESULT
{
    unsafe
    {
        DefWindowProcA(h_wnd, msg, w_param, l_param)
    }
}

/// https://msdn.microsoft.com/de-de/library/windows/desktop/ms633572(v=vs.85).aspx
pub fn def_window_proc_w(
    h_wnd: HWND,
    msg: UINT,
    w_param: WPARAM,
    l_param: LPARAM) -> LRESULT
{
    unsafe
    {
        DefWindowProcW(h_wnd, msg, w_param, l_param)
    }
}

/// https://msdn.microsoft.com/de-de/library/windows/desktop/ms633548(v=vs.85).aspx
pub fn show_window(h_wnd: HWND, n_cmd_show: c_int) -> bool
{
    unsafe
    {
        if ShowWindow(h_wnd, n_cmd_show) == TRUE
        {
            return true;
        }
        else
        {
            return false;
        }
    }
}

/// https://msdn.microsoft.com/de-de/library/windows/desktop/ms633545(v=vs.85).aspx
pub fn set_window_pos(
    h_wnd: HWND, h_wnd_insert_after: HWND, x: c_int, y: c_int, cx: c_int, cy: c_int, u_flags: UINT) -> bool
{
    unsafe
    {
        if SetWindowPos(h_wnd, h_wnd_insert_after, x, y, cx, cy, u_flags) == TRUE
        {
            return true;
        }
        else
        {
            return false;
        }
    }
}

/// https://msdn.microsoft.com/en-us/library/windows/desktop/ms645505(v=vs.85).aspx
pub fn message_box_a(h_wnd: HWND, lp_text: LPCSTR, lp_caption: LPCSTR, u_type: UINT) -> c_int
{
    unsafe
    {
        MessageBoxA(h_wnd, lp_text, lp_caption, u_type)
    }
}

/// https://docs.microsoft.com/en-us/windows/desktop/api/winuser/nf-winuser-getwindowlongptra
pub fn get_window_long_ptr_a(h_wnd: HWND, n_index: c_int) -> LONG_PTR
{
    unsafe
    {
        GetWindowLongPtrA(h_wnd, n_index)
    }
}

/// https://docs.microsoft.com/en-us/windows/desktop/api/winuser/nf-winuser-setwindowlongptra
pub fn set_window_long_ptr_a(h_wnd: HWND, n_index: c_int, dw_new_long: LONG_PTR)
{
    unsafe
    {
        SetWindowLongPtrA(h_wnd, n_index, dw_new_long)
    }
}

/// https://docs.microsoft.com/en-us/windows/desktop/api/winuser/nf-winuser-getdc
pub fn get_dc(h_wnd: HWND) -> Option<HDC>
{
    unsafe
    {
        let hdc = GetDC(h_wnd);
        if hdc.is_null()
        {
            return None;
        }
        else
        {
            return Some(hdc);
        }
    }
}

/// https://docs.microsoft.com/en-us/windows/desktop/api/winuser/nf-winuser-beginpaint
pub fn begin_paint(h_wnd: HWND, lp_paint: LPPAINTSTRUCT) -> Option<HDC>
{
    unsafe
    {
        let hdc = BeginPaint(h_wnd, lp_paint);
        if hdc.is_null()
        {
            return None;
        }
        else
        {
            return Some(hdc)
        }
    }
}

/// https://docs.microsoft.com/de-de/windows/desktop/api/winuser/nf-winuser-endpaint
pub fn end_paint(h_wnd: HWND, lp_paint: *const PAINTSTRUCT) -> bool
{
    unsafe
    {
        if EndPaint(h_wnd, lp_paint) == TRUE
        {
            return true;
        }
        else
        {
            return false;
        }
    }
}

/// https://docs.microsoft.com/en-us/windows/desktop/api/winuser/nf-winuser-updatewindow
pub fn update_window(hwnd: HWND) -> bool
{
    unsafe
    {
        if UpdateWindow(hwnd) == TRUE
        {
            return true;
        }
        else
        {
            return false;
        }
    }
}
