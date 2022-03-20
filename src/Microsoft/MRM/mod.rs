#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clashing_extern_declarations,
    clippy::all
)]
#[doc = "*Required features: `\"MRM\"`*"]
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
#[repr(C)]
#[doc = "*Required features: `\"MRM\"`*"]
pub struct MrmContextHandle__ {
    pub unused: i32,
}
impl ::core::marker::Copy for MrmContextHandle__ {}
impl ::core::clone::Clone for MrmContextHandle__ {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MrmContextHandle__ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MrmContextHandle__")
            .field("unused", &self.unused)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for MrmContextHandle__ {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MrmContextHandle__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            ::windows::core::memcmp(
                self as *const _ as _,
                other as *const _ as _,
                core::mem::size_of::<MrmContextHandle__>(),
            ) == 0
        }
    }
}
impl ::core::cmp::Eq for MrmContextHandle__ {}
impl ::core::default::Default for MrmContextHandle__ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"MRM\"`*"]
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
        let mut result__: *mut MrmContextHandle__ = ::core::mem::zeroed();
        MrmCreateResourceContext(
            ::core::mem::transmute(resourcemanager),
            ::core::mem::transmute(&mut result__),
        )
        .from_abi::<*mut MrmContextHandle__>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"MRM\"`*"]
#[inline]
pub unsafe fn MrmCreateResourceManager<
    'a,
    Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>,
>(
    prifilename: Param0,
) -> ::windows::core::Result<*mut MrmManagerHandle__> {
    #[cfg(windows)]
    {
        #[link(name = "mrm")]
        extern "system" {
            fn MrmCreateResourceManager(
                prifilename: ::windows::core::PCWSTR,
                resourcemanager: *mut *mut MrmManagerHandle__,
            ) -> ::windows::core::HRESULT;
        }
        let mut result__: *mut MrmManagerHandle__ = ::core::mem::zeroed();
        MrmCreateResourceManager(
            prifilename.into_param().abi(),
            ::core::mem::transmute(&mut result__),
        )
        .from_abi::<*mut MrmManagerHandle__>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"MRM\"`*"]
#[inline]
pub unsafe fn MrmDestroyResourceContext(resourcecontext: *const MrmContextHandle__) {
    #[cfg(windows)]
    {
        #[link(name = "mrm")]
        extern "system" {
            fn MrmDestroyResourceContext(resourcecontext: *const MrmContextHandle__);
        }
        MrmDestroyResourceContext(::core::mem::transmute(resourcecontext))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"MRM\"`*"]
#[inline]
pub unsafe fn MrmDestroyResourceManager(resourcemanager: *const MrmManagerHandle__) {
    #[cfg(windows)]
    {
        #[link(name = "mrm")]
        extern "system" {
            fn MrmDestroyResourceManager(resourcemanager: *const MrmManagerHandle__);
        }
        MrmDestroyResourceManager(::core::mem::transmute(resourcemanager))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"MRM\"`*"]
#[inline]
pub unsafe fn MrmFreeQualifierNamesOrValues(names: &[::windows::core::PWSTR]) {
    #[cfg(windows)]
    {
        #[link(name = "mrm")]
        extern "system" {
            fn MrmFreeQualifierNamesOrValues(size: u32, names: *const ::windows::core::PWSTR);
        }
        MrmFreeQualifierNamesOrValues(
            names.len() as _,
            ::core::mem::transmute(::windows::core::as_ptr_or_null(names)),
        )
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"MRM\"`*"]
#[inline]
pub unsafe fn MrmFreeResource(resource: *const ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "mrm")]
        extern "system" {
            fn MrmFreeResource(resource: *const ::core::ffi::c_void);
        }
        MrmFreeResource(::core::mem::transmute(resource))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"MRM\"`*"]
#[inline]
pub unsafe fn MrmGetAllQualifierNames(
    resourcecontext: *const MrmContextHandle__,
    size: *mut u32,
    names: *mut *mut ::windows::core::PWSTR,
) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "mrm")]
        extern "system" {
            fn MrmGetAllQualifierNames(
                resourcecontext: *const MrmContextHandle__,
                size: *mut u32,
                names: *mut *mut ::windows::core::PWSTR,
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
#[doc = "*Required features: `\"MRM\"`*"]
#[inline]
pub unsafe fn MrmGetChildResourceMap<
    'a,
    Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>,
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
                childresourcemapname: ::windows::core::PCWSTR,
                childresourcemap: *mut *mut MrmMapHandle__,
            ) -> ::windows::core::HRESULT;
        }
        let mut result__: *mut MrmMapHandle__ = ::core::mem::zeroed();
        MrmGetChildResourceMap(
            ::core::mem::transmute(resourcemanager),
            ::core::mem::transmute(resourcemap),
            childresourcemapname.into_param().abi(),
            ::core::mem::transmute(&mut result__),
        )
        .from_abi::<*mut MrmMapHandle__>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"MRM\"`*"]
