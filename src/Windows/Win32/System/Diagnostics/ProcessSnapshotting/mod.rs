#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq)]
#[repr(transparent)]
pub struct HPSS(pub isize);
impl ::std::default::Default for HPSS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HPSS {}
unsafe impl ::windows::runtime::Abi for HPSS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq)]
#[repr(transparent)]
pub struct HPSSWALK(pub isize);
impl ::std::default::Default for HPSSWALK {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HPSSWALK {}
unsafe impl ::windows::runtime::Abi for HPSSWALK {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PSS_ALLOCATOR {
    pub Context: *mut ::std::ffi::c_void,
    pub AllocRoutine: isize,
    pub FreeRoutine: isize,
}
impl PSS_ALLOCATOR {}
impl ::std::default::Default for PSS_ALLOCATOR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PSS_ALLOCATOR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PSS_ALLOCATOR").field("Context", &self.Context).field("AllocRoutine", &self.AllocRoutine).field("FreeRoutine", &self.FreeRoutine).finish()
    }
}
impl ::std::cmp::PartialEq for PSS_ALLOCATOR {
    fn eq(&self, other: &Self) -> bool {
        self.Context == other.Context && self.AllocRoutine == other.AllocRoutine && self.FreeRoutine == other.FreeRoutine
    }
}
impl ::std::cmp::Eq for PSS_ALLOCATOR {}
unsafe impl ::windows::runtime::Abi for PSS_ALLOCATOR {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PSS_AUXILIARY_PAGES_INFORMATION {
    pub AuxPagesCaptured: u32,
}
impl PSS_AUXILIARY_PAGES_INFORMATION {}
impl ::std::default::Default for PSS_AUXILIARY_PAGES_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PSS_AUXILIARY_PAGES_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PSS_AUXILIARY_PAGES_INFORMATION").field("AuxPagesCaptured", &self.AuxPagesCaptured).finish()
    }
}
impl ::std::cmp::PartialEq for PSS_AUXILIARY_PAGES_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.AuxPagesCaptured == other.AuxPagesCaptured
    }
}
impl ::std::cmp::Eq for PSS_AUXILIARY_PAGES_INFORMATION {}
unsafe impl ::windows::runtime::Abi for PSS_AUXILIARY_PAGES_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
pub struct PSS_AUXILIARY_PAGE_ENTRY {
    pub Address: *mut ::std::ffi::c_void,
    pub BasicInformation: super::super::Memory::MEMORY_BASIC_INFORMATION,
    pub CaptureTime: super::super::super::Foundation::FILETIME,
    pub PageContents: *mut ::std::ffi::c_void,
    pub PageSize: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
impl PSS_AUXILIARY_PAGE_ENTRY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
impl ::std::default::Default for PSS_AUXILIARY_PAGE_ENTRY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
impl ::std::fmt::Debug for PSS_AUXILIARY_PAGE_ENTRY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PSS_AUXILIARY_PAGE_ENTRY").field("Address", &self.Address).field("BasicInformation", &self.BasicInformation).field("CaptureTime", &self.CaptureTime).field("PageContents", &self.PageContents).field("PageSize", &self.PageSize).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
impl ::std::cmp::PartialEq for PSS_AUXILIARY_PAGE_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.Address == other.Address && self.BasicInformation == other.BasicInformation && self.CaptureTime == other.CaptureTime && self.PageContents == other.PageContents && self.PageSize == other.PageSize
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
impl ::std::cmp::Eq for PSS_AUXILIARY_PAGE_ENTRY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
unsafe impl ::windows::runtime::Abi for PSS_AUXILIARY_PAGE_ENTRY {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PSS_CAPTURE_FLAGS(pub u32);
pub const PSS_CAPTURE_NONE: PSS_CAPTURE_FLAGS = PSS_CAPTURE_FLAGS(0u32);
pub const PSS_CAPTURE_VA_CLONE: PSS_CAPTURE_FLAGS = PSS_CAPTURE_FLAGS(1u32);
pub const PSS_CAPTURE_RESERVED_00000002: PSS_CAPTURE_FLAGS = PSS_CAPTURE_FLAGS(2u32);
pub const PSS_CAPTURE_HANDLES: PSS_CAPTURE_FLAGS = PSS_CAPTURE_FLAGS(4u32);
pub const PSS_CAPTURE_HANDLE_NAME_INFORMATION: PSS_CAPTURE_FLAGS = PSS_CAPTURE_FLAGS(8u32);
pub const PSS_CAPTURE_HANDLE_BASIC_INFORMATION: PSS_CAPTURE_FLAGS = PSS_CAPTURE_FLAGS(16u32);
pub const PSS_CAPTURE_HANDLE_TYPE_SPECIFIC_INFORMATION: PSS_CAPTURE_FLAGS = PSS_CAPTURE_FLAGS(32u32);
pub const PSS_CAPTURE_HANDLE_TRACE: PSS_CAPTURE_FLAGS = PSS_CAPTURE_FLAGS(64u32);
pub const PSS_CAPTURE_THREADS: PSS_CAPTURE_FLAGS = PSS_CAPTURE_FLAGS(128u32);
pub const PSS_CAPTURE_THREAD_CONTEXT: PSS_CAPTURE_FLAGS = PSS_CAPTURE_FLAGS(256u32);
pub const PSS_CAPTURE_THREAD_CONTEXT_EXTENDED: PSS_CAPTURE_FLAGS = PSS_CAPTURE_FLAGS(512u32);
pub const PSS_CAPTURE_RESERVED_00000400: PSS_CAPTURE_FLAGS = PSS_CAPTURE_FLAGS(1024u32);
pub const PSS_CAPTURE_VA_SPACE: PSS_CAPTURE_FLAGS = PSS_CAPTURE_FLAGS(2048u32);
pub const PSS_CAPTURE_VA_SPACE_SECTION_INFORMATION: PSS_CAPTURE_FLAGS = PSS_CAPTURE_FLAGS(4096u32);
pub const PSS_CAPTURE_IPT_TRACE: PSS_CAPTURE_FLAGS = PSS_CAPTURE_FLAGS(8192u32);
pub const PSS_CAPTURE_RESERVED_00004000: PSS_CAPTURE_FLAGS = PSS_CAPTURE_FLAGS(16384u32);
pub const PSS_CREATE_BREAKAWAY_OPTIONAL: PSS_CAPTURE_FLAGS = PSS_CAPTURE_FLAGS(67108864u32);
pub const PSS_CREATE_BREAKAWAY: PSS_CAPTURE_FLAGS = PSS_CAPTURE_FLAGS(134217728u32);
pub const PSS_CREATE_FORCE_BREAKAWAY: PSS_CAPTURE_FLAGS = PSS_CAPTURE_FLAGS(268435456u32);
pub const PSS_CREATE_USE_VM_ALLOCATIONS: PSS_CAPTURE_FLAGS = PSS_CAPTURE_FLAGS(536870912u32);
pub const PSS_CREATE_MEASURE_PERFORMANCE: PSS_CAPTURE_FLAGS = PSS_CAPTURE_FLAGS(1073741824u32);
pub const PSS_CREATE_RELEASE_SECTION: PSS_CAPTURE_FLAGS = PSS_CAPTURE_FLAGS(2147483648u32);
impl ::std::convert::From<u32> for PSS_CAPTURE_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PSS_CAPTURE_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for PSS_CAPTURE_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for PSS_CAPTURE_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for PSS_CAPTURE_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for PSS_CAPTURE_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for PSS_CAPTURE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PSS_DUPLICATE_FLAGS(pub u32);
pub const PSS_DUPLICATE_NONE: PSS_DUPLICATE_FLAGS = PSS_DUPLICATE_FLAGS(0u32);
pub const PSS_DUPLICATE_CLOSE_SOURCE: PSS_DUPLICATE_FLAGS = PSS_DUPLICATE_FLAGS(1u32);
impl ::std::convert::From<u32> for PSS_DUPLICATE_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PSS_DUPLICATE_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for PSS_DUPLICATE_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for PSS_DUPLICATE_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for PSS_DUPLICATE_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for PSS_DUPLICATE_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for PSS_DUPLICATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PSS_HANDLE_ENTRY {
    pub Handle: super::super::super::Foundation::HANDLE,
    pub Flags: PSS_HANDLE_FLAGS,
    pub ObjectType: PSS_OBJECT_TYPE,
    pub CaptureTime: super::super::super::Foundation::FILETIME,
    pub Attributes: u32,
    pub GrantedAccess: u32,
    pub HandleCount: u32,
    pub PointerCount: u32,
    pub PagedPoolCharge: u32,
    pub NonPagedPoolCharge: u32,
    pub CreationTime: super::super::super::Foundation::FILETIME,
    pub TypeNameLength: u16,
    pub TypeName: super::super::super::Foundation::PWSTR,
    pub ObjectNameLength: u16,
    pub ObjectName: super::super::super::Foundation::PWSTR,
    pub TypeSpecificInformation: PSS_HANDLE_ENTRY_0,
}
#[cfg(feature = "Win32_Foundation")]
impl PSS_HANDLE_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PSS_HANDLE_ENTRY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PSS_HANDLE_ENTRY {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PSS_HANDLE_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PSS_HANDLE_ENTRY {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union PSS_HANDLE_ENTRY_0 {
    pub Process: PSS_HANDLE_ENTRY_0_2,
    pub Thread: PSS_HANDLE_ENTRY_0_5,
    pub Mutant: PSS_HANDLE_ENTRY_0_1,
    pub Event: PSS_HANDLE_ENTRY_0_0,
    pub Section: PSS_HANDLE_ENTRY_0_3,
    pub Semaphore: PSS_HANDLE_ENTRY_0_4,
}
#[cfg(feature = "Win32_Foundation")]
impl PSS_HANDLE_ENTRY_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PSS_HANDLE_ENTRY_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PSS_HANDLE_ENTRY_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PSS_HANDLE_ENTRY_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PSS_HANDLE_ENTRY_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PSS_HANDLE_ENTRY_0_0 {
    pub ManualReset: super::super::super::Foundation::BOOL,
    pub Signaled: super::super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl PSS_HANDLE_ENTRY_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PSS_HANDLE_ENTRY_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for PSS_HANDLE_ENTRY_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Event_e__Struct").field("ManualReset", &self.ManualReset).field("Signaled", &self.Signaled).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PSS_HANDLE_ENTRY_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.ManualReset == other.ManualReset && self.Signaled == other.Signaled
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PSS_HANDLE_ENTRY_0_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PSS_HANDLE_ENTRY_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PSS_HANDLE_ENTRY_0_1 {
    pub CurrentCount: i32,
    pub Abandoned: super::super::super::Foundation::BOOL,
    pub OwnerProcessId: u32,
    pub OwnerThreadId: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl PSS_HANDLE_ENTRY_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PSS_HANDLE_ENTRY_0_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for PSS_HANDLE_ENTRY_0_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Mutant_e__Struct").field("CurrentCount", &self.CurrentCount).field("Abandoned", &self.Abandoned).field("OwnerProcessId", &self.OwnerProcessId).field("OwnerThreadId", &self.OwnerThreadId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PSS_HANDLE_ENTRY_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.CurrentCount == other.CurrentCount && self.Abandoned == other.Abandoned && self.OwnerProcessId == other.OwnerProcessId && self.OwnerThreadId == other.OwnerThreadId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PSS_HANDLE_ENTRY_0_1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PSS_HANDLE_ENTRY_0_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PSS_HANDLE_ENTRY_0_2 {
    pub ExitStatus: u32,
    pub PebBaseAddress: *mut ::std::ffi::c_void,
    pub AffinityMask: usize,
    pub BasePriority: i32,
    pub ProcessId: u32,
    pub ParentProcessId: u32,
    pub Flags: u32,
}
impl PSS_HANDLE_ENTRY_0_2 {}
impl ::std::default::Default for PSS_HANDLE_ENTRY_0_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PSS_HANDLE_ENTRY_0_2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Process_e__Struct")
            .field("ExitStatus", &self.ExitStatus)
            .field("PebBaseAddress", &self.PebBaseAddress)
            .field("AffinityMask", &self.AffinityMask)
            .field("BasePriority", &self.BasePriority)
            .field("ProcessId", &self.ProcessId)
            .field("ParentProcessId", &self.ParentProcessId)
            .field("Flags", &self.Flags)
            .finish()
    }
}
impl ::std::cmp::PartialEq for PSS_HANDLE_ENTRY_0_2 {
    fn eq(&self, other: &Self) -> bool {
        self.ExitStatus == other.ExitStatus && self.PebBaseAddress == other.PebBaseAddress && self.AffinityMask == other.AffinityMask && self.BasePriority == other.BasePriority && self.ProcessId == other.ProcessId && self.ParentProcessId == other.ParentProcessId && self.Flags == other.Flags
    }
}
impl ::std::cmp::Eq for PSS_HANDLE_ENTRY_0_2 {}
unsafe impl ::windows::runtime::Abi for PSS_HANDLE_ENTRY_0_2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PSS_HANDLE_ENTRY_0_3 {
    pub BaseAddress: *mut ::std::ffi::c_void,
    pub AllocationAttributes: u32,
    pub MaximumSize: i64,
}
impl PSS_HANDLE_ENTRY_0_3 {}
impl ::std::default::Default for PSS_HANDLE_ENTRY_0_3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PSS_HANDLE_ENTRY_0_3 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Section_e__Struct").field("BaseAddress", &self.BaseAddress).field("AllocationAttributes", &self.AllocationAttributes).field("MaximumSize", &self.MaximumSize).finish()
    }
}
impl ::std::cmp::PartialEq for PSS_HANDLE_ENTRY_0_3 {
    fn eq(&self, other: &Self) -> bool {
        self.BaseAddress == other.BaseAddress && self.AllocationAttributes == other.AllocationAttributes && self.MaximumSize == other.MaximumSize
    }
}
impl ::std::cmp::Eq for PSS_HANDLE_ENTRY_0_3 {}
unsafe impl ::windows::runtime::Abi for PSS_HANDLE_ENTRY_0_3 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PSS_HANDLE_ENTRY_0_4 {
    pub CurrentCount: i32,
    pub MaximumCount: i32,
}
impl PSS_HANDLE_ENTRY_0_4 {}
impl ::std::default::Default for PSS_HANDLE_ENTRY_0_4 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PSS_HANDLE_ENTRY_0_4 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Semaphore_e__Struct").field("CurrentCount", &self.CurrentCount).field("MaximumCount", &self.MaximumCount).finish()
    }
}
impl ::std::cmp::PartialEq for PSS_HANDLE_ENTRY_0_4 {
    fn eq(&self, other: &Self) -> bool {
        self.CurrentCount == other.CurrentCount && self.MaximumCount == other.MaximumCount
    }
}
impl ::std::cmp::Eq for PSS_HANDLE_ENTRY_0_4 {}
unsafe impl ::windows::runtime::Abi for PSS_HANDLE_ENTRY_0_4 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PSS_HANDLE_ENTRY_0_5 {
    pub ExitStatus: u32,
    pub TebBaseAddress: *mut ::std::ffi::c_void,
    pub ProcessId: u32,
    pub ThreadId: u32,
    pub AffinityMask: usize,
    pub Priority: i32,
    pub BasePriority: i32,
    pub Win32StartAddress: *mut ::std::ffi::c_void,
}
impl PSS_HANDLE_ENTRY_0_5 {}
impl ::std::default::Default for PSS_HANDLE_ENTRY_0_5 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PSS_HANDLE_ENTRY_0_5 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Thread_e__Struct")
            .field("ExitStatus", &self.ExitStatus)
            .field("TebBaseAddress", &self.TebBaseAddress)
            .field("ProcessId", &self.ProcessId)
            .field("ThreadId", &self.ThreadId)
            .field("AffinityMask", &self.AffinityMask)
            .field("Priority", &self.Priority)
            .field("BasePriority", &self.BasePriority)
            .field("Win32StartAddress", &self.Win32StartAddress)
            .finish()
    }
}
impl ::std::cmp::PartialEq for PSS_HANDLE_ENTRY_0_5 {
    fn eq(&self, other: &Self) -> bool {
        self.ExitStatus == other.ExitStatus && self.TebBaseAddress == other.TebBaseAddress && self.ProcessId == other.ProcessId && self.ThreadId == other.ThreadId && self.AffinityMask == other.AffinityMask && self.Priority == other.Priority && self.BasePriority == other.BasePriority && self.Win32StartAddress == other.Win32StartAddress
    }
}
impl ::std::cmp::Eq for PSS_HANDLE_ENTRY_0_5 {}
unsafe impl ::windows::runtime::Abi for PSS_HANDLE_ENTRY_0_5 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PSS_HANDLE_FLAGS(pub u32);
pub const PSS_HANDLE_NONE: PSS_HANDLE_FLAGS = PSS_HANDLE_FLAGS(0u32);
pub const PSS_HANDLE_HAVE_TYPE: PSS_HANDLE_FLAGS = PSS_HANDLE_FLAGS(1u32);
pub const PSS_HANDLE_HAVE_NAME: PSS_HANDLE_FLAGS = PSS_HANDLE_FLAGS(2u32);
pub const PSS_HANDLE_HAVE_BASIC_INFORMATION: PSS_HANDLE_FLAGS = PSS_HANDLE_FLAGS(4u32);
pub const PSS_HANDLE_HAVE_TYPE_SPECIFIC_INFORMATION: PSS_HANDLE_FLAGS = PSS_HANDLE_FLAGS(8u32);
impl ::std::convert::From<u32> for PSS_HANDLE_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PSS_HANDLE_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for PSS_HANDLE_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for PSS_HANDLE_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for PSS_HANDLE_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for PSS_HANDLE_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for PSS_HANDLE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PSS_HANDLE_INFORMATION {
    pub HandlesCaptured: u32,
}
impl PSS_HANDLE_INFORMATION {}
impl ::std::default::Default for PSS_HANDLE_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PSS_HANDLE_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PSS_HANDLE_INFORMATION").field("HandlesCaptured", &self.HandlesCaptured).finish()
    }
}
impl ::std::cmp::PartialEq for PSS_HANDLE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.HandlesCaptured == other.HandlesCaptured
    }
}
impl ::std::cmp::Eq for PSS_HANDLE_INFORMATION {}
unsafe impl ::windows::runtime::Abi for PSS_HANDLE_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PSS_HANDLE_TRACE_INFORMATION {
    pub SectionHandle: super::super::super::Foundation::HANDLE,
    pub Size: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl PSS_HANDLE_TRACE_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PSS_HANDLE_TRACE_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for PSS_HANDLE_TRACE_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PSS_HANDLE_TRACE_INFORMATION").field("SectionHandle", &self.SectionHandle).field("Size", &self.Size).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PSS_HANDLE_TRACE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.SectionHandle == other.SectionHandle && self.Size == other.Size
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PSS_HANDLE_TRACE_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PSS_HANDLE_TRACE_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PSS_OBJECT_TYPE(pub i32);
pub const PSS_OBJECT_TYPE_UNKNOWN: PSS_OBJECT_TYPE = PSS_OBJECT_TYPE(0i32);
pub const PSS_OBJECT_TYPE_PROCESS: PSS_OBJECT_TYPE = PSS_OBJECT_TYPE(1i32);
pub const PSS_OBJECT_TYPE_THREAD: PSS_OBJECT_TYPE = PSS_OBJECT_TYPE(2i32);
pub const PSS_OBJECT_TYPE_MUTANT: PSS_OBJECT_TYPE = PSS_OBJECT_TYPE(3i32);
pub const PSS_OBJECT_TYPE_EVENT: PSS_OBJECT_TYPE = PSS_OBJECT_TYPE(4i32);
pub const PSS_OBJECT_TYPE_SECTION: PSS_OBJECT_TYPE = PSS_OBJECT_TYPE(5i32);
pub const PSS_OBJECT_TYPE_SEMAPHORE: PSS_OBJECT_TYPE = PSS_OBJECT_TYPE(6i32);
impl ::std::convert::From<i32> for PSS_OBJECT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PSS_OBJECT_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PSS_PERFORMANCE_COUNTERS {
    pub TotalCycleCount: u64,
    pub TotalWallClockPeriod: u64,
    pub VaCloneCycleCount: u64,
    pub VaCloneWallClockPeriod: u64,
    pub VaSpaceCycleCount: u64,
    pub VaSpaceWallClockPeriod: u64,
    pub AuxPagesCycleCount: u64,
    pub AuxPagesWallClockPeriod: u64,
    pub HandlesCycleCount: u64,
    pub HandlesWallClockPeriod: u64,
    pub ThreadsCycleCount: u64,
    pub ThreadsWallClockPeriod: u64,
}
impl PSS_PERFORMANCE_COUNTERS {}
impl ::std::default::Default for PSS_PERFORMANCE_COUNTERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PSS_PERFORMANCE_COUNTERS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PSS_PERFORMANCE_COUNTERS")
            .field("TotalCycleCount", &self.TotalCycleCount)
            .field("TotalWallClockPeriod", &self.TotalWallClockPeriod)
            .field("VaCloneCycleCount", &self.VaCloneCycleCount)
            .field("VaCloneWallClockPeriod", &self.VaCloneWallClockPeriod)
            .field("VaSpaceCycleCount", &self.VaSpaceCycleCount)
            .field("VaSpaceWallClockPeriod", &self.VaSpaceWallClockPeriod)
            .field("AuxPagesCycleCount", &self.AuxPagesCycleCount)
            .field("AuxPagesWallClockPeriod", &self.AuxPagesWallClockPeriod)
            .field("HandlesCycleCount", &self.HandlesCycleCount)
            .field("HandlesWallClockPeriod", &self.HandlesWallClockPeriod)
            .field("ThreadsCycleCount", &self.ThreadsCycleCount)
            .field("ThreadsWallClockPeriod", &self.ThreadsWallClockPeriod)
            .finish()
    }
}
impl ::std::cmp::PartialEq for PSS_PERFORMANCE_COUNTERS {
    fn eq(&self, other: &Self) -> bool {
        self.TotalCycleCount == other.TotalCycleCount
            && self.TotalWallClockPeriod == other.TotalWallClockPeriod
            && self.VaCloneCycleCount == other.VaCloneCycleCount
            && self.VaCloneWallClockPeriod == other.VaCloneWallClockPeriod
            && self.VaSpaceCycleCount == other.VaSpaceCycleCount
            && self.VaSpaceWallClockPeriod == other.VaSpaceWallClockPeriod
            && self.AuxPagesCycleCount == other.AuxPagesCycleCount
            && self.AuxPagesWallClockPeriod == other.AuxPagesWallClockPeriod
            && self.HandlesCycleCount == other.HandlesCycleCount
            && self.HandlesWallClockPeriod == other.HandlesWallClockPeriod
            && self.ThreadsCycleCount == other.ThreadsCycleCount
            && self.ThreadsWallClockPeriod == other.ThreadsWallClockPeriod
    }
}
impl ::std::cmp::Eq for PSS_PERFORMANCE_COUNTERS {}
unsafe impl ::windows::runtime::Abi for PSS_PERFORMANCE_COUNTERS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const PSS_PERF_RESOLUTION: u32 = 1000000u32;
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PSS_PROCESS_FLAGS(pub u32);
pub const PSS_PROCESS_FLAGS_NONE: PSS_PROCESS_FLAGS = PSS_PROCESS_FLAGS(0u32);
pub const PSS_PROCESS_FLAGS_PROTECTED: PSS_PROCESS_FLAGS = PSS_PROCESS_FLAGS(1u32);
pub const PSS_PROCESS_FLAGS_WOW64: PSS_PROCESS_FLAGS = PSS_PROCESS_FLAGS(2u32);
pub const PSS_PROCESS_FLAGS_RESERVED_03: PSS_PROCESS_FLAGS = PSS_PROCESS_FLAGS(4u32);
pub const PSS_PROCESS_FLAGS_RESERVED_04: PSS_PROCESS_FLAGS = PSS_PROCESS_FLAGS(8u32);
pub const PSS_PROCESS_FLAGS_FROZEN: PSS_PROCESS_FLAGS = PSS_PROCESS_FLAGS(16u32);
impl ::std::convert::From<u32> for PSS_PROCESS_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PSS_PROCESS_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for PSS_PROCESS_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for PSS_PROCESS_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for PSS_PROCESS_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for PSS_PROCESS_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for PSS_PROCESS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PSS_PROCESS_INFORMATION {
    pub ExitStatus: u32,
    pub PebBaseAddress: *mut ::std::ffi::c_void,
    pub AffinityMask: usize,
    pub BasePriority: i32,
    pub ProcessId: u32,
    pub ParentProcessId: u32,
    pub Flags: PSS_PROCESS_FLAGS,
    pub CreateTime: super::super::super::Foundation::FILETIME,
    pub ExitTime: super::super::super::Foundation::FILETIME,
    pub KernelTime: super::super::super::Foundation::FILETIME,
    pub UserTime: super::super::super::Foundation::FILETIME,
    pub PriorityClass: u32,
    pub PeakVirtualSize: usize,
    pub VirtualSize: usize,
    pub PageFaultCount: u32,
    pub PeakWorkingSetSize: usize,
    pub WorkingSetSize: usize,
    pub QuotaPeakPagedPoolUsage: usize,
    pub QuotaPagedPoolUsage: usize,
    pub QuotaPeakNonPagedPoolUsage: usize,
    pub QuotaNonPagedPoolUsage: usize,
    pub PagefileUsage: usize,
    pub PeakPagefileUsage: usize,
    pub PrivateUsage: usize,
    pub ExecuteFlags: u32,
    pub ImageFileName: [u16; 260],
}
#[cfg(feature = "Win32_Foundation")]
impl PSS_PROCESS_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PSS_PROCESS_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for PSS_PROCESS_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PSS_PROCESS_INFORMATION")
            .field("ExitStatus", &self.ExitStatus)
            .field("PebBaseAddress", &self.PebBaseAddress)
            .field("AffinityMask", &self.AffinityMask)
            .field("BasePriority", &self.BasePriority)
            .field("ProcessId", &self.ProcessId)
            .field("ParentProcessId", &self.ParentProcessId)
            .field("Flags", &self.Flags)
            .field("CreateTime", &self.CreateTime)
            .field("ExitTime", &self.ExitTime)
            .field("KernelTime", &self.KernelTime)
            .field("UserTime", &self.UserTime)
            .field("PriorityClass", &self.PriorityClass)
            .field("PeakVirtualSize", &self.PeakVirtualSize)
            .field("VirtualSize", &self.VirtualSize)
            .field("PageFaultCount", &self.PageFaultCount)
            .field("PeakWorkingSetSize", &self.PeakWorkingSetSize)
            .field("WorkingSetSize", &self.WorkingSetSize)
            .field("QuotaPeakPagedPoolUsage", &self.QuotaPeakPagedPoolUsage)
            .field("QuotaPagedPoolUsage", &self.QuotaPagedPoolUsage)
            .field("QuotaPeakNonPagedPoolUsage", &self.QuotaPeakNonPagedPoolUsage)
            .field("QuotaNonPagedPoolUsage", &self.QuotaNonPagedPoolUsage)
            .field("PagefileUsage", &self.PagefileUsage)
            .field("PeakPagefileUsage", &self.PeakPagefileUsage)
            .field("PrivateUsage", &self.PrivateUsage)
            .field("ExecuteFlags", &self.ExecuteFlags)
            .field("ImageFileName", &self.ImageFileName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PSS_PROCESS_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.ExitStatus == other.ExitStatus
            && self.PebBaseAddress == other.PebBaseAddress
            && self.AffinityMask == other.AffinityMask
            && self.BasePriority == other.BasePriority
            && self.ProcessId == other.ProcessId
            && self.ParentProcessId == other.ParentProcessId
            && self.Flags == other.Flags
            && self.CreateTime == other.CreateTime
            && self.ExitTime == other.ExitTime
            && self.KernelTime == other.KernelTime
            && self.UserTime == other.UserTime
            && self.PriorityClass == other.PriorityClass
            && self.PeakVirtualSize == other.PeakVirtualSize
            && self.VirtualSize == other.VirtualSize
            && self.PageFaultCount == other.PageFaultCount
            && self.PeakWorkingSetSize == other.PeakWorkingSetSize
            && self.WorkingSetSize == other.WorkingSetSize
            && self.QuotaPeakPagedPoolUsage == other.QuotaPeakPagedPoolUsage
            && self.QuotaPagedPoolUsage == other.QuotaPagedPoolUsage
            && self.QuotaPeakNonPagedPoolUsage == other.QuotaPeakNonPagedPoolUsage
            && self.QuotaNonPagedPoolUsage == other.QuotaNonPagedPoolUsage
            && self.PagefileUsage == other.PagefileUsage
            && self.PeakPagefileUsage == other.PeakPagefileUsage
            && self.PrivateUsage == other.PrivateUsage
            && self.ExecuteFlags == other.ExecuteFlags
            && self.ImageFileName == other.ImageFileName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PSS_PROCESS_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PSS_PROCESS_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PSS_QUERY_INFORMATION_CLASS(pub i32);
pub const PSS_QUERY_PROCESS_INFORMATION: PSS_QUERY_INFORMATION_CLASS = PSS_QUERY_INFORMATION_CLASS(0i32);
pub const PSS_QUERY_VA_CLONE_INFORMATION: PSS_QUERY_INFORMATION_CLASS = PSS_QUERY_INFORMATION_CLASS(1i32);
pub const PSS_QUERY_AUXILIARY_PAGES_INFORMATION: PSS_QUERY_INFORMATION_CLASS = PSS_QUERY_INFORMATION_CLASS(2i32);
pub const PSS_QUERY_VA_SPACE_INFORMATION: PSS_QUERY_INFORMATION_CLASS = PSS_QUERY_INFORMATION_CLASS(3i32);
pub const PSS_QUERY_HANDLE_INFORMATION: PSS_QUERY_INFORMATION_CLASS = PSS_QUERY_INFORMATION_CLASS(4i32);
pub const PSS_QUERY_THREAD_INFORMATION: PSS_QUERY_INFORMATION_CLASS = PSS_QUERY_INFORMATION_CLASS(5i32);
pub const PSS_QUERY_HANDLE_TRACE_INFORMATION: PSS_QUERY_INFORMATION_CLASS = PSS_QUERY_INFORMATION_CLASS(6i32);
pub const PSS_QUERY_PERFORMANCE_COUNTERS: PSS_QUERY_INFORMATION_CLASS = PSS_QUERY_INFORMATION_CLASS(7i32);
impl ::std::convert::From<i32> for PSS_QUERY_INFORMATION_CLASS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PSS_QUERY_INFORMATION_CLASS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_SystemServices"))]
pub struct PSS_THREAD_ENTRY {
    pub ExitStatus: u32,
    pub TebBaseAddress: *mut ::std::ffi::c_void,
    pub ProcessId: u32,
    pub ThreadId: u32,
    pub AffinityMask: usize,
    pub Priority: i32,
    pub BasePriority: i32,
    pub LastSyscallFirstArgument: *mut ::std::ffi::c_void,
    pub LastSyscallNumber: u16,
    pub CreateTime: super::super::super::Foundation::FILETIME,
    pub ExitTime: super::super::super::Foundation::FILETIME,
    pub KernelTime: super::super::super::Foundation::FILETIME,
    pub UserTime: super::super::super::Foundation::FILETIME,
    pub Win32StartAddress: *mut ::std::ffi::c_void,
    pub CaptureTime: super::super::super::Foundation::FILETIME,
    pub Flags: PSS_THREAD_FLAGS,
    pub SuspendCount: u16,
    pub SizeOfContextRecord: u16,
    pub ContextRecord: *mut super::Debug::CONTEXT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_SystemServices"))]
impl PSS_THREAD_ENTRY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_SystemServices"))]
impl ::std::default::Default for PSS_THREAD_ENTRY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_SystemServices"))]
impl ::std::fmt::Debug for PSS_THREAD_ENTRY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PSS_THREAD_ENTRY")
            .field("ExitStatus", &self.ExitStatus)
            .field("TebBaseAddress", &self.TebBaseAddress)
            .field("ProcessId", &self.ProcessId)
            .field("ThreadId", &self.ThreadId)
            .field("AffinityMask", &self.AffinityMask)
            .field("Priority", &self.Priority)
            .field("BasePriority", &self.BasePriority)
            .field("LastSyscallFirstArgument", &self.LastSyscallFirstArgument)
            .field("LastSyscallNumber", &self.LastSyscallNumber)
            .field("CreateTime", &self.CreateTime)
            .field("ExitTime", &self.ExitTime)
            .field("KernelTime", &self.KernelTime)
            .field("UserTime", &self.UserTime)
            .field("Win32StartAddress", &self.Win32StartAddress)
            .field("CaptureTime", &self.CaptureTime)
            .field("Flags", &self.Flags)
            .field("SuspendCount", &self.SuspendCount)
            .field("SizeOfContextRecord", &self.SizeOfContextRecord)
            .field("ContextRecord", &self.ContextRecord)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::PartialEq for PSS_THREAD_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.ExitStatus == other.ExitStatus
            && self.TebBaseAddress == other.TebBaseAddress
            && self.ProcessId == other.ProcessId
            && self.ThreadId == other.ThreadId
            && self.AffinityMask == other.AffinityMask
            && self.Priority == other.Priority
            && self.BasePriority == other.BasePriority
            && self.LastSyscallFirstArgument == other.LastSyscallFirstArgument
            && self.LastSyscallNumber == other.LastSyscallNumber
            && self.CreateTime == other.CreateTime
            && self.ExitTime == other.ExitTime
            && self.KernelTime == other.KernelTime
            && self.UserTime == other.UserTime
            && self.Win32StartAddress == other.Win32StartAddress
            && self.CaptureTime == other.CaptureTime
            && self.Flags == other.Flags
            && self.SuspendCount == other.SuspendCount
            && self.SizeOfContextRecord == other.SizeOfContextRecord
            && self.ContextRecord == other.ContextRecord
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::Eq for PSS_THREAD_ENTRY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::runtime::Abi for PSS_THREAD_ENTRY {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PSS_THREAD_FLAGS(pub u32);
pub const PSS_THREAD_FLAGS_NONE: PSS_THREAD_FLAGS = PSS_THREAD_FLAGS(0u32);
pub const PSS_THREAD_FLAGS_TERMINATED: PSS_THREAD_FLAGS = PSS_THREAD_FLAGS(1u32);
impl ::std::convert::From<u32> for PSS_THREAD_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PSS_THREAD_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for PSS_THREAD_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for PSS_THREAD_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for PSS_THREAD_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for PSS_THREAD_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for PSS_THREAD_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PSS_THREAD_INFORMATION {
    pub ThreadsCaptured: u32,
    pub ContextLength: u32,
}
impl PSS_THREAD_INFORMATION {}
impl ::std::default::Default for PSS_THREAD_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PSS_THREAD_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PSS_THREAD_INFORMATION").field("ThreadsCaptured", &self.ThreadsCaptured).field("ContextLength", &self.ContextLength).finish()
    }
}
impl ::std::cmp::PartialEq for PSS_THREAD_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.ThreadsCaptured == other.ThreadsCaptured && self.ContextLength == other.ContextLength
    }
}
impl ::std::cmp::Eq for PSS_THREAD_INFORMATION {}
unsafe impl ::windows::runtime::Abi for PSS_THREAD_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PSS_VA_CLONE_INFORMATION {
    pub VaCloneHandle: super::super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl PSS_VA_CLONE_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PSS_VA_CLONE_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for PSS_VA_CLONE_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PSS_VA_CLONE_INFORMATION").field("VaCloneHandle", &self.VaCloneHandle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PSS_VA_CLONE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.VaCloneHandle == other.VaCloneHandle
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PSS_VA_CLONE_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PSS_VA_CLONE_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PSS_VA_SPACE_ENTRY {
    pub BaseAddress: *mut ::std::ffi::c_void,
    pub AllocationBase: *mut ::std::ffi::c_void,
    pub AllocationProtect: u32,
    pub RegionSize: usize,
    pub State: u32,
    pub Protect: u32,
    pub Type: u32,
    pub TimeDateStamp: u32,
    pub SizeOfImage: u32,
    pub ImageBase: *mut ::std::ffi::c_void,
    pub CheckSum: u32,
    pub MappedFileNameLength: u16,
    pub MappedFileName: super::super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl PSS_VA_SPACE_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PSS_VA_SPACE_ENTRY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for PSS_VA_SPACE_ENTRY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PSS_VA_SPACE_ENTRY")
            .field("BaseAddress", &self.BaseAddress)
            .field("AllocationBase", &self.AllocationBase)
            .field("AllocationProtect", &self.AllocationProtect)
            .field("RegionSize", &self.RegionSize)
            .field("State", &self.State)
            .field("Protect", &self.Protect)
            .field("Type", &self.Type)
            .field("TimeDateStamp", &self.TimeDateStamp)
            .field("SizeOfImage", &self.SizeOfImage)
            .field("ImageBase", &self.ImageBase)
            .field("CheckSum", &self.CheckSum)
            .field("MappedFileNameLength", &self.MappedFileNameLength)
            .field("MappedFileName", &self.MappedFileName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PSS_VA_SPACE_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.BaseAddress == other.BaseAddress
            && self.AllocationBase == other.AllocationBase
            && self.AllocationProtect == other.AllocationProtect
            && self.RegionSize == other.RegionSize
            && self.State == other.State
            && self.Protect == other.Protect
            && self.Type == other.Type
            && self.TimeDateStamp == other.TimeDateStamp
            && self.SizeOfImage == other.SizeOfImage
            && self.ImageBase == other.ImageBase
            && self.CheckSum == other.CheckSum
            && self.MappedFileNameLength == other.MappedFileNameLength
            && self.MappedFileName == other.MappedFileName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PSS_VA_SPACE_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PSS_VA_SPACE_ENTRY {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PSS_VA_SPACE_INFORMATION {
    pub RegionCount: u32,
}
impl PSS_VA_SPACE_INFORMATION {}
impl ::std::default::Default for PSS_VA_SPACE_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PSS_VA_SPACE_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PSS_VA_SPACE_INFORMATION").field("RegionCount", &self.RegionCount).finish()
    }
}
impl ::std::cmp::PartialEq for PSS_VA_SPACE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.RegionCount == other.RegionCount
    }
}
impl ::std::cmp::Eq for PSS_VA_SPACE_INFORMATION {}
unsafe impl ::windows::runtime::Abi for PSS_VA_SPACE_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PSS_WALK_INFORMATION_CLASS(pub i32);
pub const PSS_WALK_AUXILIARY_PAGES: PSS_WALK_INFORMATION_CLASS = PSS_WALK_INFORMATION_CLASS(0i32);
pub const PSS_WALK_VA_SPACE: PSS_WALK_INFORMATION_CLASS = PSS_WALK_INFORMATION_CLASS(1i32);
pub const PSS_WALK_HANDLES: PSS_WALK_INFORMATION_CLASS = PSS_WALK_INFORMATION_CLASS(2i32);
pub const PSS_WALK_THREADS: PSS_WALK_INFORMATION_CLASS = PSS_WALK_INFORMATION_CLASS(3i32);
impl ::std::convert::From<i32> for PSS_WALK_INFORMATION_CLASS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PSS_WALK_INFORMATION_CLASS {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PssCaptureSnapshot<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(processhandle: Param0, captureflags: PSS_CAPTURE_FLAGS, threadcontextflags: u32, snapshothandle: *mut HPSS) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PssCaptureSnapshot(processhandle: super::super::super::Foundation::HANDLE, captureflags: PSS_CAPTURE_FLAGS, threadcontextflags: u32, snapshothandle: *mut HPSS) -> u32;
        }
        ::std::mem::transmute(PssCaptureSnapshot(processhandle.into_param().abi(), ::std::mem::transmute(captureflags), ::std::mem::transmute(threadcontextflags), ::std::mem::transmute(snapshothandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PssDuplicateSnapshot<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, HPSS>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(sourceprocesshandle: Param0, snapshothandle: Param1, targetprocesshandle: Param2, targetsnapshothandle: *mut HPSS, flags: PSS_DUPLICATE_FLAGS) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PssDuplicateSnapshot(sourceprocesshandle: super::super::super::Foundation::HANDLE, snapshothandle: HPSS, targetprocesshandle: super::super::super::Foundation::HANDLE, targetsnapshothandle: *mut HPSS, flags: PSS_DUPLICATE_FLAGS) -> u32;
        }
        ::std::mem::transmute(PssDuplicateSnapshot(sourceprocesshandle.into_param().abi(), snapshothandle.into_param().abi(), targetprocesshandle.into_param().abi(), ::std::mem::transmute(targetsnapshothandle), ::std::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PssFreeSnapshot<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, HPSS>>(processhandle: Param0, snapshothandle: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PssFreeSnapshot(processhandle: super::super::super::Foundation::HANDLE, snapshothandle: HPSS) -> u32;
        }
        ::std::mem::transmute(PssFreeSnapshot(processhandle.into_param().abi(), snapshothandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PssQuerySnapshot<'a, Param0: ::windows::runtime::IntoParam<'a, HPSS>>(snapshothandle: Param0, informationclass: PSS_QUERY_INFORMATION_CLASS, buffer: *mut ::std::ffi::c_void, bufferlength: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PssQuerySnapshot(snapshothandle: HPSS, informationclass: PSS_QUERY_INFORMATION_CLASS, buffer: *mut ::std::ffi::c_void, bufferlength: u32) -> u32;
        }
        ::std::mem::transmute(PssQuerySnapshot(snapshothandle.into_param().abi(), ::std::mem::transmute(informationclass), ::std::mem::transmute(buffer), ::std::mem::transmute(bufferlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PssWalkMarkerCreate(allocator: *const PSS_ALLOCATOR, walkmarkerhandle: *mut HPSSWALK) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PssWalkMarkerCreate(allocator: *const PSS_ALLOCATOR, walkmarkerhandle: *mut HPSSWALK) -> u32;
        }
        ::std::mem::transmute(PssWalkMarkerCreate(::std::mem::transmute(allocator), ::std::mem::transmute(walkmarkerhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PssWalkMarkerFree<'a, Param0: ::windows::runtime::IntoParam<'a, HPSSWALK>>(walkmarkerhandle: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PssWalkMarkerFree(walkmarkerhandle: HPSSWALK) -> u32;
        }
        ::std::mem::transmute(PssWalkMarkerFree(walkmarkerhandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PssWalkMarkerGetPosition<'a, Param0: ::windows::runtime::IntoParam<'a, HPSSWALK>>(walkmarkerhandle: Param0, position: *mut usize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PssWalkMarkerGetPosition(walkmarkerhandle: HPSSWALK, position: *mut usize) -> u32;
        }
        ::std::mem::transmute(PssWalkMarkerGetPosition(walkmarkerhandle.into_param().abi(), ::std::mem::transmute(position)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PssWalkMarkerSeekToBeginning<'a, Param0: ::windows::runtime::IntoParam<'a, HPSSWALK>>(walkmarkerhandle: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PssWalkMarkerSeekToBeginning(walkmarkerhandle: HPSSWALK) -> u32;
        }
        ::std::mem::transmute(PssWalkMarkerSeekToBeginning(walkmarkerhandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PssWalkMarkerSetPosition<'a, Param0: ::windows::runtime::IntoParam<'a, HPSSWALK>>(walkmarkerhandle: Param0, position: usize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PssWalkMarkerSetPosition(walkmarkerhandle: HPSSWALK, position: usize) -> u32;
        }
        ::std::mem::transmute(PssWalkMarkerSetPosition(walkmarkerhandle.into_param().abi(), ::std::mem::transmute(position)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PssWalkSnapshot<'a, Param0: ::windows::runtime::IntoParam<'a, HPSS>, Param2: ::windows::runtime::IntoParam<'a, HPSSWALK>>(snapshothandle: Param0, informationclass: PSS_WALK_INFORMATION_CLASS, walkmarkerhandle: Param2, buffer: *mut ::std::ffi::c_void, bufferlength: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PssWalkSnapshot(snapshothandle: HPSS, informationclass: PSS_WALK_INFORMATION_CLASS, walkmarkerhandle: HPSSWALK, buffer: *mut ::std::ffi::c_void, bufferlength: u32) -> u32;
        }
        ::std::mem::transmute(PssWalkSnapshot(snapshothandle.into_param().abi(), ::std::mem::transmute(informationclass), walkmarkerhandle.into_param().abi(), ::std::mem::transmute(buffer), ::std::mem::transmute(bufferlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}