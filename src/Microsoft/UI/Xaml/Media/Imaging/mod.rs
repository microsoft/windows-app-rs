#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct BitmapCreateOptions(pub u32);
impl BitmapCreateOptions {
    pub const None: BitmapCreateOptions = BitmapCreateOptions(0u32);
    pub const IgnoreImageCache: BitmapCreateOptions = BitmapCreateOptions(8u32);
}
impl ::core::convert::From<u32> for BitmapCreateOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for BitmapCreateOptions {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for BitmapCreateOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Media.Imaging.BitmapCreateOptions;u4)",
    );
}
impl ::windows::core::DefaultType for BitmapCreateOptions {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for BitmapCreateOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for BitmapCreateOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for BitmapCreateOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for BitmapCreateOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for BitmapCreateOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct BitmapImage(pub ::windows::core::IInspectable);
impl BitmapImage {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            BitmapImage,
            ::windows::core::IActivationFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn CreateOptions(&self) -> ::windows::core::Result<BitmapCreateOptions> {
        let this = self;
        unsafe {
            let mut result__: BitmapCreateOptions = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<BitmapCreateOptions>(result__)
        }
    }
    pub fn SetCreateOptions(&self, value: BitmapCreateOptions) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn UriSource(&self) -> ::windows::core::Result<::windows::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Uri>(result__)
        }
    }
    pub fn SetUriSource<'a, Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Uri>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn DecodePixelWidth(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetDecodePixelWidth(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn DecodePixelHeight(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetDecodePixelHeight(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn DecodePixelType(&self) -> ::windows::core::Result<DecodePixelType> {
        let this = self;
        unsafe {
            let mut result__: DecodePixelType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<DecodePixelType>(result__)
        }
    }
    pub fn SetDecodePixelType(&self, value: DecodePixelType) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn IsAnimatedBitmap(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn IsPlaying(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn AutoPlay(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAutoPlay(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn DownloadProgress<
        'a,
        Param0: ::windows::core::IntoParam<'a, DownloadProgressEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveDownloadProgress<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).21)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ImageOpened<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::RoutedEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveImageOpened<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).23)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ImageFailed<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::ExceptionRoutedEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveImageFailed<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).25)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Play(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this)).ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn CreateInstanceWithUriSource<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Uri>,
    >(
        urisource: Param0,
    ) -> ::windows::core::Result<BitmapImage> {
        Self::IBitmapImageFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                urisource.into_param().abi(),
                &mut result__,
            )
            .from_abi::<BitmapImage>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn CreateOptionsProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IBitmapImageStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn UriSourceProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IBitmapImageStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DecodePixelWidthProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IBitmapImageStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DecodePixelHeightProperty() -> ::windows::core::Result<super::super::DependencyProperty>
    {
        Self::IBitmapImageStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DecodePixelTypeProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IBitmapImageStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn IsAnimatedBitmapProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IBitmapImageStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn IsPlayingProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IBitmapImageStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn AutoPlayProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IBitmapImageStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ClearValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ReadLocalValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetAnimationBaseValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn RegisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, super::super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::core::Result<i64> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                callback.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i64>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn UnregisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                token,
            )
            .ok()
        }
    }
    pub fn Dispatcher(&self) -> ::windows::core::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn PixelWidth(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<IBitmapSource>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn PixelHeight(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<IBitmapSource>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetSource<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Storage::Streams::IRandomAccessStream>,
    >(
        &self,
        streamsource: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBitmapSource>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                streamsource.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn SetSourceAsync<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Storage::Streams::IRandomAccessStream>,
    >(
        &self,
        streamsource: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IBitmapSource>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                streamsource.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn IBitmapImageFactory<R, F: FnOnce(&IBitmapImageFactory) -> ::windows::core::Result<R>>(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BitmapImage, IBitmapImageFactory> =
            ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IBitmapImageStatics<R, F: FnOnce(&IBitmapImageStatics) -> ::windows::core::Result<R>>(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BitmapImage, IBitmapImageStatics> =
            ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for BitmapImage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Media.Imaging.BitmapImage;{5cc29916-a411-5bc2-a3c5-a00d99a59da8})",
    );
}
unsafe impl ::windows::core::Interface for BitmapImage {
    type Vtable = IBitmapImage_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x5cc29916_a411_5bc2_a3c5_a00d99a59da8);
}
impl ::windows::core::RuntimeName for BitmapImage {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Imaging.BitmapImage";
}
impl ::core::convert::From<BitmapImage> for ::windows::core::IUnknown {
    fn from(value: BitmapImage) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BitmapImage> for ::windows::core::IUnknown {
    fn from(value: &BitmapImage) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BitmapImage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BitmapImage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BitmapImage> for ::windows::core::IInspectable {
    fn from(value: BitmapImage) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BitmapImage> for ::windows::core::IInspectable {
    fn from(value: &BitmapImage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BitmapImage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BitmapImage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<BitmapImage> for BitmapSource {
    fn from(value: BitmapImage) -> Self {
        ::core::convert::Into::<BitmapSource>::into(&value)
    }
}
impl ::core::convert::From<&BitmapImage> for BitmapSource {
    fn from(value: &BitmapImage) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, BitmapSource> for BitmapImage {
    fn into_param(self) -> ::windows::core::Param<'a, BitmapSource> {
        ::windows::core::Param::Owned(::core::convert::Into::<BitmapSource>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, BitmapSource> for &BitmapImage {
    fn into_param(self) -> ::windows::core::Param<'a, BitmapSource> {
        ::windows::core::Param::Owned(::core::convert::Into::<BitmapSource>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<BitmapImage> for super::ImageSource {
    fn from(value: BitmapImage) -> Self {
        ::core::convert::Into::<super::ImageSource>::into(&value)
    }
}
impl ::core::convert::From<&BitmapImage> for super::ImageSource {
    fn from(value: &BitmapImage) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::ImageSource> for BitmapImage {
    fn into_param(self) -> ::windows::core::Param<'a, super::ImageSource> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::ImageSource>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::ImageSource> for &BitmapImage {
    fn into_param(self) -> ::windows::core::Param<'a, super::ImageSource> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::ImageSource>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<BitmapImage> for super::super::DependencyObject {
    fn from(value: BitmapImage) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&BitmapImage> for super::super::DependencyObject {
    fn from(value: &BitmapImage) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for BitmapImage {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(
            ::core::convert::Into::<super::super::DependencyObject>::into(self),
        )
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &BitmapImage {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(
            ::core::convert::Into::<super::super::DependencyObject>::into(
                ::core::clone::Clone::clone(self),
            ),
        )
    }
}
unsafe impl ::core::marker::Send for BitmapImage {}
unsafe impl ::core::marker::Sync for BitmapImage {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct BitmapSource(pub ::windows::core::IInspectable);
impl BitmapSource {
    pub fn PixelWidth(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn PixelHeight(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetSource<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Storage::Streams::IRandomAccessStream>,
    >(
        &self,
        streamsource: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                streamsource.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn SetSourceAsync<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Storage::Streams::IRandomAccessStream>,
    >(
        &self,
        streamsource: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                streamsource.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn PixelWidthProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IBitmapSourceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn PixelHeightProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IBitmapSourceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ClearValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ReadLocalValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetAnimationBaseValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn RegisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, super::super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::core::Result<i64> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                callback.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i64>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn UnregisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                token,
            )
            .ok()
        }
    }
    pub fn Dispatcher(&self) -> ::windows::core::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn IBitmapSourceStatics<
        R,
        F: FnOnce(&IBitmapSourceStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BitmapSource, IBitmapSourceStatics> =
            ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for BitmapSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Media.Imaging.BitmapSource;{8424269d-9b82-534f-8fea-af5b5ef96bf2})",
    );
}
unsafe impl ::windows::core::Interface for BitmapSource {
    type Vtable = IBitmapSource_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x8424269d_9b82_534f_8fea_af5b5ef96bf2);
}
impl ::windows::core::RuntimeName for BitmapSource {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Imaging.BitmapSource";
}
impl ::core::convert::From<BitmapSource> for ::windows::core::IUnknown {
    fn from(value: BitmapSource) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BitmapSource> for ::windows::core::IUnknown {
    fn from(value: &BitmapSource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BitmapSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BitmapSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BitmapSource> for ::windows::core::IInspectable {
    fn from(value: BitmapSource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BitmapSource> for ::windows::core::IInspectable {
    fn from(value: &BitmapSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BitmapSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BitmapSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<BitmapSource> for super::ImageSource {
    fn from(value: BitmapSource) -> Self {
        ::core::convert::Into::<super::ImageSource>::into(&value)
    }
}
impl ::core::convert::From<&BitmapSource> for super::ImageSource {
    fn from(value: &BitmapSource) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::ImageSource> for BitmapSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::ImageSource> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::ImageSource>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::ImageSource> for &BitmapSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::ImageSource> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::ImageSource>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<BitmapSource> for super::super::DependencyObject {
    fn from(value: BitmapSource) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&BitmapSource> for super::super::DependencyObject {
    fn from(value: &BitmapSource) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for BitmapSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(
            ::core::convert::Into::<super::super::DependencyObject>::into(self),
        )
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &BitmapSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(
            ::core::convert::Into::<super::super::DependencyObject>::into(
                ::core::clone::Clone::clone(self),
            ),
        )
    }
}
unsafe impl ::core::marker::Send for BitmapSource {}
unsafe impl ::core::marker::Sync for BitmapSource {}
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DecodePixelType(pub i32);
impl DecodePixelType {
    pub const Physical: DecodePixelType = DecodePixelType(0i32);
    pub const Logical: DecodePixelType = DecodePixelType(1i32);
}
impl ::core::convert::From<i32> for DecodePixelType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DecodePixelType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DecodePixelType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Media.Imaging.DecodePixelType;i4)",
    );
}
impl ::windows::core::DefaultType for DecodePixelType {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct DownloadProgressEventArgs(pub ::windows::core::IInspectable);
impl DownloadProgressEventArgs {
    pub fn Progress(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetProgress(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
}
unsafe impl ::windows::core::RuntimeType for DownloadProgressEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Media.Imaging.DownloadProgressEventArgs;{9a0ea80b-1a17-50d5-83f3-377738212619})" ) ;
}
unsafe impl ::windows::core::Interface for DownloadProgressEventArgs {
    type Vtable = IDownloadProgressEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x9a0ea80b_1a17_50d5_83f3_377738212619);
}
impl ::windows::core::RuntimeName for DownloadProgressEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Imaging.DownloadProgressEventArgs";
}
impl ::core::convert::From<DownloadProgressEventArgs> for ::windows::core::IUnknown {
    fn from(value: DownloadProgressEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DownloadProgressEventArgs> for ::windows::core::IUnknown {
    fn from(value: &DownloadProgressEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DownloadProgressEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a DownloadProgressEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DownloadProgressEventArgs> for ::windows::core::IInspectable {
    fn from(value: DownloadProgressEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DownloadProgressEventArgs> for ::windows::core::IInspectable {
    fn from(value: &DownloadProgressEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for DownloadProgressEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a DownloadProgressEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DownloadProgressEventArgs {}
unsafe impl ::core::marker::Sync for DownloadProgressEventArgs {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct DownloadProgressEventHandler(::windows::core::IUnknown);
impl DownloadProgressEventHandler {
    pub fn new<
        F: FnMut(
                &::core::option::Option<::windows::core::IInspectable>,
                &::core::option::Option<DownloadProgressEventArgs>,
            ) -> ::windows::core::Result<()>
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = DownloadProgressEventHandler_box::<F> {
            vtable: &DownloadProgressEventHandler_box::<F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
        Param1: ::windows::core::IntoParam<'a, DownloadProgressEventArgs>,
    >(
        &self,
        sender: Param0,
        e: Param1,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).3)(
                ::core::mem::transmute_copy(this),
                sender.into_param().abi(),
                e.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::core::RuntimeType for DownloadProgressEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"delegate({9a8e4af5-b124-5205-8ae9-3496e063c569})",
    );
}
unsafe impl ::windows::core::Interface for DownloadProgressEventHandler {
    type Vtable = DownloadProgressEventHandler_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x9a8e4af5_b124_5205_8ae9_3496e063c569);
}
#[repr(C)]
#[doc(hidden)]
pub struct DownloadProgressEventHandler_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        sender: ::windows::core::RawPtr,
        e: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(C)]
struct DownloadProgressEventHandler_box<
    F: FnMut(
            &::core::option::Option<::windows::core::IInspectable>,
            &::core::option::Option<DownloadProgressEventArgs>,
        ) -> ::windows::core::Result<()>
        + 'static,
> {
    vtable: *const DownloadProgressEventHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<
        F: FnMut(
                &::core::option::Option<::windows::core::IInspectable>,
                &::core::option::Option<DownloadProgressEventArgs>,
            ) -> ::windows::core::Result<()>
            + 'static,
    > DownloadProgressEventHandler_box<F>
{
    const VTABLE: DownloadProgressEventHandler_abi = DownloadProgressEventHandler_abi(
        Self::QueryInterface,
        Self::AddRef,
        Self::Release,
        Self::Invoke,
    );
    unsafe extern "system" fn QueryInterface(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<DownloadProgressEventHandler as ::windows::core::Interface>::IID
            || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID
            || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID
        {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::core::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(
        this: ::windows::core::RawPtr,
        sender: ::windows::core::RawPtr,
        e: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ( ( * this ) . invoke ) ( & * ( & sender as * const < :: windows :: core :: IInspectable as :: windows :: core :: Abi > :: Abi as * const < :: windows :: core :: IInspectable as :: windows :: core :: DefaultType > :: DefaultType ) , & * ( & e as * const < DownloadProgressEventArgs as :: windows :: core :: Abi > :: Abi as * const < DownloadProgressEventArgs as :: windows :: core :: DefaultType > :: DefaultType ) , ) . into ( )
    }
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IBitmapImage(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBitmapImage {
    type Vtable = IBitmapImage_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x5cc29916_a411_5bc2_a3c5_a00d99a59da8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapImage_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::core::GUID,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut BitmapCreateOptions,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: BitmapCreateOptions,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: i32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: i32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut DecodePixelType,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: DecodePixelType,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBitmapImageFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBitmapImageFactory {
    type Vtable = IBitmapImageFactory_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xf037e0e9_f229_522e_95c9_da2211a14b05);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapImageFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::core::GUID,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut i32,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        urisource: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBitmapImageStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBitmapImageStatics {
    type Vtable = IBitmapImageStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x4bcf71a9_1897_51dc_8e3f_2c5c796d1cd9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapImageStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::core::GUID,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut i32,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBitmapSource(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBitmapSource {
    type Vtable = IBitmapSource_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x8424269d_9b82_534f_8fea_af5b5ef96bf2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapSource_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::core::GUID,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        streamsource: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        streamsource: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBitmapSourceFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBitmapSourceFactory {
    type Vtable = IBitmapSourceFactory_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x0392f025_1868_5876_ad67_12e94a8da5bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapSourceFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::core::GUID,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut i32,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        baseinterface: ::windows::core::RawPtr,
        innerinterface: *mut ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBitmapSourceStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBitmapSourceStatics {
    type Vtable = IBitmapSourceStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xefa3745e_4400_5f0b_bdc7_3f2911a3d719);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapSourceStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::core::GUID,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut i32,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDownloadProgressEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDownloadProgressEventArgs {
    type Vtable = IDownloadProgressEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x9a0ea80b_1a17_50d5_83f3_377738212619);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDownloadProgressEventArgs_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::core::GUID,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: i32,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRenderTargetBitmap(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRenderTargetBitmap {
    type Vtable = IRenderTargetBitmap_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xcf10407d_fa8b_57a3_9574_710529ae0b04);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRenderTargetBitmap_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::core::GUID,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Input",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Input",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Media3D"
    )))]
    usize,
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Input",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        scaledwidth: i32,
        scaledheight: i32,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Input",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Media3D"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRenderTargetBitmapStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRenderTargetBitmapStatics {
    type Vtable = IRenderTargetBitmapStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x83e822e4_9f84_5986_93b0_e4f7019c367d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRenderTargetBitmapStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::core::GUID,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut i32,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISoftwareBitmapSource(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISoftwareBitmapSource {
    type Vtable = ISoftwareBitmapSource_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xa6aca802_1f24_5a1e_bf08_781a85ed5511);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISoftwareBitmapSource_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::core::GUID,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        softwarebitmap: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISurfaceImageSource(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISurfaceImageSource {
    type Vtable = ISurfaceImageSource_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xac078d9c_d0e0_5ff9_b73e_98e82e4c8d36);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISurfaceImageSource_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::core::GUID,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut i32,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISurfaceImageSourceFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISurfaceImageSourceFactory {
    type Vtable = ISurfaceImageSourceFactory_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x09a26ed2_11b3_5ef1_ac56_20d064ccca34);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISurfaceImageSourceFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::core::GUID,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        pixelwidth: i32,
        pixelheight: i32,
        baseinterface: ::windows::core::RawPtr,
        innerinterface: *mut ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        pixelwidth: i32,
        pixelheight: i32,
        isopaque: bool,
        baseinterface: ::windows::core::RawPtr,
        innerinterface: *mut ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISvgImageSource(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISvgImageSource {
    type Vtable = ISvgImageSource_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xd5b61d3c_b68d_53a2_b07b_ba6adfdd5887);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISvgImageSource_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::core::GUID,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut f64,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: f64,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut f64,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: f64,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        streamsource: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISvgImageSourceFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISvgImageSourceFactory {
    type Vtable = ISvgImageSourceFactory_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x2f85673f_ac64_570d_9bda_94fa082eead9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISvgImageSourceFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::core::GUID,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut i32,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        baseinterface: ::windows::core::RawPtr,
        innerinterface: *mut ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        urisource: ::windows::core::RawPtr,
        baseinterface: ::windows::core::RawPtr,
        innerinterface: *mut ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISvgImageSourceFailedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISvgImageSourceFailedEventArgs {
    type Vtable = ISvgImageSourceFailedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x76e66278_7804_5439_a50d_14c5ba896714);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISvgImageSourceFailedEventArgs_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::core::GUID,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut SvgImageSourceLoadStatus,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISvgImageSourceOpenedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISvgImageSourceOpenedEventArgs {
    type Vtable = ISvgImageSourceOpenedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x1c9860d5_38d0_5b21_8d48_072f1e254e39);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISvgImageSourceOpenedEventArgs_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::core::GUID,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut i32,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISvgImageSourceStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISvgImageSourceStatics {
    type Vtable = ISvgImageSourceStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xe3ad1068_f4c6_5513_a777_2980f0ba41bd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISvgImageSourceStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::core::GUID,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut i32,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVirtualSurfaceImageSource(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVirtualSurfaceImageSource {
    type Vtable = IVirtualSurfaceImageSource_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xe4ff96a6_fede_589c_a007_4178b53b6739);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVirtualSurfaceImageSource_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::core::GUID,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut i32,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVirtualSurfaceImageSourceFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVirtualSurfaceImageSourceFactory {
    type Vtable = IVirtualSurfaceImageSourceFactory_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x08490f2c_04a8_5031_b9c7_707060d7cd48);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVirtualSurfaceImageSourceFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::core::GUID,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        pixelwidth: i32,
        pixelheight: i32,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        pixelwidth: i32,
        pixelheight: i32,
        isopaque: bool,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWriteableBitmap(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWriteableBitmap {
    type Vtable = IWriteableBitmap_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x78c824a9_0e43_5f1e_93bc_d046cca82b7e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWriteableBitmap_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::core::GUID,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWriteableBitmapFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWriteableBitmapFactory {
    type Vtable = IWriteableBitmapFactory_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x26e861d9_b080_512b_96c4_80050e7e08d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWriteableBitmapFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::core::GUID,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        pixelwidth: i32,
        pixelheight: i32,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXamlRenderingBackgroundTask(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXamlRenderingBackgroundTask {
    type Vtable = IXamlRenderingBackgroundTask_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x7807000c_a050_5121_ac74_3322d5358e39);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlRenderingBackgroundTask_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::core::GUID,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut i32,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXamlRenderingBackgroundTaskFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXamlRenderingBackgroundTaskFactory {
    type Vtable = IXamlRenderingBackgroundTaskFactory_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x205247a3_9ffe_599a_a21a_7181442a9d75);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlRenderingBackgroundTaskFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::core::GUID,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        baseinterface: ::windows::core::RawPtr,
        innerinterface: *mut ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXamlRenderingBackgroundTaskOverrides(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXamlRenderingBackgroundTaskOverrides {
    type Vtable = IXamlRenderingBackgroundTaskOverrides_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x18733237_324b_57c0_89b2_5875472acc80);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlRenderingBackgroundTaskOverrides_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::core::GUID,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        taskinstance: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct RenderTargetBitmap(pub ::windows::core::IInspectable);
impl RenderTargetBitmap {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            RenderTargetBitmap,
            ::windows::core::IActivationFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn PixelWidth(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn PixelHeight(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Input",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn RenderAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::UIElement>>(
        &self,
        element: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Input",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn RenderToSizeAsync<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::UIElement>,
    >(
        &self,
        element: Param0,
        scaledwidth: i32,
        scaledheight: i32,
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                scaledwidth,
                scaledheight,
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn GetPixelsAsync(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::IAsyncOperation<::windows::Storage::Streams::IBuffer>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IAsyncOperation<
                ::windows::Storage::Streams::IBuffer,
            >>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn PixelWidthProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IRenderTargetBitmapStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn PixelHeightProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IRenderTargetBitmapStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ClearValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ReadLocalValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetAnimationBaseValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn RegisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, super::super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::core::Result<i64> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                callback.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i64>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn UnregisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                token,
            )
            .ok()
        }
    }
    pub fn Dispatcher(&self) -> ::windows::core::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn IRenderTargetBitmapStatics<
        R,
        F: FnOnce(&IRenderTargetBitmapStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            RenderTargetBitmap,
            IRenderTargetBitmapStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for RenderTargetBitmap {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Media.Imaging.RenderTargetBitmap;{cf10407d-fa8b-57a3-9574-710529ae0b04})" ) ;
}
unsafe impl ::windows::core::Interface for RenderTargetBitmap {
    type Vtable = IRenderTargetBitmap_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xcf10407d_fa8b_57a3_9574_710529ae0b04);
}
impl ::windows::core::RuntimeName for RenderTargetBitmap {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Imaging.RenderTargetBitmap";
}
impl ::core::convert::From<RenderTargetBitmap> for ::windows::core::IUnknown {
    fn from(value: RenderTargetBitmap) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RenderTargetBitmap> for ::windows::core::IUnknown {
    fn from(value: &RenderTargetBitmap) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RenderTargetBitmap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RenderTargetBitmap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RenderTargetBitmap> for ::windows::core::IInspectable {
    fn from(value: RenderTargetBitmap) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RenderTargetBitmap> for ::windows::core::IInspectable {
    fn from(value: &RenderTargetBitmap) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RenderTargetBitmap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RenderTargetBitmap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<RenderTargetBitmap> for super::ImageSource {
    fn from(value: RenderTargetBitmap) -> Self {
        ::core::convert::Into::<super::ImageSource>::into(&value)
    }
}
impl ::core::convert::From<&RenderTargetBitmap> for super::ImageSource {
    fn from(value: &RenderTargetBitmap) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::ImageSource> for RenderTargetBitmap {
    fn into_param(self) -> ::windows::core::Param<'a, super::ImageSource> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::ImageSource>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::ImageSource> for &RenderTargetBitmap {
    fn into_param(self) -> ::windows::core::Param<'a, super::ImageSource> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::ImageSource>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<RenderTargetBitmap> for super::super::DependencyObject {
    fn from(value: RenderTargetBitmap) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&RenderTargetBitmap> for super::super::DependencyObject {
    fn from(value: &RenderTargetBitmap) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for RenderTargetBitmap {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(
            ::core::convert::Into::<super::super::DependencyObject>::into(self),
        )
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &RenderTargetBitmap {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(
            ::core::convert::Into::<super::super::DependencyObject>::into(
                ::core::clone::Clone::clone(self),
            ),
        )
    }
}
unsafe impl ::core::marker::Send for RenderTargetBitmap {}
unsafe impl ::core::marker::Sync for RenderTargetBitmap {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct SoftwareBitmapSource(pub ::windows::core::IInspectable);
impl SoftwareBitmapSource {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            SoftwareBitmapSource,
            ::windows::core::IActivationFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn SetBitmapAsync<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Graphics::Imaging::SoftwareBitmap>,
    >(
        &self,
        softwarebitmap: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                softwarebitmap.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ClearValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ReadLocalValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetAnimationBaseValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn RegisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, super::super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::core::Result<i64> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                callback.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i64>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn UnregisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                token,
            )
            .ok()
        }
    }
    pub fn Dispatcher(&self) -> ::windows::core::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for SoftwareBitmapSource {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Media.Imaging.SoftwareBitmapSource;{a6aca802-1f24-5a1e-bf08-781a85ed5511})" ) ;
}
unsafe impl ::windows::core::Interface for SoftwareBitmapSource {
    type Vtable = ISoftwareBitmapSource_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xa6aca802_1f24_5a1e_bf08_781a85ed5511);
}
impl ::windows::core::RuntimeName for SoftwareBitmapSource {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Imaging.SoftwareBitmapSource";
}
impl ::core::convert::From<SoftwareBitmapSource> for ::windows::core::IUnknown {
    fn from(value: SoftwareBitmapSource) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SoftwareBitmapSource> for ::windows::core::IUnknown {
    fn from(value: &SoftwareBitmapSource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SoftwareBitmapSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SoftwareBitmapSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SoftwareBitmapSource> for ::windows::core::IInspectable {
    fn from(value: SoftwareBitmapSource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SoftwareBitmapSource> for ::windows::core::IInspectable {
    fn from(value: &SoftwareBitmapSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SoftwareBitmapSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a SoftwareBitmapSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<SoftwareBitmapSource> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: SoftwareBitmapSource) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SoftwareBitmapSource> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &SoftwareBitmapSource) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable> for SoftwareBitmapSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable>
    for &SoftwareBitmapSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::core::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<SoftwareBitmapSource> for super::ImageSource {
    fn from(value: SoftwareBitmapSource) -> Self {
        ::core::convert::Into::<super::ImageSource>::into(&value)
    }
}
impl ::core::convert::From<&SoftwareBitmapSource> for super::ImageSource {
    fn from(value: &SoftwareBitmapSource) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::ImageSource> for SoftwareBitmapSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::ImageSource> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::ImageSource>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::ImageSource> for &SoftwareBitmapSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::ImageSource> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::ImageSource>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<SoftwareBitmapSource> for super::super::DependencyObject {
    fn from(value: SoftwareBitmapSource) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&SoftwareBitmapSource> for super::super::DependencyObject {
    fn from(value: &SoftwareBitmapSource) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for SoftwareBitmapSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(
            ::core::convert::Into::<super::super::DependencyObject>::into(self),
        )
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &SoftwareBitmapSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(
            ::core::convert::Into::<super::super::DependencyObject>::into(
                ::core::clone::Clone::clone(self),
            ),
        )
    }
}
unsafe impl ::core::marker::Send for SoftwareBitmapSource {}
unsafe impl ::core::marker::Sync for SoftwareBitmapSource {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct SurfaceImageSource(pub ::windows::core::IInspectable);
impl SurfaceImageSource {
    pub fn CreateInstanceWithDimensions(
        pixelwidth: i32,
        pixelheight: i32,
    ) -> ::windows::core::Result<SurfaceImageSource> {
        Self::ISurfaceImageSourceFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                pixelwidth,
                pixelheight,
                ::core::ptr::null_mut(),
                &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _,
                &mut result__,
            )
            .from_abi::<SurfaceImageSource>(result__)
        })
    }
    pub fn CreateInstanceWithDimensionsAndOpacity(
        pixelwidth: i32,
        pixelheight: i32,
        isopaque: bool,
    ) -> ::windows::core::Result<SurfaceImageSource> {
        Self::ISurfaceImageSourceFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                pixelwidth,
                pixelheight,
                isopaque,
                ::core::ptr::null_mut(),
                &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _,
                &mut result__,
            )
            .from_abi::<SurfaceImageSource>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ClearValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ReadLocalValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetAnimationBaseValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn RegisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, super::super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::core::Result<i64> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                callback.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i64>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn UnregisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                token,
            )
            .ok()
        }
    }
    pub fn Dispatcher(&self) -> ::windows::core::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn ISurfaceImageSourceFactory<
        R,
        F: FnOnce(&ISurfaceImageSourceFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            SurfaceImageSource,
            ISurfaceImageSourceFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for SurfaceImageSource {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Media.Imaging.SurfaceImageSource;{ac078d9c-d0e0-5ff9-b73e-98e82e4c8d36})" ) ;
}
unsafe impl ::windows::core::Interface for SurfaceImageSource {
    type Vtable = ISurfaceImageSource_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xac078d9c_d0e0_5ff9_b73e_98e82e4c8d36);
}
impl ::windows::core::RuntimeName for SurfaceImageSource {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Imaging.SurfaceImageSource";
}
impl ::core::convert::From<SurfaceImageSource> for ::windows::core::IUnknown {
    fn from(value: SurfaceImageSource) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SurfaceImageSource> for ::windows::core::IUnknown {
    fn from(value: &SurfaceImageSource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SurfaceImageSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SurfaceImageSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SurfaceImageSource> for ::windows::core::IInspectable {
    fn from(value: SurfaceImageSource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SurfaceImageSource> for ::windows::core::IInspectable {
    fn from(value: &SurfaceImageSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SurfaceImageSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SurfaceImageSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<SurfaceImageSource> for super::ImageSource {
    fn from(value: SurfaceImageSource) -> Self {
        ::core::convert::Into::<super::ImageSource>::into(&value)
    }
}
impl ::core::convert::From<&SurfaceImageSource> for super::ImageSource {
    fn from(value: &SurfaceImageSource) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::ImageSource> for SurfaceImageSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::ImageSource> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::ImageSource>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::ImageSource> for &SurfaceImageSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::ImageSource> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::ImageSource>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<SurfaceImageSource> for super::super::DependencyObject {
    fn from(value: SurfaceImageSource) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&SurfaceImageSource> for super::super::DependencyObject {
    fn from(value: &SurfaceImageSource) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for SurfaceImageSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(
            ::core::convert::Into::<super::super::DependencyObject>::into(self),
        )
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &SurfaceImageSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(
            ::core::convert::Into::<super::super::DependencyObject>::into(
                ::core::clone::Clone::clone(self),
            ),
        )
    }
}
unsafe impl ::core::marker::Send for SurfaceImageSource {}
unsafe impl ::core::marker::Sync for SurfaceImageSource {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct SvgImageSource(pub ::windows::core::IInspectable);
impl SvgImageSource {
    pub fn UriSource(&self) -> ::windows::core::Result<::windows::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Uri>(result__)
        }
    }
    pub fn SetUriSource<'a, Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Uri>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn RasterizePixelWidth(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetRasterizePixelWidth(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn RasterizePixelHeight(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetRasterizePixelHeight(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn Opened<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<SvgImageSource, SvgImageSourceOpenedEventArgs>,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveOpened<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn OpenFailed<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<SvgImageSource, SvgImageSourceFailedEventArgs>,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveOpenFailed<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn SetSourceAsync<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Storage::Streams::IRandomAccessStream>,
    >(
        &self,
        streamsource: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncOperation<SvgImageSourceLoadStatus>>
    {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(
                ::core::mem::transmute_copy(this),
                streamsource.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IAsyncOperation<SvgImageSourceLoadStatus>>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn UriSourceProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISvgImageSourceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn RasterizePixelWidthProperty() -> ::windows::core::Result<super::super::DependencyProperty>
    {
        Self::ISvgImageSourceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn RasterizePixelHeightProperty(
    ) -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISvgImageSourceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn new() -> ::windows::core::Result<SvgImageSource> {
        Self::ISvgImageSourceFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                ::core::ptr::null_mut(),
                &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _,
                &mut result__,
            )
            .from_abi::<SvgImageSource>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn CreateInstanceWithUriSource<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Uri>,
    >(
        urisource: Param0,
    ) -> ::windows::core::Result<SvgImageSource> {
        Self::ISvgImageSourceFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                urisource.into_param().abi(),
                ::core::ptr::null_mut(),
                &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _,
                &mut result__,
            )
            .from_abi::<SvgImageSource>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ClearValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ReadLocalValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetAnimationBaseValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn RegisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, super::super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::core::Result<i64> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                callback.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i64>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn UnregisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                token,
            )
            .ok()
        }
    }
    pub fn Dispatcher(&self) -> ::windows::core::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn ISvgImageSourceStatics<
        R,
        F: FnOnce(&ISvgImageSourceStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SvgImageSource, ISvgImageSourceStatics> =
            ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ISvgImageSourceFactory<
        R,
        F: FnOnce(&ISvgImageSourceFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SvgImageSource, ISvgImageSourceFactory> =
            ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for SvgImageSource {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Media.Imaging.SvgImageSource;{d5b61d3c-b68d-53a2-b07b-ba6adfdd5887})" ) ;
}
unsafe impl ::windows::core::Interface for SvgImageSource {
    type Vtable = ISvgImageSource_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xd5b61d3c_b68d_53a2_b07b_ba6adfdd5887);
}
impl ::windows::core::RuntimeName for SvgImageSource {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Imaging.SvgImageSource";
}
impl ::core::convert::From<SvgImageSource> for ::windows::core::IUnknown {
    fn from(value: SvgImageSource) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SvgImageSource> for ::windows::core::IUnknown {
    fn from(value: &SvgImageSource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SvgImageSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SvgImageSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SvgImageSource> for ::windows::core::IInspectable {
    fn from(value: SvgImageSource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SvgImageSource> for ::windows::core::IInspectable {
    fn from(value: &SvgImageSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SvgImageSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SvgImageSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<SvgImageSource> for super::ImageSource {
    fn from(value: SvgImageSource) -> Self {
        ::core::convert::Into::<super::ImageSource>::into(&value)
    }
}
impl ::core::convert::From<&SvgImageSource> for super::ImageSource {
    fn from(value: &SvgImageSource) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::ImageSource> for SvgImageSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::ImageSource> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::ImageSource>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::ImageSource> for &SvgImageSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::ImageSource> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::ImageSource>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<SvgImageSource> for super::super::DependencyObject {
    fn from(value: SvgImageSource) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&SvgImageSource> for super::super::DependencyObject {
    fn from(value: &SvgImageSource) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for SvgImageSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(
            ::core::convert::Into::<super::super::DependencyObject>::into(self),
        )
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &SvgImageSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(
            ::core::convert::Into::<super::super::DependencyObject>::into(
                ::core::clone::Clone::clone(self),
            ),
        )
    }
}
unsafe impl ::core::marker::Send for SvgImageSource {}
unsafe impl ::core::marker::Sync for SvgImageSource {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct SvgImageSourceFailedEventArgs(pub ::windows::core::IInspectable);
impl SvgImageSourceFailedEventArgs {
    pub fn Status(&self) -> ::windows::core::Result<SvgImageSourceLoadStatus> {
        let this = self;
        unsafe {
            let mut result__: SvgImageSourceLoadStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SvgImageSourceLoadStatus>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for SvgImageSourceFailedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Media.Imaging.SvgImageSourceFailedEventArgs;{76e66278-7804-5439-a50d-14c5ba896714})" ) ;
}
unsafe impl ::windows::core::Interface for SvgImageSourceFailedEventArgs {
    type Vtable = ISvgImageSourceFailedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x76e66278_7804_5439_a50d_14c5ba896714);
}
impl ::windows::core::RuntimeName for SvgImageSourceFailedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Imaging.SvgImageSourceFailedEventArgs";
}
impl ::core::convert::From<SvgImageSourceFailedEventArgs> for ::windows::core::IUnknown {
    fn from(value: SvgImageSourceFailedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SvgImageSourceFailedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &SvgImageSourceFailedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for SvgImageSourceFailedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a SvgImageSourceFailedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SvgImageSourceFailedEventArgs> for ::windows::core::IInspectable {
    fn from(value: SvgImageSourceFailedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SvgImageSourceFailedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &SvgImageSourceFailedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for SvgImageSourceFailedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a SvgImageSourceFailedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SvgImageSourceFailedEventArgs {}
unsafe impl ::core::marker::Sync for SvgImageSourceFailedEventArgs {}
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct SvgImageSourceLoadStatus(pub i32);
impl SvgImageSourceLoadStatus {
    pub const Success: SvgImageSourceLoadStatus = SvgImageSourceLoadStatus(0i32);
    pub const NetworkError: SvgImageSourceLoadStatus = SvgImageSourceLoadStatus(1i32);
    pub const InvalidFormat: SvgImageSourceLoadStatus = SvgImageSourceLoadStatus(2i32);
    pub const Other: SvgImageSourceLoadStatus = SvgImageSourceLoadStatus(3i32);
}
impl ::core::convert::From<i32> for SvgImageSourceLoadStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SvgImageSourceLoadStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for SvgImageSourceLoadStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Media.Imaging.SvgImageSourceLoadStatus;i4)",
    );
}
impl ::windows::core::DefaultType for SvgImageSourceLoadStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct SvgImageSourceOpenedEventArgs(pub ::windows::core::IInspectable);
impl SvgImageSourceOpenedEventArgs {}
unsafe impl ::windows::core::RuntimeType for SvgImageSourceOpenedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Media.Imaging.SvgImageSourceOpenedEventArgs;{1c9860d5-38d0-5b21-8d48-072f1e254e39})" ) ;
}
unsafe impl ::windows::core::Interface for SvgImageSourceOpenedEventArgs {
    type Vtable = ISvgImageSourceOpenedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x1c9860d5_38d0_5b21_8d48_072f1e254e39);
}
impl ::windows::core::RuntimeName for SvgImageSourceOpenedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Imaging.SvgImageSourceOpenedEventArgs";
}
impl ::core::convert::From<SvgImageSourceOpenedEventArgs> for ::windows::core::IUnknown {
    fn from(value: SvgImageSourceOpenedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SvgImageSourceOpenedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &SvgImageSourceOpenedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for SvgImageSourceOpenedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a SvgImageSourceOpenedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SvgImageSourceOpenedEventArgs> for ::windows::core::IInspectable {
    fn from(value: SvgImageSourceOpenedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SvgImageSourceOpenedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &SvgImageSourceOpenedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for SvgImageSourceOpenedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a SvgImageSourceOpenedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SvgImageSourceOpenedEventArgs {}
unsafe impl ::core::marker::Sync for SvgImageSourceOpenedEventArgs {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct VirtualSurfaceImageSource(pub ::windows::core::IInspectable);
impl VirtualSurfaceImageSource {
    pub fn CreateInstanceWithDimensions(
        pixelwidth: i32,
        pixelheight: i32,
    ) -> ::windows::core::Result<VirtualSurfaceImageSource> {
        Self::IVirtualSurfaceImageSourceFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                pixelwidth,
                pixelheight,
                &mut result__,
            )
            .from_abi::<VirtualSurfaceImageSource>(result__)
        })
    }
    pub fn CreateInstanceWithDimensionsAndOpacity(
        pixelwidth: i32,
        pixelheight: i32,
        isopaque: bool,
    ) -> ::windows::core::Result<VirtualSurfaceImageSource> {
        Self::IVirtualSurfaceImageSourceFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                pixelwidth,
                pixelheight,
                isopaque,
                &mut result__,
            )
            .from_abi::<VirtualSurfaceImageSource>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ClearValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ReadLocalValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetAnimationBaseValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn RegisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, super::super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::core::Result<i64> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                callback.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i64>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn UnregisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                token,
            )
            .ok()
        }
    }
    pub fn Dispatcher(&self) -> ::windows::core::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn IVirtualSurfaceImageSourceFactory<
        R,
        F: FnOnce(&IVirtualSurfaceImageSourceFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            VirtualSurfaceImageSource,
            IVirtualSurfaceImageSourceFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for VirtualSurfaceImageSource {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Media.Imaging.VirtualSurfaceImageSource;{e4ff96a6-fede-589c-a007-4178b53b6739})" ) ;
}
unsafe impl ::windows::core::Interface for VirtualSurfaceImageSource {
    type Vtable = IVirtualSurfaceImageSource_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xe4ff96a6_fede_589c_a007_4178b53b6739);
}
impl ::windows::core::RuntimeName for VirtualSurfaceImageSource {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Imaging.VirtualSurfaceImageSource";
}
impl ::core::convert::From<VirtualSurfaceImageSource> for ::windows::core::IUnknown {
    fn from(value: VirtualSurfaceImageSource) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VirtualSurfaceImageSource> for ::windows::core::IUnknown {
    fn from(value: &VirtualSurfaceImageSource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VirtualSurfaceImageSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a VirtualSurfaceImageSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VirtualSurfaceImageSource> for ::windows::core::IInspectable {
    fn from(value: VirtualSurfaceImageSource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VirtualSurfaceImageSource> for ::windows::core::IInspectable {
    fn from(value: &VirtualSurfaceImageSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for VirtualSurfaceImageSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a VirtualSurfaceImageSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<VirtualSurfaceImageSource> for SurfaceImageSource {
    fn from(value: VirtualSurfaceImageSource) -> Self {
        ::core::convert::Into::<SurfaceImageSource>::into(&value)
    }
}
impl ::core::convert::From<&VirtualSurfaceImageSource> for SurfaceImageSource {
    fn from(value: &VirtualSurfaceImageSource) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, SurfaceImageSource> for VirtualSurfaceImageSource {
    fn into_param(self) -> ::windows::core::Param<'a, SurfaceImageSource> {
        ::windows::core::Param::Owned(::core::convert::Into::<SurfaceImageSource>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, SurfaceImageSource> for &VirtualSurfaceImageSource {
    fn into_param(self) -> ::windows::core::Param<'a, SurfaceImageSource> {
        ::windows::core::Param::Owned(::core::convert::Into::<SurfaceImageSource>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<VirtualSurfaceImageSource> for super::ImageSource {
    fn from(value: VirtualSurfaceImageSource) -> Self {
        ::core::convert::Into::<super::ImageSource>::into(&value)
    }
}
impl ::core::convert::From<&VirtualSurfaceImageSource> for super::ImageSource {
    fn from(value: &VirtualSurfaceImageSource) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::ImageSource> for VirtualSurfaceImageSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::ImageSource> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::ImageSource>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::ImageSource> for &VirtualSurfaceImageSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::ImageSource> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::ImageSource>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<VirtualSurfaceImageSource> for super::super::DependencyObject {
    fn from(value: VirtualSurfaceImageSource) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&VirtualSurfaceImageSource> for super::super::DependencyObject {
    fn from(value: &VirtualSurfaceImageSource) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject>
    for VirtualSurfaceImageSource
{
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(
            ::core::convert::Into::<super::super::DependencyObject>::into(self),
        )
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject>
    for &VirtualSurfaceImageSource
{
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(
            ::core::convert::Into::<super::super::DependencyObject>::into(
                ::core::clone::Clone::clone(self),
            ),
        )
    }
}
unsafe impl ::core::marker::Send for VirtualSurfaceImageSource {}
unsafe impl ::core::marker::Sync for VirtualSurfaceImageSource {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct WriteableBitmap(pub ::windows::core::IInspectable);
impl WriteableBitmap {
    pub fn PixelBuffer(&self) -> ::windows::core::Result<::windows::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Storage::Streams::IBuffer>(result__)
        }
    }
    pub fn Invalidate(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this)).ok()
        }
    }
    pub fn CreateInstanceWithDimensions(
        pixelwidth: i32,
        pixelheight: i32,
    ) -> ::windows::core::Result<WriteableBitmap> {
        Self::IWriteableBitmapFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                pixelwidth,
                pixelheight,
                &mut result__,
            )
            .from_abi::<WriteableBitmap>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ClearValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ReadLocalValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetAnimationBaseValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn RegisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, super::super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::core::Result<i64> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                callback.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i64>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn UnregisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                token,
            )
            .ok()
        }
    }
    pub fn Dispatcher(&self) -> ::windows::core::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn PixelWidth(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<IBitmapSource>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn PixelHeight(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<IBitmapSource>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetSource<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Storage::Streams::IRandomAccessStream>,
    >(
        &self,
        streamsource: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBitmapSource>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                streamsource.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn SetSourceAsync<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Storage::Streams::IRandomAccessStream>,
    >(
        &self,
        streamsource: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IBitmapSource>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                streamsource.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn IWriteableBitmapFactory<
        R,
        F: FnOnce(&IWriteableBitmapFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WriteableBitmap, IWriteableBitmapFactory> =
            ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for WriteableBitmap {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Media.Imaging.WriteableBitmap;{78c824a9-0e43-5f1e-93bc-d046cca82b7e})" ) ;
}
unsafe impl ::windows::core::Interface for WriteableBitmap {
    type Vtable = IWriteableBitmap_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x78c824a9_0e43_5f1e_93bc_d046cca82b7e);
}
impl ::windows::core::RuntimeName for WriteableBitmap {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Imaging.WriteableBitmap";
}
impl ::core::convert::From<WriteableBitmap> for ::windows::core::IUnknown {
    fn from(value: WriteableBitmap) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&WriteableBitmap> for ::windows::core::IUnknown {
    fn from(value: &WriteableBitmap) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WriteableBitmap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WriteableBitmap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<WriteableBitmap> for ::windows::core::IInspectable {
    fn from(value: WriteableBitmap) -> Self {
        value.0
    }
}
impl ::core::convert::From<&WriteableBitmap> for ::windows::core::IInspectable {
    fn from(value: &WriteableBitmap) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WriteableBitmap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WriteableBitmap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<WriteableBitmap> for BitmapSource {
    fn from(value: WriteableBitmap) -> Self {
        ::core::convert::Into::<BitmapSource>::into(&value)
    }
}
impl ::core::convert::From<&WriteableBitmap> for BitmapSource {
    fn from(value: &WriteableBitmap) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, BitmapSource> for WriteableBitmap {
    fn into_param(self) -> ::windows::core::Param<'a, BitmapSource> {
        ::windows::core::Param::Owned(::core::convert::Into::<BitmapSource>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, BitmapSource> for &WriteableBitmap {
    fn into_param(self) -> ::windows::core::Param<'a, BitmapSource> {
        ::windows::core::Param::Owned(::core::convert::Into::<BitmapSource>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<WriteableBitmap> for super::ImageSource {
    fn from(value: WriteableBitmap) -> Self {
        ::core::convert::Into::<super::ImageSource>::into(&value)
    }
}
impl ::core::convert::From<&WriteableBitmap> for super::ImageSource {
    fn from(value: &WriteableBitmap) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::ImageSource> for WriteableBitmap {
    fn into_param(self) -> ::windows::core::Param<'a, super::ImageSource> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::ImageSource>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::ImageSource> for &WriteableBitmap {
    fn into_param(self) -> ::windows::core::Param<'a, super::ImageSource> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::ImageSource>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<WriteableBitmap> for super::super::DependencyObject {
    fn from(value: WriteableBitmap) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&WriteableBitmap> for super::super::DependencyObject {
    fn from(value: &WriteableBitmap) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for WriteableBitmap {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(
            ::core::convert::Into::<super::super::DependencyObject>::into(self),
        )
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &WriteableBitmap {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(
            ::core::convert::Into::<super::super::DependencyObject>::into(
                ::core::clone::Clone::clone(self),
            ),
        )
    }
}
unsafe impl ::core::marker::Send for WriteableBitmap {}
unsafe impl ::core::marker::Sync for WriteableBitmap {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct XamlRenderingBackgroundTask(pub ::windows::core::IInspectable);
impl XamlRenderingBackgroundTask {}
unsafe impl ::windows::core::RuntimeType for XamlRenderingBackgroundTask {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Media.Imaging.XamlRenderingBackgroundTask;{7807000c-a050-5121-ac74-3322d5358e39})" ) ;
}
unsafe impl ::windows::core::Interface for XamlRenderingBackgroundTask {
    type Vtable = IXamlRenderingBackgroundTask_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x7807000c_a050_5121_ac74_3322d5358e39);
}
impl ::windows::core::RuntimeName for XamlRenderingBackgroundTask {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Imaging.XamlRenderingBackgroundTask";
}
impl ::core::convert::From<XamlRenderingBackgroundTask> for ::windows::core::IUnknown {
    fn from(value: XamlRenderingBackgroundTask) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&XamlRenderingBackgroundTask> for ::windows::core::IUnknown {
    fn from(value: &XamlRenderingBackgroundTask) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for XamlRenderingBackgroundTask {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a XamlRenderingBackgroundTask
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<XamlRenderingBackgroundTask> for ::windows::core::IInspectable {
    fn from(value: XamlRenderingBackgroundTask) -> Self {
        value.0
    }
}
impl ::core::convert::From<&XamlRenderingBackgroundTask> for ::windows::core::IInspectable {
    fn from(value: &XamlRenderingBackgroundTask) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for XamlRenderingBackgroundTask
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a XamlRenderingBackgroundTask
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for XamlRenderingBackgroundTask {}
unsafe impl ::core::marker::Sync for XamlRenderingBackgroundTask {}