#[inline]
pub unsafe fn MrmGetFilePathFromName<
    'a,
    Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>,
>(
    filename: Param0,
) -> ::windows::core::Result<::windows::core::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "mrm")]
        extern "system" {
            fn MrmGetFilePathFromName(
                filename: ::windows::core::PCWSTR,
                filepath: *mut ::windows::core::PWSTR,
            ) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        MrmGetFilePathFromName(
            filename.into_param().abi(),
            ::core::mem::transmute(&mut result__),
        )
        .from_abi::<::windows::core::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"MRM\"`*"]
#[inline]
pub unsafe fn MrmGetQualifier<
    'a,
    Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>,
>(
    resourcecontext: *const MrmContextHandle__,
    qualifiername: Param1,
) -> ::windows::core::Result<::windows::core::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "mrm")]
        extern "system" {
            fn MrmGetQualifier(
                resourcecontext: *const MrmContextHandle__,
                qualifiername: ::windows::core::PCWSTR,
                qualifiervalue: *mut ::windows::core::PWSTR,
            ) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        MrmGetQualifier(
            ::core::mem::transmute(resourcecontext),
            qualifiername.into_param().abi(),
            ::core::mem::transmute(&mut result__),
        )
        .from_abi::<::windows::core::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"MRM\"`*"]
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
        let mut result__: u32 = ::core::mem::zeroed();
        MrmGetResourceCount(
            ::core::mem::transmute(resourcemanager),
            ::core::mem::transmute(resourcemap),
            ::core::mem::transmute(&mut result__),
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"MRM\"`*"]
#[inline]
pub unsafe fn MrmLoadEmbeddedResource<
    'a,
    Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>,
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
                resourceid: ::windows::core::PCWSTR,
                data: *mut MrmResourceData,
            ) -> ::windows::core::HRESULT;
        }
        let mut result__: MrmResourceData = ::core::mem::zeroed();
        MrmLoadEmbeddedResource(
            ::core::mem::transmute(resourcemanager),
            ::core::mem::transmute(resourcecontext),
            ::core::mem::transmute(resourcemap),
            resourceid.into_param().abi(),
            ::core::mem::transmute(&mut result__),
        )
        .from_abi::<MrmResourceData>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"MRM\"`*"]
#[inline]
pub unsafe fn MrmLoadEmbeddedResourceFromResourceUri<
    'a,
    Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>,
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
                resourceuri: ::windows::core::PCWSTR,
                data: *mut MrmResourceData,
            ) -> ::windows::core::HRESULT;
        }
        let mut result__: MrmResourceData = ::core::mem::zeroed();
        MrmLoadEmbeddedResourceFromResourceUri(
            ::core::mem::transmute(resourcemanager),
            ::core::mem::transmute(resourcecontext),
            resourceuri.into_param().abi(),
            ::core::mem::transmute(&mut result__),
        )
        .from_abi::<MrmResourceData>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"MRM\"`*"]
#[inline]
pub unsafe fn MrmLoadStringOrEmbeddedFromResourceUri<
    'a,
    Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>,
