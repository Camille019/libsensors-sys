/* automatically generated by rust-bindgen */

pub const SENSORS_API_VERSION: u32 = 1088;
pub const SENSORS_CHIP_NAME_ADDR_ANY: i32 = -1;
pub const SENSORS_BUS_TYPE_ANY: i32 = -1;
pub const SENSORS_BUS_TYPE_I2C: u32 = 0;
pub const SENSORS_BUS_TYPE_ISA: u32 = 1;
pub const SENSORS_BUS_TYPE_PCI: u32 = 2;
pub const SENSORS_BUS_TYPE_SPI: u32 = 3;
pub const SENSORS_BUS_TYPE_VIRTUAL: u32 = 4;
pub const SENSORS_BUS_TYPE_ACPI: u32 = 5;
pub const SENSORS_BUS_TYPE_HID: u32 = 6;
pub const SENSORS_BUS_NR_ANY: i32 = -1;
pub const SENSORS_BUS_NR_IGNORE: i32 = -2;
pub const SENSORS_MODE_R: u32 = 1;
pub const SENSORS_MODE_W: u32 = 2;
pub const SENSORS_COMPUTE_MAPPING: u32 = 4;
pub type __off_t = ::std::os::raw::c_long;
pub type __off64_t = ::std::os::raw::c_long;
pub type FILE = _IO_FILE;
pub type _IO_lock_t = ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout__IO_marker() {
    assert_eq!(
        ::std::mem::size_of::<_IO_marker>(),
        24usize,
        concat!("Size of: ", stringify!(_IO_marker))
    );
    assert_eq!(
        ::std::mem::align_of::<_IO_marker>(),
        8usize,
        concat!("Alignment of ", stringify!(_IO_marker))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_marker>()))._next as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_marker),
            "::",
            stringify!(_next)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_marker>()))._sbuf as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_marker),
            "::",
            stringify!(_sbuf)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_marker>()))._pos as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_marker),
            "::",
            stringify!(_pos)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_FILE {
    pub _flags: ::std::os::raw::c_int,
    pub _IO_read_ptr: *mut ::std::os::raw::c_char,
    pub _IO_read_end: *mut ::std::os::raw::c_char,
    pub _IO_read_base: *mut ::std::os::raw::c_char,
    pub _IO_write_base: *mut ::std::os::raw::c_char,
    pub _IO_write_ptr: *mut ::std::os::raw::c_char,
    pub _IO_write_end: *mut ::std::os::raw::c_char,
    pub _IO_buf_base: *mut ::std::os::raw::c_char,
    pub _IO_buf_end: *mut ::std::os::raw::c_char,
    pub _IO_save_base: *mut ::std::os::raw::c_char,
    pub _IO_backup_base: *mut ::std::os::raw::c_char,
    pub _IO_save_end: *mut ::std::os::raw::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::std::os::raw::c_int,
    pub _flags2: ::std::os::raw::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: ::std::os::raw::c_ushort,
    pub _vtable_offset: ::std::os::raw::c_schar,
    pub _shortbuf: [::std::os::raw::c_char; 1usize],
    pub _lock: *mut _IO_lock_t,
    pub _offset: __off64_t,
    pub __pad1: *mut ::std::os::raw::c_void,
    pub __pad2: *mut ::std::os::raw::c_void,
    pub __pad3: *mut ::std::os::raw::c_void,
    pub __pad4: *mut ::std::os::raw::c_void,
    pub __pad5: usize,
    pub _mode: ::std::os::raw::c_int,
    pub _unused2: [::std::os::raw::c_char; 20usize],
}
#[test]
fn bindgen_test_layout__IO_FILE() {
    assert_eq!(
        ::std::mem::size_of::<_IO_FILE>(),
        216usize,
        concat!("Size of: ", stringify!(_IO_FILE))
    );
    assert_eq!(
        ::std::mem::align_of::<_IO_FILE>(),
        8usize,
        concat!("Alignment of ", stringify!(_IO_FILE))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._flags as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_read_ptr as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_read_ptr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_read_end as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_read_end)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_read_base as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_read_base)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_write_base as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_write_base)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_write_ptr as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_write_ptr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_write_end as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_write_end)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_buf_base as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_buf_base)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_buf_end as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_buf_end)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_save_base as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_save_base)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_backup_base as *const _ as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_backup_base)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_save_end as *const _ as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_save_end)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._markers as *const _ as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_markers)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._chain as *const _ as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_chain)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._fileno as *const _ as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_fileno)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._flags2 as *const _ as usize },
        116usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_flags2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._old_offset as *const _ as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_old_offset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._cur_column as *const _ as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_cur_column)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._vtable_offset as *const _ as usize },
        130usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_vtable_offset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._shortbuf as *const _ as usize },
        131usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_shortbuf)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._lock as *const _ as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_lock)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._offset as *const _ as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_offset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>())).__pad1 as *const _ as usize },
        152usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(__pad1)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>())).__pad2 as *const _ as usize },
        160usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(__pad2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>())).__pad3 as *const _ as usize },
        168usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(__pad3)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>())).__pad4 as *const _ as usize },
        176usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(__pad4)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>())).__pad5 as *const _ as usize },
        184usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(__pad5)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._mode as *const _ as usize },
        192usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_mode)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._unused2 as *const _ as usize },
        196usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_unused2)
        )
    );
}
extern "C" {
    #[link_name = "\u{1}libsensors_version"]
    pub static mut libsensors_version: *const ::std::os::raw::c_char;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sensors_bus_id {
    pub type_: ::std::os::raw::c_short,
    pub nr: ::std::os::raw::c_short,
}
#[test]
fn bindgen_test_layout_sensors_bus_id() {
    assert_eq!(
        ::std::mem::size_of::<sensors_bus_id>(),
        4usize,
        concat!("Size of: ", stringify!(sensors_bus_id))
    );
    assert_eq!(
        ::std::mem::align_of::<sensors_bus_id>(),
        2usize,
        concat!("Alignment of ", stringify!(sensors_bus_id))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sensors_bus_id>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sensors_bus_id),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sensors_bus_id>())).nr as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(sensors_bus_id),
            "::",
            stringify!(nr)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sensors_chip_name {
    pub prefix: *mut ::std::os::raw::c_char,
    pub bus: sensors_bus_id,
    pub addr: ::std::os::raw::c_int,
    pub path: *mut ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_sensors_chip_name() {
    assert_eq!(
        ::std::mem::size_of::<sensors_chip_name>(),
        24usize,
        concat!("Size of: ", stringify!(sensors_chip_name))
    );
    assert_eq!(
        ::std::mem::align_of::<sensors_chip_name>(),
        8usize,
        concat!("Alignment of ", stringify!(sensors_chip_name))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sensors_chip_name>())).prefix as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sensors_chip_name),
            "::",
            stringify!(prefix)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sensors_chip_name>())).bus as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(sensors_chip_name),
            "::",
            stringify!(bus)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sensors_chip_name>())).addr as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(sensors_chip_name),
            "::",
            stringify!(addr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sensors_chip_name>())).path as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(sensors_chip_name),
            "::",
            stringify!(path)
        )
    );
}
extern "C" {
    pub fn sensors_init(input: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sensors_cleanup();
}
extern "C" {
    pub fn sensors_parse_chip_name(
        orig_name: *const ::std::os::raw::c_char,
        res: *mut sensors_chip_name,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sensors_free_chip_name(chip: *mut sensors_chip_name);
}
extern "C" {
    pub fn sensors_snprintf_chip_name(
        str: *mut ::std::os::raw::c_char,
        size: usize,
        chip: *const sensors_chip_name,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sensors_get_adapter_name(bus: *const sensors_bus_id) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn sensors_get_label(
        name: *const sensors_chip_name,
        feature: *const sensors_feature,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn sensors_get_value(
        name: *const sensors_chip_name,
        subfeat_nr: ::std::os::raw::c_int,
        value: *mut f64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sensors_set_value(
        name: *const sensors_chip_name,
        subfeat_nr: ::std::os::raw::c_int,
        value: f64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sensors_do_chip_sets(name: *const sensors_chip_name) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sensors_get_detected_chips(
        match_: *const sensors_chip_name,
        nr: *mut ::std::os::raw::c_int,
    ) -> *const sensors_chip_name;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum sensors_feature_type {
    SENSORS_FEATURE_IN = 0,
    SENSORS_FEATURE_FAN = 1,
    SENSORS_FEATURE_TEMP = 2,
    SENSORS_FEATURE_POWER = 3,
    SENSORS_FEATURE_ENERGY = 4,
    SENSORS_FEATURE_CURR = 5,
    SENSORS_FEATURE_HUMIDITY = 6,
    SENSORS_FEATURE_MAX_MAIN = 7,
    SENSORS_FEATURE_VID = 16,
    SENSORS_FEATURE_INTRUSION = 17,
    SENSORS_FEATURE_MAX_OTHER = 18,
    SENSORS_FEATURE_BEEP_ENABLE = 24,
    SENSORS_FEATURE_MAX = 25,
    SENSORS_FEATURE_UNKNOWN = 2147483647,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum sensors_subfeature_type {
    SENSORS_SUBFEATURE_IN_INPUT = 0,
    SENSORS_SUBFEATURE_IN_MIN = 1,
    SENSORS_SUBFEATURE_IN_MAX = 2,
    SENSORS_SUBFEATURE_IN_LCRIT = 3,
    SENSORS_SUBFEATURE_IN_CRIT = 4,
    SENSORS_SUBFEATURE_IN_AVERAGE = 5,
    SENSORS_SUBFEATURE_IN_LOWEST = 6,
    SENSORS_SUBFEATURE_IN_HIGHEST = 7,
    SENSORS_SUBFEATURE_IN_ALARM = 128,
    SENSORS_SUBFEATURE_IN_MIN_ALARM = 129,
    SENSORS_SUBFEATURE_IN_MAX_ALARM = 130,
    SENSORS_SUBFEATURE_IN_BEEP = 131,
    SENSORS_SUBFEATURE_IN_LCRIT_ALARM = 132,
    SENSORS_SUBFEATURE_IN_CRIT_ALARM = 133,
    SENSORS_SUBFEATURE_FAN_INPUT = 256,
    SENSORS_SUBFEATURE_FAN_MIN = 257,
    SENSORS_SUBFEATURE_FAN_MAX = 258,
    SENSORS_SUBFEATURE_FAN_ALARM = 384,
    SENSORS_SUBFEATURE_FAN_FAULT = 385,
    SENSORS_SUBFEATURE_FAN_DIV = 386,
    SENSORS_SUBFEATURE_FAN_BEEP = 387,
    SENSORS_SUBFEATURE_FAN_PULSES = 388,
    SENSORS_SUBFEATURE_FAN_MIN_ALARM = 389,
    SENSORS_SUBFEATURE_FAN_MAX_ALARM = 390,
    SENSORS_SUBFEATURE_TEMP_INPUT = 512,
    SENSORS_SUBFEATURE_TEMP_MAX = 513,
    SENSORS_SUBFEATURE_TEMP_MAX_HYST = 514,
    SENSORS_SUBFEATURE_TEMP_MIN = 515,
    SENSORS_SUBFEATURE_TEMP_CRIT = 516,
    SENSORS_SUBFEATURE_TEMP_CRIT_HYST = 517,
    SENSORS_SUBFEATURE_TEMP_LCRIT = 518,
    SENSORS_SUBFEATURE_TEMP_EMERGENCY = 519,
    SENSORS_SUBFEATURE_TEMP_EMERGENCY_HYST = 520,
    SENSORS_SUBFEATURE_TEMP_LOWEST = 521,
    SENSORS_SUBFEATURE_TEMP_HIGHEST = 522,
    SENSORS_SUBFEATURE_TEMP_MIN_HYST = 523,
    SENSORS_SUBFEATURE_TEMP_LCRIT_HYST = 524,
    SENSORS_SUBFEATURE_TEMP_ALARM = 640,
    SENSORS_SUBFEATURE_TEMP_MAX_ALARM = 641,
    SENSORS_SUBFEATURE_TEMP_MIN_ALARM = 642,
    SENSORS_SUBFEATURE_TEMP_CRIT_ALARM = 643,
    SENSORS_SUBFEATURE_TEMP_FAULT = 644,
    SENSORS_SUBFEATURE_TEMP_TYPE = 645,
    SENSORS_SUBFEATURE_TEMP_OFFSET = 646,
    SENSORS_SUBFEATURE_TEMP_BEEP = 647,
    SENSORS_SUBFEATURE_TEMP_EMERGENCY_ALARM = 648,
    SENSORS_SUBFEATURE_TEMP_LCRIT_ALARM = 649,
    SENSORS_SUBFEATURE_POWER_AVERAGE = 768,
    SENSORS_SUBFEATURE_POWER_AVERAGE_HIGHEST = 769,
    SENSORS_SUBFEATURE_POWER_AVERAGE_LOWEST = 770,
    SENSORS_SUBFEATURE_POWER_INPUT = 771,
    SENSORS_SUBFEATURE_POWER_INPUT_HIGHEST = 772,
    SENSORS_SUBFEATURE_POWER_INPUT_LOWEST = 773,
    SENSORS_SUBFEATURE_POWER_CAP = 774,
    SENSORS_SUBFEATURE_POWER_CAP_HYST = 775,
    SENSORS_SUBFEATURE_POWER_MAX = 776,
    SENSORS_SUBFEATURE_POWER_CRIT = 777,
    SENSORS_SUBFEATURE_POWER_AVERAGE_INTERVAL = 896,
    SENSORS_SUBFEATURE_POWER_ALARM = 897,
    SENSORS_SUBFEATURE_POWER_CAP_ALARM = 898,
    SENSORS_SUBFEATURE_POWER_MAX_ALARM = 899,
    SENSORS_SUBFEATURE_POWER_CRIT_ALARM = 900,
    SENSORS_SUBFEATURE_ENERGY_INPUT = 1024,
    SENSORS_SUBFEATURE_CURR_INPUT = 1280,
    SENSORS_SUBFEATURE_CURR_MIN = 1281,
    SENSORS_SUBFEATURE_CURR_MAX = 1282,
    SENSORS_SUBFEATURE_CURR_LCRIT = 1283,
    SENSORS_SUBFEATURE_CURR_CRIT = 1284,
    SENSORS_SUBFEATURE_CURR_AVERAGE = 1285,
    SENSORS_SUBFEATURE_CURR_LOWEST = 1286,
    SENSORS_SUBFEATURE_CURR_HIGHEST = 1287,
    SENSORS_SUBFEATURE_CURR_ALARM = 1408,
    SENSORS_SUBFEATURE_CURR_MIN_ALARM = 1409,
    SENSORS_SUBFEATURE_CURR_MAX_ALARM = 1410,
    SENSORS_SUBFEATURE_CURR_BEEP = 1411,
    SENSORS_SUBFEATURE_CURR_LCRIT_ALARM = 1412,
    SENSORS_SUBFEATURE_CURR_CRIT_ALARM = 1413,
    SENSORS_SUBFEATURE_HUMIDITY_INPUT = 1536,
    SENSORS_SUBFEATURE_VID = 4096,
    SENSORS_SUBFEATURE_INTRUSION_ALARM = 4352,
    SENSORS_SUBFEATURE_INTRUSION_BEEP = 4353,
    SENSORS_SUBFEATURE_BEEP_ENABLE = 6144,
    SENSORS_SUBFEATURE_UNKNOWN = 2147483647,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sensors_feature {
    pub name: *mut ::std::os::raw::c_char,
    pub number: ::std::os::raw::c_int,
    pub type_: sensors_feature_type,
    pub first_subfeature: ::std::os::raw::c_int,
    pub padding1: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_sensors_feature() {
    assert_eq!(
        ::std::mem::size_of::<sensors_feature>(),
        24usize,
        concat!("Size of: ", stringify!(sensors_feature))
    );
    assert_eq!(
        ::std::mem::align_of::<sensors_feature>(),
        8usize,
        concat!("Alignment of ", stringify!(sensors_feature))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sensors_feature>())).name as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sensors_feature),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sensors_feature>())).number as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(sensors_feature),
            "::",
            stringify!(number)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sensors_feature>())).type_ as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(sensors_feature),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<sensors_feature>())).first_subfeature as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(sensors_feature),
            "::",
            stringify!(first_subfeature)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sensors_feature>())).padding1 as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(sensors_feature),
            "::",
            stringify!(padding1)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sensors_subfeature {
    pub name: *mut ::std::os::raw::c_char,
    pub number: ::std::os::raw::c_int,
    pub type_: sensors_subfeature_type,
    pub mapping: ::std::os::raw::c_int,
    pub flags: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_sensors_subfeature() {
    assert_eq!(
        ::std::mem::size_of::<sensors_subfeature>(),
        24usize,
        concat!("Size of: ", stringify!(sensors_subfeature))
    );
    assert_eq!(
        ::std::mem::align_of::<sensors_subfeature>(),
        8usize,
        concat!("Alignment of ", stringify!(sensors_subfeature))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sensors_subfeature>())).name as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sensors_subfeature),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sensors_subfeature>())).number as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(sensors_subfeature),
            "::",
            stringify!(number)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sensors_subfeature>())).type_ as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(sensors_subfeature),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sensors_subfeature>())).mapping as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(sensors_subfeature),
            "::",
            stringify!(mapping)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sensors_subfeature>())).flags as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(sensors_subfeature),
            "::",
            stringify!(flags)
        )
    );
}
extern "C" {
    pub fn sensors_get_features(
        name: *const sensors_chip_name,
        nr: *mut ::std::os::raw::c_int,
    ) -> *const sensors_feature;
}
extern "C" {
    pub fn sensors_get_all_subfeatures(
        name: *const sensors_chip_name,
        feature: *const sensors_feature,
        nr: *mut ::std::os::raw::c_int,
    ) -> *const sensors_subfeature;
}
extern "C" {
    pub fn sensors_get_subfeature(
        name: *const sensors_chip_name,
        feature: *const sensors_feature,
        type_: sensors_subfeature_type,
    ) -> *const sensors_subfeature;
}
