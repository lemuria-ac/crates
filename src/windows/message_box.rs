type HWND = isize;
type LPCWSTR = *const u16;
type UINT = u32;

#[derive(Clone)]
pub struct MessageBox {
    hwnd: HWND,
    title: String,
    message: String,
    icon: ICON,
    button: BUTTON,
}

#[derive(Copy, Clone)]
pub enum ICON {
    None,
    Error = 0x10,
    Question = 0x20,
    Warning = 0x30,
    Information = 0x40,
}

#[derive(Copy, Clone)]
pub enum BUTTON {
    Ok = 0x0,
    OkCancel = 0x1,
    Abortretryignore = 0x2,
    YesNoCancel = 0x3,
    YesNo = 0x4,
    RetryCancel = 0x5,
    Canceltrycontinue = 0x6,
}

#[derive(Copy, Clone)]
enum CODE {
    NONE,
    OK,
    CANCEL,
    ABORT,
    RETRY,
    IGNORE,
    YES,
    NO,
    TRYAGAIN = 10,
    CONTINUE = 11,
}

impl From<i32> for CODE {
    fn from(id: i32) -> Self {
        match id {
            1 => CODE::OK,
            2 => CODE::CANCEL,
            3 => CODE::ABORT,
            4 => CODE::RETRY,
            5 => CODE::IGNORE,
            6 => CODE::YES,
            7 => CODE::NO,
            10 => CODE::TRYAGAIN,
            11 => CODE::CONTINUE,
            _ => CODE::NONE,
        }
    }
}

#[link(name = "user32")]
unsafe extern "system" {
    fn MessageBoxW(hwnd: HWND, lpText: LPCWSTR, lpCaption: LPCWSTR, uType: UINT) -> i32;
}

impl MessageBox {
    pub fn new() -> Self {
        Self {
            hwnd: 0,
            title: "No Title".to_string(),
            message: "No Description".to_string(),
            icon: ICON::None,
            button: BUTTON::Ok,
        }
    }

    fn to_lpcstr(&self, lp_text: &str) -> Vec<u16> {
        let mut text: Vec<u16> = lp_text.encode_utf16().collect();
        text.push(0);
        text
    }

    fn message_box_w(&self) -> bool {
        let icon = self.icon as u32;
        let button = self.button as u32;
        let u_type = icon + button;

        let code: CODE = unsafe {
            MessageBoxW(
                self.hwnd,
                self.to_lpcstr(&self.message).as_ptr(),
                self.to_lpcstr(&self.title).as_ptr(),
                u_type,
            )
        }
        .into();

        self.press_ok(code)
    }

    fn press_ok(&self, code: CODE) -> bool {
        match code {
            CODE::OK | CODE::CONTINUE | CODE::RETRY | CODE::TRYAGAIN | CODE::YES => true,
            _ => false,
        }
    }

    pub fn hwnd(mut self, hwnd: HWND) -> Self {
        self.hwnd = hwnd;
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }

    pub fn icon(mut self, icon: ICON) -> Self {
        self.icon = icon;
        self
    }

    pub fn button(mut self, button: BUTTON) -> Self {
        self.button = button;
        self
    }

    pub fn show(&self) {
        self.message_box_w();
    }

    pub fn show_action<F>(&self, callback: F)
    where
        F: Fn(bool),
    {
        callback(self.message_box_w())
    }
}