>(
    resourcemanager: *const MrmManagerHandle__,
    resourcecontext: *const MrmContextHandle__,
    resourceuri: Param2,
    resourcetype: *mut MrmType,
    resourcestring: *mut ::windows::core::PWSTR,
    data: *mut MrmResourceData,
) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "mrm")]
        extern "system" {
            fn MrmLoadStringOrEmbeddedFromResourceUri(
                resourcemanager: *const MrmManagerHandle__,
                resourcecontext: *const MrmContextHandle__,
                resourceuri: ::windows::core::PCWSTR,
                resourcetype: *mut MrmType,
                resourcestring: *mut ::windows::core::PWSTR,
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
#[doc = "*Required features: `\"MRM\"`*"]
#[inline]
pub unsafe fn MrmLoadStringOrEmbeddedResource<
    'a,
    Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>,
>(
    resourcemanager: *const MrmManagerHandle__,
    resourcecontext: *const MrmContextHandle__,
    resourcemap: *const MrmMapHandle__,
    resourceid: Param3,
    resourcetype: *mut MrmType,
    resourcestring: *mut ::windows::core::PWSTR,
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
                resourceid: ::windows::core::PCWSTR,
                resourcetype: *mut MrmType,
                resourcestring: *mut ::windows::core::PWSTR,
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
#[doc = "*Required features: `\"MRM\"`*"]
#[inline]
pub unsafe fn MrmLoadStringOrEmbeddedResourceByIndex(
    resourcemanager: *const MrmManagerHandle__,
    resourcecontext: *const MrmContextHandle__,
    resourcemap: *const MrmMapHandle__,
    index: u32,
    resourcetype: *mut MrmType,
    resourcename: *mut ::windows::core::PWSTR,
    resourcestring: *mut ::windows::core::PWSTR,
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
                resourcename: *mut ::windows::core::PWSTR,
                resourcestring: *mut ::windows::core::PWSTR,
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
#[doc = "*Required features: `\"MRM\"`*"]
#[inline]
pub unsafe fn MrmLoadStringOrEmbeddedResourceByIndexWithQualifierValues(
    resourcemanager: *const MrmManagerHandle__,
    resourcecontext: *const MrmContextHandle__,
    resourcemap: *const MrmMapHandle__,
    index: u32,
    resourcetype: *mut MrmType,
    resourcename: *mut ::windows::core::PWSTR,
    resourcestring: *mut ::windows::core::PWSTR,
    data: *mut MrmResourceData,
    qualifiercount: *mut u32,
    qualifiernames: *mut *mut ::windows::core::PWSTR,
    qualifiervalues: *mut *mut ::windows::core::PWSTR,
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
                resourcename: *mut ::windows::core::PWSTR,
                resourcestring: *mut ::windows::core::PWSTR,
                data: *mut MrmResourceData,
                qualifiercount: *mut u32,
                qualifiernames: *mut *mut ::windows::core::PWSTR,
                qualifiervalues: *mut *mut ::windows::core::PWSTR,
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
#[doc = "*Required features: `\"MRM\"`*"]
#[inline]
pub unsafe fn MrmLoadStringOrEmbeddedResourceWithQualifierValues<
    'a,
    Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>,
>(
    resourcemanager: *const MrmManagerHandle__,
    resourcecontext: *const MrmContextHandle__,
    resourcemap: *const MrmMapHandle__,
    resourceid: Param3,
    resourcetype: *mut MrmType,
    resourcestring: *mut ::windows::core::PWSTR,
    data: *mut MrmResourceData,
    qualifiercount: *mut u32,
    qualifiernames: *mut *mut ::windows::core::PWSTR,
    qualifiervalues: *mut *mut ::windows::core::PWSTR,
) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "mrm")]
        extern "system" {
            fn MrmLoadStringOrEmbeddedResourceWithQualifierValues(
                resourcemanager: *const MrmManagerHandle__,
                resourcecontext: *const MrmContextHandle__,
                resourcemap: *const MrmMapHandle__,
                resourceid: ::windows::core::PCWSTR,
                resourcetype: *mut MrmType,
                resourcestring: *mut ::windows::core::PWSTR,
                data: *mut MrmResourceData,
                qualifiercount: *mut u32,
                qualifiernames: *mut *mut ::windows::core::PWSTR,
                qualifiervalues: *mut *mut ::windows::core::PWSTR,
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
#[doc = "*Required features: `\"MRM\"`*"]
#[inline]
pub unsafe fn MrmLoadStringResource<
    'a,
    Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>,
>(
    resourcemanager: *const MrmManagerHandle__,
    resourcecontext: *const MrmContextHandle__,
    resourcemap: *const MrmMapHandle__,
    resourceid: Param3,
) -> ::windows::core::Result<::windows::core::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "mrm")]
        extern "system" {
            fn MrmLoadStringResource(
                resourcemanager: *const MrmManagerHandle__,
                resourcecontext: *const MrmContextHandle__,
                resourcemap: *const MrmMapHandle__,
                resourceid: ::windows::core::PCWSTR,
                resourcestring: *mut ::windows::core::PWSTR,
            ) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        MrmLoadStringResource(
            ::core::mem::transmute(resourcemanager),
            ::core::mem::transmute(resourcecontext),
            ::core::mem::transmute(resourcemap),
            resourceid.into_param().abi(),
            ::core::mem::transmute(&mut result__),
        )
        .from_abi::<::windows::core::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"MRM\"`*"]
#[inline]
pub unsafe fn MrmLoadStringResourceFromResourceUri<
    'a,
    Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>,
>(
    resourcemanager: *const MrmManagerHandle__,
    resourcecontext: *const MrmContextHandle__,
    resourceuri: Param2,
) -> ::windows::core::Result<::windows::core::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "mrm")]
        extern "system" {
            fn MrmLoadStringResourceFromResourceUri(
                resourcemanager: *const MrmManagerHandle__,
                resourcecontext: *const MrmContextHandle__,
                resourceuri: ::windows::core::PCWSTR,
                resourcestring: *mut ::windows::core::PWSTR,
            ) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        MrmLoadStringResourceFromResourceUri(
            ::core::mem::transmute(resourcemanager),
            ::core::mem::transmute(resourcecontext),
            resourceuri.into_param().abi(),
            ::core::mem::transmute(&mut result__),
        )
        .from_abi::<::windows::core::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: `\"MRM\"`*"]
pub struct MrmManagerHandle__ {
    pub unused: i32,
}
impl ::core::marker::Copy for MrmManagerHandle__ {}
impl ::core::clone::Clone for MrmManagerHandle__ {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MrmManagerHandle__ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MrmManagerHandle__")
            .field("unused", &self.unused)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for MrmManagerHandle__ {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MrmManagerHandle__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            ::windows::core::memcmp(
                self as *const _ as _,
                other as *const _ as _,
                core::mem::size_of::<MrmManagerHandle__>(),
            ) == 0
        }
    }
}
impl ::core::cmp::Eq for MrmManagerHandle__ {}
impl ::core::default::Default for MrmManagerHandle__ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"MRM\"`*"]
pub struct MrmMapHandle__ {
    pub unused: i32,
}
impl ::core::marker::Copy for MrmMapHandle__ {}
impl ::core::clone::Clone for MrmMapHandle__ {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MrmMapHandle__ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MrmMapHandle__")
            .field("unused", &self.unused)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for MrmMapHandle__ {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MrmMapHandle__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            ::windows::core::memcmp(
                self as *const _ as _,
                other as *const _ as _,
                core::mem::size_of::<MrmMapHandle__>(),
            ) == 0
        }
    }
}
impl ::core::cmp::Eq for MrmMapHandle__ {}
impl ::core::default::Default for MrmMapHandle__ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"MRM\"`*"]
pub struct MrmResourceData {
    pub size: u32,
    pub data: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for MrmResourceData {}
impl ::core::clone::Clone for MrmResourceData {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MrmResourceData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MrmResourceData")
            .field("size", &self.size)
            .field("data", &self.data)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for MrmResourceData {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MrmResourceData {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            ::windows::core::memcmp(
                self as *const _ as _,
                other as *const _ as _,
                core::mem::size_of::<MrmResourceData>(),
            ) == 0
        }
    }
}
impl ::core::cmp::Eq for MrmResourceData {}
impl ::core::default::Default for MrmResourceData {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"MRM\"`*"]
#[inline]
pub unsafe fn MrmSetQualifier<
    'a,
    Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>,
    Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>,
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
                qualifiername: ::windows::core::PCWSTR,
                qualifiervalue: ::windows::core::PCWSTR,
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
#[doc = "*Required features: `\"MRM\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MrmType(pub i32);
#[doc = "*Required features: `\"MRM\"`*"]
pub const MrmType_Unknown: MrmType = MrmType(0i32);
#[doc = "*Required features: `\"MRM\"`*"]
pub const MrmType_String: MrmType = MrmType(1i32);
#[doc = "*Required features: `\"MRM\"`*"]
pub const MrmType_Path: MrmType = MrmType(2i32);
#[doc = "*Required features: `\"MRM\"`*"]
pub const MrmType_Embedded: MrmType = MrmType(3i32);
impl ::core::marker::Copy for MrmType {}
impl ::core::clone::Clone for MrmType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MrmType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MrmType {
    type Abi = Self;
}
impl ::core::fmt::Debug for MrmType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MrmType").field(&self.0).finish()
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
