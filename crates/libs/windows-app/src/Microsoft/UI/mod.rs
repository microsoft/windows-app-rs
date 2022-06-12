#[cfg(feature = "UI_Composition")]
pub mod Composition;
#[cfg(feature = "UI_Dispatching")]
pub mod Dispatching;
#[cfg(feature = "UI_Input")]
pub mod Input;
#[cfg(feature = "UI_Text")]
pub mod Text;
#[cfg(feature = "UI_Windowing")]
pub mod Windowing;
#[cfg(feature = "UI_Xaml")]
pub mod Xaml;
#[doc = "*Required features: `\"UI\"`*"]
#[repr(transparent)]
pub struct ColorHelper(::windows::core::IUnknown);
impl ColorHelper {
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn FromArgb(a: u8, r: u8, g: u8, b: u8) -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorHelperStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).FromArgb)(
                ::windows::core::Interface::as_raw(this),
                a,
                r,
                g,
                b,
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IColorHelperStatics<R, F: FnOnce(&IColorHelperStatics) -> ::windows::core::Result<R>>(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ColorHelper, IColorHelperStatics> =
            ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ColorHelper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ColorHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ColorHelper {}
impl ::core::fmt::Debug for ColorHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ColorHelper").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ColorHelper {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.ColorHelper;{3adddccd-3949-585b-a566-ccb8350dd221})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ColorHelper {
    type Vtable = IColorHelper_Vtbl;
    const IID: ::windows::core::GUID = <IColorHelper as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ColorHelper {
    const NAME: &'static str = "Microsoft.UI.ColorHelper";
}
impl ::core::convert::From<ColorHelper> for ::windows::core::IUnknown {
    fn from(value: ColorHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ColorHelper> for ::windows::core::IUnknown {
    fn from(value: &ColorHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ColorHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ColorHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ColorHelper> for ::windows::core::IInspectable {
    fn from(value: ColorHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ColorHelper> for ::windows::core::IInspectable {
    fn from(value: &ColorHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ColorHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ColorHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ColorHelper {}
unsafe impl ::core::marker::Sync for ColorHelper {}
#[doc = "*Required features: `\"UI\"`*"]
#[repr(transparent)]
pub struct Colors(::windows::core::IUnknown);
impl Colors {
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn AliceBlue() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).AliceBlue)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn AntiqueWhite() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).AntiqueWhite)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Aqua() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).Aqua)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Aquamarine() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).Aquamarine)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Azure() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).Azure)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Beige() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).Beige)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Bisque() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).Bisque)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Black() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).Black)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn BlanchedAlmond() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).BlanchedAlmond)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Blue() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).Blue)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn BlueViolet() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).BlueViolet)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Brown() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).Brown)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn BurlyWood() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).BurlyWood)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn CadetBlue() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).CadetBlue)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Chartreuse() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).Chartreuse)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Chocolate() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).Chocolate)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Coral() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).Coral)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn CornflowerBlue() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).CornflowerBlue)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Cornsilk() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).Cornsilk)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Crimson() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).Crimson)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Cyan() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).Cyan)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DarkBlue() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).DarkBlue)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DarkCyan() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).DarkCyan)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DarkGoldenrod() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).DarkGoldenrod)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DarkGray() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).DarkGray)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DarkGreen() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).DarkGreen)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DarkKhaki() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).DarkKhaki)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DarkMagenta() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).DarkMagenta)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DarkOliveGreen() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).DarkOliveGreen)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DarkOrange() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).DarkOrange)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DarkOrchid() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).DarkOrchid)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DarkRed() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).DarkRed)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DarkSalmon() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).DarkSalmon)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DarkSeaGreen() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).DarkSeaGreen)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DarkSlateBlue() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).DarkSlateBlue)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DarkSlateGray() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).DarkSlateGray)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DarkTurquoise() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).DarkTurquoise)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DarkViolet() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).DarkViolet)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DeepPink() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).DeepPink)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DeepSkyBlue() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).DeepSkyBlue)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DimGray() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).DimGray)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DodgerBlue() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).DodgerBlue)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Firebrick() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).Firebrick)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn FloralWhite() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).FloralWhite)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn ForestGreen() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).ForestGreen)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Fuchsia() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).Fuchsia)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Gainsboro() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).Gainsboro)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn GhostWhite() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).GhostWhite)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Gold() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).Gold)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Goldenrod() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).Goldenrod)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Gray() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).Gray)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Green() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).Green)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn GreenYellow() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).GreenYellow)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Honeydew() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).Honeydew)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn HotPink() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).HotPink)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn IndianRed() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).IndianRed)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Indigo() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).Indigo)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Ivory() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).Ivory)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Khaki() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).Khaki)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Lavender() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).Lavender)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn LavenderBlush() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).LavenderBlush)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn LawnGreen() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).LawnGreen)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn LemonChiffon() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).LemonChiffon)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn LightBlue() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).LightBlue)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn LightCoral() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).LightCoral)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn LightCyan() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).LightCyan)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn LightGoldenrodYellow() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).LightGoldenrodYellow)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn LightGreen() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).LightGreen)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn LightGray() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).LightGray)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn LightPink() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).LightPink)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn LightSalmon() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).LightSalmon)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn LightSeaGreen() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).LightSeaGreen)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn LightSkyBlue() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).LightSkyBlue)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn LightSlateGray() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).LightSlateGray)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn LightSteelBlue() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).LightSteelBlue)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn LightYellow() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).LightYellow)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Lime() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).Lime)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn LimeGreen() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).LimeGreen)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Linen() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).Linen)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Magenta() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).Magenta)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Maroon() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).Maroon)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn MediumAquamarine() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).MediumAquamarine)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn MediumBlue() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).MediumBlue)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn MediumOrchid() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).MediumOrchid)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn MediumPurple() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).MediumPurple)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn MediumSeaGreen() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).MediumSeaGreen)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn MediumSlateBlue() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).MediumSlateBlue)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn MediumSpringGreen() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).MediumSpringGreen)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn MediumTurquoise() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).MediumTurquoise)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn MediumVioletRed() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).MediumVioletRed)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn MidnightBlue() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).MidnightBlue)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn MintCream() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).MintCream)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn MistyRose() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).MistyRose)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Moccasin() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).Moccasin)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn NavajoWhite() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).NavajoWhite)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Navy() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).Navy)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn OldLace() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).OldLace)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Olive() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).Olive)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn OliveDrab() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).OliveDrab)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Orange() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).Orange)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn OrangeRed() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).OrangeRed)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Orchid() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).Orchid)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn PaleGoldenrod() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).PaleGoldenrod)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn PaleGreen() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).PaleGreen)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn PaleTurquoise() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).PaleTurquoise)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn PaleVioletRed() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).PaleVioletRed)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn PapayaWhip() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).PapayaWhip)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn PeachPuff() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).PeachPuff)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Peru() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).Peru)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Pink() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).Pink)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Plum() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).Plum)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn PowderBlue() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).PowderBlue)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Purple() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).Purple)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Red() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).Red)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn RosyBrown() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).RosyBrown)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn RoyalBlue() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).RoyalBlue)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn SaddleBrown() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).SaddleBrown)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Salmon() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).Salmon)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn SandyBrown() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).SandyBrown)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn SeaGreen() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).SeaGreen)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn SeaShell() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).SeaShell)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Sienna() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).Sienna)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Silver() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).Silver)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn SkyBlue() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).SkyBlue)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn SlateBlue() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).SlateBlue)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn SlateGray() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).SlateGray)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Snow() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).Snow)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn SpringGreen() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).SpringGreen)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn SteelBlue() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).SteelBlue)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Tan() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).Tan)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Teal() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).Teal)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Thistle() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).Thistle)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Tomato() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).Tomato)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Transparent() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).Transparent)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Turquoise() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).Turquoise)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Violet() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).Violet)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Wheat() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).Wheat)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn White() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).White)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn WhiteSmoke() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).WhiteSmoke)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Yellow() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).Yellow)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn YellowGreen() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).YellowGreen)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IColorsStatics<R, F: FnOnce(&IColorsStatics) -> ::windows::core::Result<R>>(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Colors, IColorsStatics> =
            ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Colors {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Colors {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Colors {}
impl ::core::fmt::Debug for Colors {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Colors").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Colors {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Colors;{8cf15863-8411-5afd-946c-328e04da2f2f})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for Colors {
    type Vtable = IColors_Vtbl;
    const IID: ::windows::core::GUID = <IColors as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Colors {
    const NAME: &'static str = "Microsoft.UI.Colors";
}
impl ::core::convert::From<Colors> for ::windows::core::IUnknown {
    fn from(value: Colors) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Colors> for ::windows::core::IUnknown {
    fn from(value: &Colors) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Colors {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Colors {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Colors> for ::windows::core::IInspectable {
    fn from(value: Colors) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Colors> for ::windows::core::IInspectable {
    fn from(value: &Colors) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Colors {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Colors {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for Colors {}
unsafe impl ::core::marker::Sync for Colors {}
#[repr(C)]
#[doc = "*Required features: `\"UI\"`*"]
pub struct DisplayId {
    pub Value: u64,
}
impl ::core::marker::Copy for DisplayId {}
impl ::core::clone::Clone for DisplayId {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DisplayId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DisplayId")
            .field("Value", &self.Value)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for DisplayId {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DisplayId {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"struct(Microsoft.UI.DisplayId;u8)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for DisplayId {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            ::windows::core::memcmp(
                self as *const _ as _,
                other as *const _ as _,
                core::mem::size_of::<DisplayId>(),
            ) == 0
        }
    }
}
impl ::core::cmp::Eq for DisplayId {}
impl ::core::default::Default for DisplayId {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IColorHelper(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IColorHelper {
    type Vtable = IColorHelper_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x3adddccd_3949_585b_a566_ccb8350dd221);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorHelper_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IColorHelperStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IColorHelperStatics {
    type Vtable = IColorHelperStatics_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x1d1d85a1_eb63_538a_84f0_019210bc406b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorHelperStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub FromArgb: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        a: u8,
        r: u8,
        g: u8,
        b: u8,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IColors(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IColors {
    type Vtable = IColors_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x8cf15863_8411_5afd_946c_328e04da2f2f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColors_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IColorsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IColorsStatics {
    type Vtable = IColorsStatics_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x8620a5b0_015a_57ac_a3f3_895d0b1269ae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorsStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub AliceBlue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub AntiqueWhite: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Aqua: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Aquamarine: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Azure: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Beige: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Bisque: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Black: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub BlanchedAlmond: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Blue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub BlueViolet: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Brown: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub BurlyWood: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub CadetBlue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Chartreuse: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Chocolate: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Coral: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub CornflowerBlue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Cornsilk: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Crimson: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Cyan: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub DarkBlue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub DarkCyan: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub DarkGoldenrod: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub DarkGray: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub DarkGreen: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub DarkKhaki: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub DarkMagenta: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub DarkOliveGreen: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub DarkOrange: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub DarkOrchid: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub DarkRed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub DarkSalmon: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub DarkSeaGreen: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub DarkSlateBlue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub DarkSlateGray: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub DarkTurquoise: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub DarkViolet: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub DeepPink: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub DeepSkyBlue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub DimGray: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub DodgerBlue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Firebrick: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub FloralWhite: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub ForestGreen: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Fuchsia: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Gainsboro: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub GhostWhite: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Gold: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Goldenrod: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Gray: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Green: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub GreenYellow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Honeydew: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub HotPink: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub IndianRed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Indigo: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Ivory: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Khaki: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Lavender: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub LavenderBlush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub LawnGreen: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub LemonChiffon: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub LightBlue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub LightCoral: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub LightCyan: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub LightGoldenrodYellow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub LightGreen: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub LightGray: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub LightPink: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub LightSalmon: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub LightSeaGreen: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub LightSkyBlue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub LightSlateGray: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub LightSteelBlue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub LightYellow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Lime: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub LimeGreen: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Linen: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Magenta: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Maroon: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub MediumAquamarine: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub MediumBlue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub MediumOrchid: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub MediumPurple: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub MediumSeaGreen: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub MediumSlateBlue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub MediumSpringGreen: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub MediumTurquoise: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub MediumVioletRed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub MidnightBlue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub MintCream: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub MistyRose: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Moccasin: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub NavajoWhite: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Navy: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub OldLace: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Olive: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub OliveDrab: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Orange: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub OrangeRed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Orchid: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub PaleGoldenrod: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub PaleGreen: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub PaleTurquoise: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub PaleVioletRed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub PapayaWhip: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub PeachPuff: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Peru: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Pink: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Plum: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub PowderBlue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Purple: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Red: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub RosyBrown: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub RoyalBlue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub SaddleBrown: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Salmon: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub SandyBrown: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub SeaGreen: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub SeaShell: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Sienna: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Silver: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub SkyBlue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub SlateBlue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub SlateGray: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Snow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub SpringGreen: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub SteelBlue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Tan: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Teal: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Thistle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Tomato: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Transparent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Turquoise: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Violet: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Wheat: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub White: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub WhiteSmoke: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Yellow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub YellowGreen: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
}
#[repr(C)]
#[doc = "*Required features: `\"UI\"`*"]
pub struct IconId {
    pub Value: u64,
}
impl ::core::marker::Copy for IconId {}
impl ::core::clone::Clone for IconId {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IconId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IconId")
            .field("Value", &self.Value)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for IconId {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for IconId {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"struct(Microsoft.UI.IconId;u8)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for IconId {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            ::windows::core::memcmp(
                self as *const _ as _,
                other as *const _ as _,
                core::mem::size_of::<IconId>(),
            ) == 0
        }
    }
}
impl ::core::cmp::Eq for IconId {}
impl ::core::default::Default for IconId {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"UI\"`*"]
pub struct WindowId {
    pub Value: u64,
}
impl ::core::marker::Copy for WindowId {}
impl ::core::clone::Clone for WindowId {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WindowId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WindowId")
            .field("Value", &self.Value)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for WindowId {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for WindowId {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"struct(Microsoft.UI.WindowId;u8)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for WindowId {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            ::windows::core::memcmp(
                self as *const _ as _,
                other as *const _ as _,
                core::mem::size_of::<WindowId>(),
            ) == 0
        }
    }
}
impl ::core::cmp::Eq for WindowId {}
impl ::core::default::Default for WindowId {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
