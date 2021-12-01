#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[inline]
pub unsafe fn MrmAllocateBuffer(size: usize) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "mrm")]
        extern "system" {
            fn MrmAllocateBuffer(size: usize) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(MrmAllocateBuffer(::core::mem::transmute(size)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct MrmContextHandle__ {
    pub unused: i32,
}
impl MrmContextHandle__ {}
impl ::core::default::Default for MrmContextHandle__ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for MrmContextHandle__ {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("MrmContextHandle__")
            .field("unused", &self.unused)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MrmContextHandle__ {
    fn eq(&self, other: &Self) -> bool {
        self.unused == other.unused
    }
}
impl ::core::cmp::Eq for MrmContextHandle__ {}
unsafe impl ::windows::core::Abi for MrmContextHandle__ {
    type Abi = Self;
}
#[inline]
pub unsafe fn MrmCreateResourceContext(
    resourcemanager: *const MrmManagerHandle__,
) -> ::windows::core::Result<*mut MrmContextHandle__> {
    #[cfg(windows)]
    {
        #[link(name = "mrm")]
        extern "system" {
            fn MrmCreateResourceContext(
                resourcemanager: *const MrmManagerHandle__,
                resourcecontext: *mut *mut MrmContextHandle__,
            ) -> ::windows::core::HRESULT;
        }
        let mut result__: <*mut MrmContextHandle__ as ::windows::core::Abi>::Abi =
            ::core::mem::zeroed();
        MrmCreateResourceContext(::core::mem::transmute(resourcemanager), &mut result__)
            .from_abi::<*mut MrmContextHandle__>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MrmCreateResourceManager<
    'a,
    Param0: ::windows::core::IntoParam<'a, ::windows::Win32::Foundation::PWSTR>,
>(
    prifilename: Param0,
) -> ::windows::core::Result<*mut MrmManagerHandle__> {
    #[cfg(windows)]
    {
        #[link(name = "mrm")]
        extern "system" {
            fn MrmCreateResourceManager(
                prifilename: ::windows::Win32::Foundation::PWSTR,
                resourcemanager: *mut *mut MrmManagerHandle__,
            ) -> ::windows::core::HRESULT;
        }
        let mut result__: <*mut MrmManagerHandle__ as ::windows::core::Abi>::Abi =
            ::core::mem::zeroed();
        MrmCreateResourceManager(prifilename.into_param().abi(), &mut result__)
            .from_abi::<*mut MrmManagerHandle__>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MrmDestroyResourceContext(resourcecontext: *const MrmContextHandle__) {
    #[cfg(windows)]
    {
        #[link(name = "mrm")]
        extern "system" {
            fn MrmDestroyResourceContext(resourcecontext: *const MrmContextHandle__);
        }
        ::core::mem::transmute(MrmDestroyResourceContext(::core::mem::transmute(
            resourcecontext,
        )))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MrmDestroyResourceManager(resourcemanager: *const MrmManagerHandle__) {
    #[cfg(windows)]
    {
        #[link(name = "mrm")]
        extern "system" {
            fn MrmDestroyResourceManager(resourcemanager: *const MrmManagerHandle__);
        }
        ::core::mem::transmute(MrmDestroyResourceManager(::core::mem::transmute(
            resourcemanager,
        )))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MrmFreeQualifierNamesOrValues(
    size: u32,
    names: *const ::windows::Win32::Foundation::PWSTR,
) {
    #[cfg(windows)]
    {
        #[link(name = "mrm")]
        extern "system" {
            fn MrmFreeQualifierNamesOrValues(
                size: u32,
                names: *const ::windows::Win32::Foundation::PWSTR,
            );
        }
        ::core::mem::transmute(MrmFreeQualifierNamesOrValues(
            ::core::mem::transmute(size),
            ::core::mem::transmute(names),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MrmFreeResource(resource: *const ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "mrm")]
        extern "system" {
            fn MrmFreeResource(resource: *const ::core::ffi::c_void);
        }
        ::core::mem::transmute(MrmFreeResource(::core::mem::transmute(resource)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MrmGetAllQualifierNames(
    resourcecontext: *const MrmContextHandle__,
    size: *mut u32,
    names: *mut *mut ::windows::Win32::Foundation::PWSTR,
) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "mrm")]
        extern "system" {
            fn MrmGetAllQualifierNames(
                resourcecontext: *const MrmContextHandle__,
                size: *mut u32,
                names: *mut *mut ::windows::Win32::Foundation::PWSTR,
            ) -> ::windows::core::HRESULT;
        }
        MrmGetAllQualifierNames(
            ::core::mem::transmute(resourcecontext),
            ::core::mem::transmute(size),
            ::core::mem::transmute(names),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MrmGetChildResourceMap<
    'a,
    Param2: ::windows::core::IntoParam<'a, ::windows::Win32::Foundation::PWSTR>,
>(
    resourcemanager: *const MrmManagerHandle__,
    resourcemap: *const MrmMapHandle__,
    childresourcemapname: Param2,
) -> ::windows::core::Result<*mut MrmMapHandle__> {
    #[cfg(windows)]
    {
        #[link(name = "mrm")]
        extern "system" {
            fn MrmGetChildResourceMap(
                resourcemanager: *const MrmManagerHandle__,
                resourcemap: *const MrmMapHandle__,
                childresourcemapname: ::windows::Win32::Foundation::PWSTR,
                childresourcemap: *mut *mut MrmMapHandle__,
            ) -> ::windows::core::HRESULT;
        }
        let mut result__: <*mut MrmMapHandle__ as ::windows::core::Abi>::Abi =
            ::core::mem::zeroed();
        MrmGetChildResourceMap(
            ::core::mem::transmute(resourcemanager),
            ::core::mem::transmute(resourcemap),
            childresourcemapname.into_param().abi(),
            &mut result__,
        )
        .from_abi::<*mut MrmMapHandle__>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MrmGetFilePathFromName<
    'a,
    Param0: ::windows::core::IntoParam<'a, ::windows::Win32::Foundation::PWSTR>,
>(
    filename: Param0,
) -> ::windows::core::Result<::windows::Win32::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "mrm")]
        extern "system" {
            fn MrmGetFilePathFromName(
                filename: ::windows::Win32::Foundation::PWSTR,
                filepath: *mut ::windows::Win32::Foundation::PWSTR,
            ) -> ::windows::core::HRESULT;
        }
        let mut result__: <::windows::Win32::Foundation::PWSTR as ::windows::core::Abi>::Abi =
            ::core::mem::zeroed();
        MrmGetFilePathFromName(filename.into_param().abi(), &mut result__)
            .from_abi::<::windows::Win32::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MrmGetQualifier<
    'a,
    Param1: ::windows::core::IntoParam<'a, ::windows::Win32::Foundation::PWSTR>,
>(
    resourcecontext: *const MrmContextHandle__,
    qualifiername: Param1,
) -> ::windows::core::Result<::windows::Win32::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "mrm")]
        extern "system" {
            fn MrmGetQualifier(
                resourcecontext: *const MrmContextHandle__,
                qualifiername: ::windows::Win32::Foundation::PWSTR,
                qualifiervalue: *mut ::windows::Win32::Foundation::PWSTR,
            ) -> ::windows::core::HRESULT;
        }
        let mut result__: <::windows::Win32::Foundation::PWSTR as ::windows::core::Abi>::Abi =
            ::core::mem::zeroed();
        MrmGetQualifier(
            ::core::mem::transmute(resourcecontext),
            qualifiername.into_param().abi(),
            &mut result__,
        )
        .from_abi::<::windows::Win32::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MrmGetResourceCount(
    resourcemanager: *const MrmManagerHandle__,
    resourcemap: *const MrmMapHandle__,
) -> ::windows::core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "mrm")]
        extern "system" {
            fn MrmGetResourceCount(
                resourcemanager: *const MrmManagerHandle__,
                resourcemap: *const MrmMapHandle__,
                count: *mut u32,
            ) -> ::windows::core::HRESULT;
        }
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        MrmGetResourceCount(
            ::core::mem::transmute(resourcemanager),
            ::core::mem::transmute(resourcemap),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MrmLoadEmbeddedResource<
    'a,
    Param3: ::windows::core::IntoParam<'a, ::windows::Win32::Foundation::PWSTR>,
>(
    resourcemanager: *const MrmManagerHandle__,
    resourcecontext: *const MrmContextHandle__,
    resourcemap: *const MrmMapHandle__,
    resourceid: Param3,
) -> ::windows::core::Result<MrmResourceData> {
    #[cfg(windows)]
    {
        #[link(name = "mrm")]
        extern "system" {
            fn MrmLoadEmbeddedResource(
                resourcemanager: *const MrmManagerHandle__,
                resourcecontext: *const MrmContextHandle__,
                resourcemap: *const MrmMapHandle__,
                resourceid: ::windows::Win32::Foundation::PWSTR,
                data: *mut MrmResourceData,
            ) -> ::windows::core::HRESULT;
        }
        let mut result__: <MrmResourceData as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        MrmLoadEmbeddedResource(
            ::core::mem::transmute(resourcemanager),
            ::core::mem::transmute(resourcecontext),
            ::core::mem::transmute(resourcemap),
            resourceid.into_param().abi(),
            &mut result__,
        )
        .from_abi::<MrmResourceData>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MrmLoadEmbeddedResourceFromResourceUri<
    'a,
    Param2: ::windows::core::IntoParam<'a, ::windows::Win32::Foundation::PWSTR>,
>(
    resourcemanager: *const MrmManagerHandle__,
    resourcecontext: *const MrmContextHandle__,
    resourceuri: Param2,
) -> ::windows::core::Result<MrmResourceData> {
    #[cfg(windows)]
    {
        #[link(name = "mrm")]
        extern "system" {
            fn MrmLoadEmbeddedResourceFromResourceUri(
                resourcemanager: *const MrmManagerHandle__,
                resourcecontext: *const MrmContextHandle__,
                resourceuri: ::windows::Win32::Foundation::PWSTR,
                data: *mut MrmResourceData,
            ) -> ::windows::core::HRESULT;
        }
        let mut result__: <MrmResourceData as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        MrmLoadEmbeddedResourceFromResourceUri(
            ::core::mem::transmute(resourcemanager),
            ::core::mem::transmute(resourcecontext),
            resourceuri.into_param().abi(),
            &mut result__,
        )
        .from_abi::<MrmResourceData>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MrmLoadStringOrEmbeddedFromResourceUri<
    'a,
    Param2: ::windows::core::IntoParam<'a, ::windows::Win32::Foundation::PWSTR>,
>(
    resourcemanager: *const MrmManagerHandle__,
    resourcecontext: *const MrmContextHandle__,
    resourceuri: Param2,
    resourcetype: *mut MrmType,
    resourcestring: *mut ::windows::Win32::Foundation::PWSTR,
    data: *mut MrmResourceData,
) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "mrm")]
        extern "system" {
            fn MrmLoadStringOrEmbeddedFromResourceUri(
                resourcemanager: *const MrmManagerHandle__,
                resourcecontext: *const MrmContextHandle__,
                resourceuri: ::windows::Win32::Foundation::PWSTR,
                resourcetype: *mut MrmType,
                resourcestring: *mut ::windows::Win32::Foundation::PWSTR,
                data: *mut MrmResourceData,
            ) -> ::windows::core::HRESULT;
        }
        MrmLoadStringOrEmbeddedFromResourceUri(
            ::core::mem::transmute(resourcemanager),
            ::core::mem::transmute(resourcecontext),
            resourceuri.into_param().abi(),
            ::core::mem::transmute(resourcetype),
            ::core::mem::transmute(resourcestring),
            ::core::mem::transmute(data),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MrmLoadStringOrEmbeddedResource<
    'a,
    Param3: ::windows::core::IntoParam<'a, ::windows::Win32::Foundation::PWSTR>,
>(
    resourcemanager: *const MrmManagerHandle__,
    resourcecontext: *const MrmContextHandle__,
    resourcemap: *const MrmMapHandle__,
    resourceid: Param3,
    resourcetype: *mut MrmType,
    resourcestring: *mut ::windows::Win32::Foundation::PWSTR,
    data: *mut MrmResourceData,
) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "mrm")]
        extern "system" {
            fn MrmLoadStringOrEmbeddedResource(
                resourcemanager: *const MrmManagerHandle__,
                resourcecontext: *const MrmContextHandle__,
                resourcemap: *const MrmMapHandle__,
                resourceid: ::windows::Win32::Foundation::PWSTR,
                resourcetype: *mut MrmType,
                resourcestring: *mut ::windows::Win32::Foundation::PWSTR,
                data: *mut MrmResourceData,
            ) -> ::windows::core::HRESULT;
        }
        MrmLoadStringOrEmbeddedResource(
            ::core::mem::transmute(resourcemanager),
            ::core::mem::transmute(resourcecontext),
            ::core::mem::transmute(resourcemap),
            resourceid.into_param().abi(),
            ::core::mem::transmute(resourcetype),
            ::core::mem::transmute(resourcestring),
            ::core::mem::transmute(data),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MrmLoadStringOrEmbeddedResourceByIndex(
    resourcemanager: *const MrmManagerHandle__,
    resourcecontext: *const MrmContextHandle__,
    resourcemap: *const MrmMapHandle__,
    index: u32,
    resourcetype: *mut MrmType,
    resourcename: *mut ::windows::Win32::Foundation::PWSTR,
    resourcestring: *mut ::windows::Win32::Foundation::PWSTR,
    data: *mut MrmResourceData,
) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "mrm")]
        extern "system" {
            fn MrmLoadStringOrEmbeddedResourceByIndex(
                resourcemanager: *const MrmManagerHandle__,
                resourcecontext: *const MrmContextHandle__,
                resourcemap: *const MrmMapHandle__,
                index: u32,
                resourcetype: *mut MrmType,
                resourcename: *mut ::windows::Win32::Foundation::PWSTR,
                resourcestring: *mut ::windows::Win32::Foundation::PWSTR,
                data: *mut MrmResourceData,
            ) -> ::windows::core::HRESULT;
        }
        MrmLoadStringOrEmbeddedResourceByIndex(
            ::core::mem::transmute(resourcemanager),
            ::core::mem::transmute(resourcecontext),
            ::core::mem::transmute(resourcemap),
            ::core::mem::transmute(index),
            ::core::mem::transmute(resourcetype),
            ::core::mem::transmute(resourcename),
            ::core::mem::transmute(resourcestring),
            ::core::mem::transmute(data),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MrmLoadStringOrEmbeddedResourceByIndexWithQualifierValues(
    resourcemanager: *const MrmManagerHandle__,
    resourcecontext: *const MrmContextHandle__,
    resourcemap: *const MrmMapHandle__,
    index: u32,
    resourcetype: *mut MrmType,
    resourcename: *mut ::windows::Win32::Foundation::PWSTR,
    resourcestring: *mut ::windows::Win32::Foundation::PWSTR,
    data: *mut MrmResourceData,
    qualifiercount: *mut u32,
    qualifiernames: *mut *mut ::windows::Win32::Foundation::PWSTR,
    qualifiervalues: *mut *mut ::windows::Win32::Foundation::PWSTR,
) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "mrm")]
        extern "system" {
            fn MrmLoadStringOrEmbeddedResourceByIndexWithQualifierValues(
                resourcemanager: *const MrmManagerHandle__,
                resourcecontext: *const MrmContextHandle__,
                resourcemap: *const MrmMapHandle__,
                index: u32,
                resourcetype: *mut MrmType,
                resourcename: *mut ::windows::Win32::Foundation::PWSTR,
                resourcestring: *mut ::windows::Win32::Foundation::PWSTR,
                data: *mut MrmResourceData,
                qualifiercount: *mut u32,
                qualifiernames: *mut *mut ::windows::Win32::Foundation::PWSTR,
                qualifiervalues: *mut *mut ::windows::Win32::Foundation::PWSTR,
            ) -> ::windows::core::HRESULT;
        }
        MrmLoadStringOrEmbeddedResourceByIndexWithQualifierValues(
            ::core::mem::transmute(resourcemanager),
            ::core::mem::transmute(resourcecontext),
            ::core::mem::transmute(resourcemap),
            ::core::mem::transmute(index),
            ::core::mem::transmute(resourcetype),
            ::core::mem::transmute(resourcename),
            ::core::mem::transmute(resourcestring),
            ::core::mem::transmute(data),
            ::core::mem::transmute(qualifiercount),
            ::core::mem::transmute(qualifiernames),
            ::core::mem::transmute(qualifiervalues),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MrmLoadStringOrEmbeddedResourceWithQualifierValues<
    'a,
    Param3: ::windows::core::IntoParam<'a, ::windows::Win32::Foundation::PWSTR>,
>(
    resourcemanager: *const MrmManagerHandle__,
    resourcecontext: *const MrmContextHandle__,
    resourcemap: *const MrmMapHandle__,
    resourceid: Param3,
    resourcetype: *mut MrmType,
    resourcestring: *mut ::windows::Win32::Foundation::PWSTR,
    data: *mut MrmResourceData,
    qualifiercount: *mut u32,
    qualifiernames: *mut *mut ::windows::Win32::Foundation::PWSTR,
    qualifiervalues: *mut *mut ::windows::Win32::Foundation::PWSTR,
) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "mrm")]
        extern "system" {
            fn MrmLoadStringOrEmbeddedResourceWithQualifierValues(
                resourcemanager: *const MrmManagerHandle__,
                resourcecontext: *const MrmContextHandle__,
                resourcemap: *const MrmMapHandle__,
                resourceid: ::windows::Win32::Foundation::PWSTR,
                resourcetype: *mut MrmType,
                resourcestring: *mut ::windows::Win32::Foundation::PWSTR,
                data: *mut MrmResourceData,
                qualifiercount: *mut u32,
                qualifiernames: *mut *mut ::windows::Win32::Foundation::PWSTR,
                qualifiervalues: *mut *mut ::windows::Win32::Foundation::PWSTR,
            ) -> ::windows::core::HRESULT;
        }
        MrmLoadStringOrEmbeddedResourceWithQualifierValues(
            ::core::mem::transmute(resourcemanager),
            ::core::mem::transmute(resourcecontext),
            ::core::mem::transmute(resourcemap),
            resourceid.into_param().abi(),
            ::core::mem::transmute(resourcetype),
            ::core::mem::transmute(resourcestring),
            ::core::mem::transmute(data),
            ::core::mem::transmute(qualifiercount),
            ::core::mem::transmute(qualifiernames),
            ::core::mem::transmute(qualifiervalues),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MrmLoadStringResource<
    'a,
    Param3: ::windows::core::IntoParam<'a, ::windows::Win32::Foundation::PWSTR>,
>(
    resourcemanager: *const MrmManagerHandle__,
    resourcecontext: *const MrmContextHandle__,
    resourcemap: *const MrmMapHandle__,
    resourceid: Param3,
) -> ::windows::core::Result<::windows::Win32::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "mrm")]
        extern "system" {
            fn MrmLoadStringResource(
                resourcemanager: *const MrmManagerHandle__,
                resourcecontext: *const MrmContextHandle__,
                resourcemap: *const MrmMapHandle__,
                resourceid: ::windows::Win32::Foundation::PWSTR,
                resourcestring: *mut ::windows::Win32::Foundation::PWSTR,
            ) -> ::windows::core::HRESULT;
        }
        let mut result__: <::windows::Win32::Foundation::PWSTR as ::windows::core::Abi>::Abi =
            ::core::mem::zeroed();
        MrmLoadStringResource(
            ::core::mem::transmute(resourcemanager),
            ::core::mem::transmute(resourcecontext),
            ::core::mem::transmute(resourcemap),
            resourceid.into_param().abi(),
            &mut result__,
        )
        .from_abi::<::windows::Win32::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MrmLoadStringResourceFromResourceUri<
    'a,
    Param2: ::windows::core::IntoParam<'a, ::windows::Win32::Foundation::PWSTR>,
>(
    resourcemanager: *const MrmManagerHandle__,
    resourcecontext: *const MrmContextHandle__,
    resourceuri: Param2,
) -> ::windows::core::Result<::windows::Win32::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "mrm")]
        extern "system" {
            fn MrmLoadStringResourceFromResourceUri(
                resourcemanager: *const MrmManagerHandle__,
                resourcecontext: *const MrmContextHandle__,
                resourceuri: ::windows::Win32::Foundation::PWSTR,
                resourcestring: *mut ::windows::Win32::Foundation::PWSTR,
            ) -> ::windows::core::HRESULT;
        }
        let mut result__: <::windows::Win32::Foundation::PWSTR as ::windows::core::Abi>::Abi =
            ::core::mem::zeroed();
        MrmLoadStringResourceFromResourceUri(
            ::core::mem::transmute(resourcemanager),
            ::core::mem::transmute(resourcecontext),
            resourceuri.into_param().abi(),
            &mut result__,
        )
        .from_abi::<::windows::Win32::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct MrmManagerHandle__ {
    pub unused: i32,
}
impl MrmManagerHandle__ {}
impl ::core::default::Default for MrmManagerHandle__ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for MrmManagerHandle__ {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("MrmManagerHandle__")
            .field("unused", &self.unused)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MrmManagerHandle__ {
    fn eq(&self, other: &Self) -> bool {
        self.unused == other.unused
    }
}
impl ::core::cmp::Eq for MrmManagerHandle__ {}
unsafe impl ::windows::core::Abi for MrmManagerHandle__ {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct MrmMapHandle__ {
    pub unused: i32,
}
impl MrmMapHandle__ {}
impl ::core::default::Default for MrmMapHandle__ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for MrmMapHandle__ {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("MrmMapHandle__")
            .field("unused", &self.unused)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MrmMapHandle__ {
    fn eq(&self, other: &Self) -> bool {
        self.unused == other.unused
    }
}
impl ::core::cmp::Eq for MrmMapHandle__ {}
unsafe impl ::windows::core::Abi for MrmMapHandle__ {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct MrmResourceData {
    pub size: u32,
    pub data: *mut ::core::ffi::c_void,
}
impl MrmResourceData {}
impl ::core::default::Default for MrmResourceData {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for MrmResourceData {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("MrmResourceData")
            .field("size", &self.size)
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MrmResourceData {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size && self.data == other.data
    }
}
impl ::core::cmp::Eq for MrmResourceData {}
unsafe impl ::windows::core::Abi for MrmResourceData {
    type Abi = Self;
}
#[inline]
pub unsafe fn MrmSetQualifier<
    'a,
    Param1: ::windows::core::IntoParam<'a, ::windows::Win32::Foundation::PWSTR>,
    Param2: ::windows::core::IntoParam<'a, ::windows::Win32::Foundation::PWSTR>,
>(
    resourcecontext: *const MrmContextHandle__,
    qualifiername: Param1,
    qualifiervalue: Param2,
) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "mrm")]
        extern "system" {
            fn MrmSetQualifier(
                resourcecontext: *const MrmContextHandle__,
                qualifiername: ::windows::Win32::Foundation::PWSTR,
                qualifiervalue: ::windows::Win32::Foundation::PWSTR,
            ) -> ::windows::core::HRESULT;
        }
        MrmSetQualifier(
            ::core::mem::transmute(resourcecontext),
            qualifiername.into_param().abi(),
            qualifiervalue.into_param().abi(),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct MrmType(pub i32);
pub const MrmType_Unknown: MrmType = MrmType(0i32);
pub const MrmType_String: MrmType = MrmType(1i32);
pub const MrmType_Path: MrmType = MrmType(2i32);
pub const MrmType_Embedded: MrmType = MrmType(3i32);
impl ::core::convert::From<i32> for MrmType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MrmType {
    type Abi = Self;
}
