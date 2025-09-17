use std::ffi::*;

/// # Safety
/// see [std::slice::from_raw_parts]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn score_create(data: *const u8, len: usize) -> Box<ScoreResult> {
    let bytes = unsafe { std::slice::from_raw_parts(data, len) };
    let result = match kaizensim::score(bytes) {
        Ok(s) => {
            let level = s.level;
            let time = s.time;
            let cost = s.cost;
            let area = s.area;
            let manipulated = s.manipulated;
            let error = None;
            ScoreResult { error, level, time, cost, area, manipulated }
        },
        Err(e) => {
            let error = Some(CString::new(e.to_string()).unwrap_or(CString::from(c"Unknown error")));
            ScoreResult { error, level: 0, time: 0, cost: 0, area: 0, manipulated: false }
        },
    };
    Box::new(result)
}

#[unsafe(no_mangle)]
pub extern "C" fn score_destroy(_: Box<ScoreResult>) {
}

#[repr(C)]
pub struct ScoreResult {
    pub error: Option<CString>,
    pub level: i32,
    pub time: i32,
    pub cost: i32,
    pub area: i32,
    pub manipulated: bool,
}