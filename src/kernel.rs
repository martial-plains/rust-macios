//! Develop kernel-resident device drivers and kernel extensions.

use libc::{
    c_char, c_double, c_float, c_int, c_longlong, c_schar, c_short, c_uchar, c_uint, c_ulonglong,
    c_ushort, c_void,
};

/* Kernel Data Types */

pub type AVIDType = UInt32;
pub type AbsoluteTime = UInt64;
pub type BDFeatures = UInt32;
pub type BDMediaType = UInt32;
pub type Boolean = bool;
pub type Byte = c_uchar;
pub type Bytef = Byte;
pub type BytePtr = *mut UInt8;
pub type CSRNodeUniqueID = UInt64;
pub type DepthMode = UInt16;
pub type DisplayIDType = AVIDType;
pub type DriverDescVersion = UInt32;
pub type Duration = SInt32;
pub type ExtendedSenseCode = UInt8;
pub type Fixed = UInt32;
pub type FixedPtr = *mut Fixed;
pub type Float32 = c_float;
pub type Float64 = c_double;
pub type FourCharCode = c_uint;
pub type Fract = UInt32;
pub type FractPtr = *mut Fract;
pub type GammaTableID = UInt32;
pub type Handle = *mut Ptr;
pub type LogicalAddress = *mut c_void;
pub type MasterMuteUpdate = fn() -> bool;
pub type MasterVolumeUpdate = fn() -> UInt16;
#[deprecated]
pub type OptionBits = UInt32;
pub type PBVersion = UInt32;
pub type Ptr = *mut c_char;
pub type RawSenseCode = UInt8;
pub type RegCStrEntryName = char;
pub type RegCStrEntryNameBuf = [char; 48];
pub type RegCStrEntryNamePtr = *mut char;
pub type RegCStrPathName = char;
pub type RegEntryIterationOp = RegIterationOp;
pub type RegEntryModifiers = RegModifiers;
pub type RegIterationOp = UInt32;
pub type RegModifiers = UInt32;
pub type RegPathNameSize = UInt32;
pub type RegPropertyModifiers = RegModifiers;
pub type RegPropertyName = char;
pub type RegPropertyNameBuf = [char; 32];
pub type RegPropertyNamePtr = *mut char;
pub type RegPropertyValue = *mut c_void;
pub type RegPropertyValueSize = UInt32;
pub type ResType = FourCharCode;
pub type ResTypePtr = ResType;
pub type SInt = c_int;
pub type SInt16 = c_short;
pub type SInt32 = c_int;
pub type SInt64 = c_longlong;
pub type SInt8 = c_schar;
pub type ServiceCount = UInt32;
pub type SignedByte = c_schar;
pub type Str31 = [char; 32];
pub type TransmissionPower = SInt8;
pub type UInt = c_uint;
pub type UInt16 = c_ushort;
pub type UInt32 = c_uint;
pub type UInt32Ptr = *mut UInt32;
pub type UInt64 = c_ulonglong;
pub type UInt8 = c_uchar;
pub type UNDKey = *mut char;
pub type UNDLabel = *mut char;
pub type UNDMessage = *mut char;
pub type UNDPath = *mut char;
pub type UniChar = u16;
pub type VOID = c_void;
pub type VideoDeviceType = UInt32;
pub type WK_word = c_uint;
pub type uint = c_uint;
