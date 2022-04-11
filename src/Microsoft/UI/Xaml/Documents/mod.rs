#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clashing_extern_declarations,
    clippy::all
)]
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct Block(::windows::core::IUnknown);
impl Block {
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn TextAlignment(&self) -> ::windows::core::Result<super::TextAlignment> {
        let this = self;
        unsafe {
            let mut result__: super::TextAlignment = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TextAlignment)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::TextAlignment>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetTextAlignment(&self, value: super::TextAlignment) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetTextAlignment)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn HorizontalTextAlignment(&self) -> ::windows::core::Result<super::TextAlignment> {
        let this = self;
        unsafe {
            let mut result__: super::TextAlignment = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HorizontalTextAlignment)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::TextAlignment>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetHorizontalTextAlignment(
        &self,
        value: super::TextAlignment,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetHorizontalTextAlignment)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn LineHeight(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LineHeight)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetLineHeight(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetLineHeight)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn LineStackingStrategy(&self) -> ::windows::core::Result<super::LineStackingStrategy> {
        let this = self;
        unsafe {
            let mut result__: super::LineStackingStrategy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LineStackingStrategy)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::LineStackingStrategy>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetLineStackingStrategy(
        &self,
        value: super::LineStackingStrategy,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetLineStackingStrategy)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Margin(&self) -> ::windows::core::Result<super::Thickness> {
        let this = self;
        unsafe {
            let mut result__: super::Thickness = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Margin)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Thickness>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetMargin<'a, Param0: ::windows::core::IntoParam<'a, super::Thickness>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetMargin)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn TextAlignmentProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IBlockStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TextAlignmentProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn HorizontalTextAlignmentProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IBlockStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HorizontalTextAlignmentProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn LineHeightProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IBlockStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LineHeightProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn LineStackingStrategyProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IBlockStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LineStackingStrategyProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn MarginProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IBlockStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MarginProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ClearValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).ClearValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ReadLocalValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReadLocalValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetAnimationBaseValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAnimationBaseValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RegisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::core::Result<i64> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RegisterPropertyChangedCallback)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                callback.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn UnregisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).UnregisterPropertyChangedCallback)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Dispatcher(&self) -> ::windows::core::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Dispatcher)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Dispatching\"`*"]
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DispatcherQueue)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontSize(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontSize)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFontSize(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontSize)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows::core::Result<super::Media::FontFamily> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontFamily)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFontFamily<'a, Param0: ::windows::core::IntoParam<'a, super::Media::FontFamily>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontFamily)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontWeight(&self) -> ::windows::core::Result<::windows::UI::Text::FontWeight> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontWeight = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontWeight)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFontWeight<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::UI::Text::FontWeight>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontWeight)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontStyle(&self) -> ::windows::core::Result<::windows::UI::Text::FontStyle> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStyle = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontStyle)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStyle>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontStyle)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontStretch(&self) -> ::windows::core::Result<::windows::UI::Text::FontStretch> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStretch = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontStretch)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStretch>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontStretch)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn CharacterSpacing(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CharacterSpacing)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetCharacterSpacing)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Foreground(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Foreground)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetForeground<'a, Param0: ::windows::core::IntoParam<'a, super::Media::Brush>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetForeground)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Language)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetLanguage)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsTextScaleFactorEnabled)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsTextScaleFactorEnabled)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn TextDecorations(&self) -> ::windows::core::Result<::windows::UI::Text::TextDecorations> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::TextDecorations = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TextDecorations)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::TextDecorations>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetTextDecorations)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ContentStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContentStart)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ContentEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContentEnd)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ElementStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ElementStart)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ElementEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ElementEnd)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn AllowFocusOnInteraction(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AllowFocusOnInteraction)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetAllowFocusOnInteraction)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn AccessKey(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKey)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetAccessKey<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetAccessKey)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExitDisplayModeOnAccessKeyInvoked)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetExitDisplayModeOnAccessKeyInvoked)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn IsAccessKeyScope(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsAccessKeyScope)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsAccessKeyScope)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn AccessKeyScopeOwner(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyScopeOwner)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetAccessKeyScopeOwner<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetAccessKeyScopeOwner)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::core::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::Input::KeyTipPlacementMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyTipPlacementMode)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetKeyTipPlacementMode)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyTipHorizontalOffset)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetKeyTipHorizontalOffset)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn KeyTipVerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyTipVerticalOffset)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetKeyTipVerticalOffset)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn XamlRoot(&self) -> ::windows::core::Result<super::XamlRoot> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XamlRoot)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::XamlRoot>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetXamlRoot<'a, Param0: ::windows::core::IntoParam<'a, super::XamlRoot>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetXamlRoot)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayRequested<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyDisplayRequested)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveAccessKeyDisplayRequested<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveAccessKeyDisplayRequested)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayDismissedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyDisplayDismissed)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveAccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveAccessKeyDisplayDismissed)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyInvoked<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyInvokedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyInvoked)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveAccessKeyInvoked<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveAccessKeyInvoked)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FindName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindName)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IBlockStatics<R, F: FnOnce(&IBlockStatics) -> ::windows::core::Result<R>>(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Block, IBlockStatics> =
            ::windows::core::FactoryCache::from_library(b"Microsoft.UI.Xaml.dll\0");
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Block {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Block {}
impl ::core::fmt::Debug for Block {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Block").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Block {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.Block;{8149d507-672f-5fd5-a10a-351389ba9659})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for Block {
    type Vtable = IBlock_Vtbl;
    const IID: ::windows::core::GUID = <IBlock as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Block {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Block";
}
impl ::core::convert::From<Block> for ::windows::core::IUnknown {
    fn from(value: Block) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Block> for ::windows::core::IUnknown {
    fn from(value: &Block) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Block {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Block {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Block> for ::windows::core::IInspectable {
    fn from(value: Block) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Block> for ::windows::core::IInspectable {
    fn from(value: &Block) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Block {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Block {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Block> for TextElement {
    fn from(value: Block) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Block> for TextElement {
    fn from(value: &Block) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for Block {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for &Block {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<TextElement>::into(self))
    }
}
impl ::core::convert::From<Block> for super::DependencyObject {
    fn from(value: Block) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Block> for super::DependencyObject {
    fn from(value: &Block) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for Block {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &Block {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for Block {}
unsafe impl ::core::marker::Sync for Block {}
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct BlockCollection(::windows::core::IUnknown);
impl BlockCollection {
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn First(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Collections::IIterator<Block>> {
        let this = &::windows::core::Interface::cast::<
            ::windows::Foundation::Collections::IIterable<Block>,
        >(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).First)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IIterator<Block>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<Block> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAt)(
                ::core::mem::transmute_copy(this),
                index,
                &mut result__,
            )
            .from_abi::<Block>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Size)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetView(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Collections::IVectorView<Block>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetView)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IVectorView<Block>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, Block>>(
        &self,
        value: Param0,
        index: &mut u32,
    ) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IndexOf)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
                index,
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, Block>>(
        &self,
        index: u32,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetAt)(
                ::core::mem::transmute_copy(this),
                index,
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, Block>>(
        &self,
        index: u32,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).InsertAt)(
                ::core::mem::transmute_copy(this),
                index,
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveAt)(
                ::core::mem::transmute_copy(this),
                index,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Append<'a, Param0: ::windows::core::IntoParam<'a, Block>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).Append)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveAtEnd)(::core::mem::transmute_copy(
                this,
            ))
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).Clear)(::core::mem::transmute_copy(this)).ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetMany(
        &self,
        startindex: u32,
        items: &mut [::core::option::Option<Block>],
    ) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetMany)(
                ::core::mem::transmute_copy(this),
                startindex,
                items.len() as u32,
                ::core::mem::transmute_copy(&items),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ReplaceAll(
        &self,
        items: &[::core::option::Option<Block>],
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).ReplaceAll)(
                ::core::mem::transmute_copy(this),
                items.len() as u32,
                ::core::mem::transmute(items.as_ptr()),
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for BlockCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BlockCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BlockCollection {}
impl ::core::fmt::Debug for BlockCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BlockCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BlockCollection {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Documents.BlockCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Microsoft.UI.Xaml.Documents.Block;{8149d507-672f-5fd5-a10a-351389ba9659})))" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for BlockCollection {
    type Vtable = ::windows::Foundation::Collections::IVector_Vtbl<Block>;
    const IID: ::windows::core::GUID =
        <::windows::Foundation::Collections::IVector<Block> as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BlockCollection {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.BlockCollection";
}
impl ::core::iter::IntoIterator for BlockCollection {
    type Item = Block;
    type IntoIter = ::windows::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
impl ::core::iter::IntoIterator for &BlockCollection {
    type Item = Block;
    type IntoIter = ::windows::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::windows::Foundation::Collections::VectorIterator::new(
            ::core::convert::TryInto::try_into(self).ok(),
        )
    }
}
impl ::core::convert::From<BlockCollection> for ::windows::core::IUnknown {
    fn from(value: BlockCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BlockCollection> for ::windows::core::IUnknown {
    fn from(value: &BlockCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BlockCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BlockCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BlockCollection> for ::windows::core::IInspectable {
    fn from(value: BlockCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BlockCollection> for ::windows::core::IInspectable {
    fn from(value: &BlockCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BlockCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BlockCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<BlockCollection>
    for ::windows::Foundation::Collections::IIterable<Block>
{
    type Error = ::windows::core::Error;
    fn try_from(value: BlockCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&BlockCollection>
    for ::windows::Foundation::Collections::IIterable<Block>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &BlockCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::Collections::IIterable<Block>>
    for BlockCollection
{
    fn into_param(
        self,
    ) -> ::windows::core::Param<'a, ::windows::Foundation::Collections::IIterable<Block>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::Collections::IIterable<Block>>
    for &BlockCollection
{
    fn into_param(
        self,
    ) -> ::windows::core::Param<'a, ::windows::Foundation::Collections::IIterable<Block>> {
        ::core::convert::TryInto::<::windows::Foundation::Collections::IIterable<Block>>::try_into(
            self,
        )
        .map(::windows::core::Param::Owned)
        .unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<BlockCollection>
    for ::windows::Foundation::Collections::IVector<Block>
{
    type Error = ::windows::core::Error;
    fn try_from(value: BlockCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&BlockCollection>
    for ::windows::Foundation::Collections::IVector<Block>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &BlockCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::Collections::IVector<Block>>
    for BlockCollection
{
    fn into_param(
        self,
    ) -> ::windows::core::Param<'a, ::windows::Foundation::Collections::IVector<Block>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::Collections::IVector<Block>>
    for &BlockCollection
{
    fn into_param(
        self,
    ) -> ::windows::core::Param<'a, ::windows::Foundation::Collections::IVector<Block>> {
        ::core::convert::TryInto::<::windows::Foundation::Collections::IVector<Block>>::try_into(
            self,
        )
        .map(::windows::core::Param::Owned)
        .unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for BlockCollection {}
unsafe impl ::core::marker::Sync for BlockCollection {}
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct Bold(::windows::core::IUnknown);
impl Bold {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            Bold,
            ::windows::core::IActivationFactory,
        > = ::windows::core::FactoryCache::from_library(b"Microsoft.UI.Xaml.dll\0");
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ClearValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).ClearValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ReadLocalValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReadLocalValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetAnimationBaseValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAnimationBaseValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RegisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::core::Result<i64> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RegisterPropertyChangedCallback)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                callback.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn UnregisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).UnregisterPropertyChangedCallback)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Dispatcher(&self) -> ::windows::core::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Dispatcher)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Dispatching\"`*"]
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DispatcherQueue)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Inlines(&self) -> ::windows::core::Result<InlineCollection> {
        let this = &::windows::core::Interface::cast::<ISpan>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Inlines)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InlineCollection>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetInlines<'a, Param0: ::windows::core::IntoParam<'a, InlineCollection>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISpan>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetInlines)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontSize(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontSize)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFontSize(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontSize)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows::core::Result<super::Media::FontFamily> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontFamily)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFontFamily<'a, Param0: ::windows::core::IntoParam<'a, super::Media::FontFamily>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontFamily)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontWeight(&self) -> ::windows::core::Result<::windows::UI::Text::FontWeight> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontWeight = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontWeight)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFontWeight<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::UI::Text::FontWeight>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontWeight)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontStyle(&self) -> ::windows::core::Result<::windows::UI::Text::FontStyle> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStyle = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontStyle)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStyle>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontStyle)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontStretch(&self) -> ::windows::core::Result<::windows::UI::Text::FontStretch> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStretch = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontStretch)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStretch>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontStretch)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn CharacterSpacing(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CharacterSpacing)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetCharacterSpacing)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Foreground(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Foreground)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetForeground<'a, Param0: ::windows::core::IntoParam<'a, super::Media::Brush>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetForeground)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Language)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetLanguage)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsTextScaleFactorEnabled)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsTextScaleFactorEnabled)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn TextDecorations(&self) -> ::windows::core::Result<::windows::UI::Text::TextDecorations> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::TextDecorations = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TextDecorations)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::TextDecorations>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetTextDecorations)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ContentStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContentStart)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ContentEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContentEnd)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ElementStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ElementStart)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ElementEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ElementEnd)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn AllowFocusOnInteraction(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AllowFocusOnInteraction)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetAllowFocusOnInteraction)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn AccessKey(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKey)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetAccessKey<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetAccessKey)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExitDisplayModeOnAccessKeyInvoked)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetExitDisplayModeOnAccessKeyInvoked)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn IsAccessKeyScope(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsAccessKeyScope)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsAccessKeyScope)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn AccessKeyScopeOwner(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyScopeOwner)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetAccessKeyScopeOwner<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetAccessKeyScopeOwner)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::core::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::Input::KeyTipPlacementMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyTipPlacementMode)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetKeyTipPlacementMode)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyTipHorizontalOffset)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetKeyTipHorizontalOffset)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn KeyTipVerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyTipVerticalOffset)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetKeyTipVerticalOffset)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn XamlRoot(&self) -> ::windows::core::Result<super::XamlRoot> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XamlRoot)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::XamlRoot>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetXamlRoot<'a, Param0: ::windows::core::IntoParam<'a, super::XamlRoot>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetXamlRoot)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayRequested<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyDisplayRequested)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveAccessKeyDisplayRequested<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveAccessKeyDisplayRequested)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayDismissedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyDisplayDismissed)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveAccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveAccessKeyDisplayDismissed)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyInvoked<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyInvokedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyInvoked)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveAccessKeyInvoked<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveAccessKeyInvoked)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FindName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindName)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
impl ::core::clone::Clone for Bold {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Bold {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Bold {}
impl ::core::fmt::Debug for Bold {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Bold").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Bold {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.Bold;{241a5f5a-c164-597f-b0db-fac7431297f2})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for Bold {
    type Vtable = IBold_Vtbl;
    const IID: ::windows::core::GUID = <IBold as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Bold {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Bold";
}
impl ::core::convert::From<Bold> for ::windows::core::IUnknown {
    fn from(value: Bold) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Bold> for ::windows::core::IUnknown {
    fn from(value: &Bold) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Bold {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Bold {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Bold> for ::windows::core::IInspectable {
    fn from(value: Bold) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Bold> for ::windows::core::IInspectable {
    fn from(value: &Bold) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Bold {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Bold {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Bold> for Span {
    fn from(value: Bold) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Bold> for Span {
    fn from(value: &Bold) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Span> for Bold {
    fn into_param(self) -> ::windows::core::Param<'a, Span> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Span> for &Bold {
    fn into_param(self) -> ::windows::core::Param<'a, Span> {
        ::windows::core::Param::Owned(::core::convert::Into::<Span>::into(self))
    }
}
impl ::core::convert::From<Bold> for Inline {
    fn from(value: Bold) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Bold> for Inline {
    fn from(value: &Bold) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Inline> for Bold {
    fn into_param(self) -> ::windows::core::Param<'a, Inline> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Inline> for &Bold {
    fn into_param(self) -> ::windows::core::Param<'a, Inline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Inline>::into(self))
    }
}
impl ::core::convert::From<Bold> for TextElement {
    fn from(value: Bold) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Bold> for TextElement {
    fn from(value: &Bold) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for Bold {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for &Bold {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<TextElement>::into(self))
    }
}
impl ::core::convert::From<Bold> for super::DependencyObject {
    fn from(value: Bold) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Bold> for super::DependencyObject {
    fn from(value: &Bold) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for Bold {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &Bold {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for Bold {}
unsafe impl ::core::marker::Sync for Bold {}
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct Glyphs(::windows::core::IUnknown);
impl Glyphs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            Glyphs,
            ::windows::core::IActivationFactory,
        > = ::windows::core::FactoryCache::from_library(b"Microsoft.UI.Xaml.dll\0");
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn PopulatePropertyInfo<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, super::super::Composition::AnimationPropertyInfo>,
    >(
        &self,
        propertyname: Param0,
        propertyinfo: Param1,
    ) -> ::windows::core::Result<()> {
        let this =
            &::windows::core::Interface::cast::<super::super::Composition::IAnimationObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).PopulatePropertyInfo)(
                ::core::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                propertyinfo.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ClearValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).ClearValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ReadLocalValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReadLocalValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetAnimationBaseValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAnimationBaseValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RegisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::core::Result<i64> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RegisterPropertyChangedCallback)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                callback.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn UnregisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).UnregisterPropertyChangedCallback)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Dispatcher(&self) -> ::windows::core::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Dispatcher)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Dispatching\"`*"]
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DispatcherQueue)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Triggers(&self) -> ::windows::core::Result<super::TriggerCollection> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Triggers)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::TriggerCollection>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Resources(&self) -> ::windows::core::Result<super::ResourceDictionary> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Resources)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ResourceDictionary>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetResources<'a, Param0: ::windows::core::IntoParam<'a, super::ResourceDictionary>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetResources)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Tag(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Tag)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetTag<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetTag)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Language)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetLanguage)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ActualWidth(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ActualWidth)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ActualHeight(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ActualHeight)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Width(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Width)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetWidth(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetWidth)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Height(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Height)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetHeight(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetHeight)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn MinWidth(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MinWidth)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetMinWidth(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetMinWidth)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn MaxWidth(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MaxWidth)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetMaxWidth(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetMaxWidth)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn MinHeight(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MinHeight)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetMinHeight(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetMinHeight)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn MaxHeight(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MaxHeight)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetMaxHeight(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetMaxHeight)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn HorizontalAlignment(&self) -> ::windows::core::Result<super::HorizontalAlignment> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: super::HorizontalAlignment = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HorizontalAlignment)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::HorizontalAlignment>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetHorizontalAlignment(
        &self,
        value: super::HorizontalAlignment,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetHorizontalAlignment)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn VerticalAlignment(&self) -> ::windows::core::Result<super::VerticalAlignment> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: super::VerticalAlignment = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VerticalAlignment)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::VerticalAlignment>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetVerticalAlignment(
        &self,
        value: super::VerticalAlignment,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetVerticalAlignment)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Margin(&self) -> ::windows::core::Result<super::Thickness> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: super::Thickness = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Margin)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Thickness>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetMargin<'a, Param0: ::windows::core::IntoParam<'a, super::Thickness>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetMargin)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetName)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn BaseUri(&self) -> ::windows::core::Result<::windows::Foundation::Uri> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BaseUri)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn DataContext(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DataContext)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetDataContext<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetDataContext)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn AllowFocusOnInteraction(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AllowFocusOnInteraction)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetAllowFocusOnInteraction)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FocusVisualMargin(&self) -> ::windows::core::Result<super::Thickness> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: super::Thickness = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FocusVisualMargin)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Thickness>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFocusVisualMargin<'a, Param0: ::windows::core::IntoParam<'a, super::Thickness>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFocusVisualMargin)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FocusVisualSecondaryThickness(&self) -> ::windows::core::Result<super::Thickness> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: super::Thickness = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FocusVisualSecondaryThickness)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Thickness>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFocusVisualSecondaryThickness<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::Thickness>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFocusVisualSecondaryThickness)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FocusVisualPrimaryThickness(&self) -> ::windows::core::Result<super::Thickness> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: super::Thickness = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FocusVisualPrimaryThickness)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Thickness>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFocusVisualPrimaryThickness<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::Thickness>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFocusVisualPrimaryThickness)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FocusVisualSecondaryBrush(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FocusVisualSecondaryBrush)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFocusVisualSecondaryBrush<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::Media::Brush>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFocusVisualSecondaryBrush)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FocusVisualPrimaryBrush(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FocusVisualPrimaryBrush)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFocusVisualPrimaryBrush<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::Media::Brush>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFocusVisualPrimaryBrush)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn AllowFocusWhenDisabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AllowFocusWhenDisabled)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetAllowFocusWhenDisabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetAllowFocusWhenDisabled)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Style(&self) -> ::windows::core::Result<super::Style> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Style)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Style>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetStyle<'a, Param0: ::windows::core::IntoParam<'a, super::Style>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetStyle)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Parent(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Parent)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FlowDirection(&self) -> ::windows::core::Result<super::FlowDirection> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: super::FlowDirection = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FlowDirection)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::FlowDirection>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFlowDirection(&self, value: super::FlowDirection) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFlowDirection)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RequestedTheme(&self) -> ::windows::core::Result<super::ElementTheme> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: super::ElementTheme = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestedTheme)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ElementTheme>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetRequestedTheme(&self, value: super::ElementTheme) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetRequestedTheme)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn IsLoaded(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsLoaded)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ActualTheme(&self) -> ::windows::core::Result<super::ElementTheme> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: super::ElementTheme = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ActualTheme)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ElementTheme>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Loaded<'a, Param0: ::windows::core::IntoParam<'a, super::RoutedEventHandler>>(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Loaded)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveLoaded<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveLoaded)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Unloaded<'a, Param0: ::windows::core::IntoParam<'a, super::RoutedEventHandler>>(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Unloaded)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveUnloaded<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveUnloaded)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn DataContextChanged<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                super::FrameworkElement,
                super::DataContextChangedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DataContextChanged)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveDataContextChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveDataContextChanged)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SizeChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::SizeChangedEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SizeChanged)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveSizeChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveSizeChanged)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn LayoutUpdated<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::EventHandler<::windows::core::IInspectable>,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LayoutUpdated)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveLayoutUpdated<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveLayoutUpdated)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Loading<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                super::FrameworkElement,
                ::windows::core::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Loading)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveLoading<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveLoading)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ActualThemeChanged<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                super::FrameworkElement,
                ::windows::core::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ActualThemeChanged)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveActualThemeChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveActualThemeChanged)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn EffectiveViewportChanged<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                super::FrameworkElement,
                super::EffectiveViewportChangedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EffectiveViewportChanged)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveEffectiveViewportChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveEffectiveViewportChanged)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FindName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindName)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Data\"`*"]
    #[cfg(feature = "UI_Xaml_Data")]
    pub fn SetBinding<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, super::Data::BindingBase>,
    >(
        &self,
        dp: Param0,
        binding: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetBinding)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                binding.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Data\"`*"]
    #[cfg(feature = "UI_Xaml_Data")]
    pub fn GetBindingExpression<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<super::Data::BindingExpression> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetBindingExpression)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::Data::BindingExpression>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn InvalidateViewport(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElementProtected>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).InvalidateViewport)(
                ::core::mem::transmute_copy(this),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn UnicodeString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UnicodeString)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetUnicodeString<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetUnicodeString)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Indices(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Indices)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetIndices<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIndices)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontUri(&self) -> ::windows::core::Result<::windows::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontUri)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFontUri<'a, Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Uri>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontUri)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn StyleSimulations(&self) -> ::windows::core::Result<super::Media::StyleSimulations> {
        let this = self;
        unsafe {
            let mut result__: super::Media::StyleSimulations = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StyleSimulations)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::StyleSimulations>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetStyleSimulations(
        &self,
        value: super::Media::StyleSimulations,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetStyleSimulations)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontRenderingEmSize(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontRenderingEmSize)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFontRenderingEmSize(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontRenderingEmSize)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn OriginX(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OriginX)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetOriginX(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetOriginX)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn OriginY(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OriginY)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetOriginY(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetOriginY)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Fill(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Fill)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFill<'a, Param0: ::windows::core::IntoParam<'a, super::Media::Brush>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFill)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn IsColorFontEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsColorFontEnabled)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetIsColorFontEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsColorFontEnabled)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ColorFontPaletteIndex(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ColorFontPaletteIndex)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetColorFontPaletteIndex(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetColorFontPaletteIndex)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn UnicodeStringProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UnicodeStringProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn IndicesProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IndicesProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontUriProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontUriProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn StyleSimulationsProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StyleSimulationsProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontRenderingEmSizeProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontRenderingEmSizeProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn OriginXProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OriginXProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn OriginYProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OriginYProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FillProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FillProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn IsColorFontEnabledProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsColorFontEnabledProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ColorFontPaletteIndexProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ColorFontPaletteIndexProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn DesiredSize(&self) -> ::windows::core::Result<::windows::Foundation::Size> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::Size = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DesiredSize)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Size>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn AllowDrop(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AllowDrop)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetAllowDrop(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetAllowDrop)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Opacity(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Opacity)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetOpacity(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetOpacity)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Clip(&self) -> ::windows::core::Result<super::Media::RectangleGeometry> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Clip)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::RectangleGeometry>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetClip<'a, Param0: ::windows::core::IntoParam<'a, super::Media::RectangleGeometry>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetClip)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn RenderTransform(&self) -> ::windows::core::Result<super::Media::Transform> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RenderTransform)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Transform>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetRenderTransform<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::Media::Transform>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetRenderTransform)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Projection(&self) -> ::windows::core::Result<super::Media::Projection> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Projection)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Projection>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetProjection<'a, Param0: ::windows::core::IntoParam<'a, super::Media::Projection>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetProjection)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media_Media3D\"`*"]
    #[cfg(feature = "UI_Xaml_Media_Media3D")]
    pub fn Transform3D(&self) -> ::windows::core::Result<super::Media::Media3D::Transform3D> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Transform3D)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Media3D::Transform3D>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media_Media3D\"`*"]
    #[cfg(feature = "UI_Xaml_Media_Media3D")]
    pub fn SetTransform3D<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::Media::Media3D::Transform3D>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetTransform3D)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RenderTransformOrigin(&self) -> ::windows::core::Result<::windows::Foundation::Point> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RenderTransformOrigin)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetRenderTransformOrigin<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Point>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetRenderTransformOrigin)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn IsHitTestVisible(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsHitTestVisible)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetIsHitTestVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsHitTestVisible)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Visibility(&self) -> ::windows::core::Result<super::Visibility> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::Visibility = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Visibility)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Visibility>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetVisibility(&self, value: super::Visibility) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetVisibility)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RenderSize(&self) -> ::windows::core::Result<::windows::Foundation::Size> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::Size = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RenderSize)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Size>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn UseLayoutRounding(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UseLayoutRounding)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetUseLayoutRounding(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetUseLayoutRounding)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media_Animation\"`*"]
    #[cfg(feature = "UI_Xaml_Media_Animation")]
    pub fn Transitions(
        &self,
    ) -> ::windows::core::Result<super::Media::Animation::TransitionCollection> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Transitions)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Animation::TransitionCollection>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media_Animation\"`*"]
    #[cfg(feature = "UI_Xaml_Media_Animation")]
    pub fn SetTransitions<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::Media::Animation::TransitionCollection>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetTransitions)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn CacheMode(&self) -> ::windows::core::Result<super::Media::CacheMode> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CacheMode)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::CacheMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetCacheMode<'a, Param0: ::windows::core::IntoParam<'a, super::Media::CacheMode>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetCacheMode)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn IsTapEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsTapEnabled)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetIsTapEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsTapEnabled)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn IsDoubleTapEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsDoubleTapEnabled)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetIsDoubleTapEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsDoubleTapEnabled)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn CanDrag(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanDrag)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetCanDrag(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetCanDrag)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn IsRightTapEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsRightTapEnabled)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetIsRightTapEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsRightTapEnabled)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn IsHoldingEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsHoldingEnabled)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetIsHoldingEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsHoldingEnabled)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationMode(&self) -> ::windows::core::Result<super::Input::ManipulationModes> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::Input::ManipulationModes = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ManipulationMode)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::ManipulationModes>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetManipulationMode(
        &self,
        value: super::Input::ManipulationModes,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetManipulationMode)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerCaptures(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IVectorView<super::Input::Pointer>,
    > {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointerCaptures)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IVectorView<super::Input::Pointer>>(
                result__,
            )
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Controls_Primitives\"`*"]
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub fn ContextFlyout(
        &self,
    ) -> ::windows::core::Result<super::Controls::Primitives::FlyoutBase> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContextFlyout)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Controls::Primitives::FlyoutBase>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Controls_Primitives\"`*"]
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub fn SetContextFlyout<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::Controls::Primitives::FlyoutBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetContextFlyout)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn CompositeMode(&self) -> ::windows::core::Result<super::Media::ElementCompositeMode> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::Media::ElementCompositeMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CompositeMode)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::ElementCompositeMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetCompositeMode(
        &self,
        value: super::Media::ElementCompositeMode,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetCompositeMode)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Lights(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Collections::IVector<super::Media::XamlLight>>
    {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Lights)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IVector<super::Media::XamlLight>>(
                result__,
            )
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn CanBeScrollAnchor(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanBeScrollAnchor)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetCanBeScrollAnchor(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetCanBeScrollAnchor)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExitDisplayModeOnAccessKeyInvoked)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetExitDisplayModeOnAccessKeyInvoked)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn IsAccessKeyScope(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsAccessKeyScope)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsAccessKeyScope)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn AccessKeyScopeOwner(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyScopeOwner)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetAccessKeyScopeOwner<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetAccessKeyScopeOwner)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn AccessKey(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKey)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetAccessKey<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetAccessKey)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::core::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::Input::KeyTipPlacementMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyTipPlacementMode)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetKeyTipPlacementMode)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyTipHorizontalOffset)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetKeyTipHorizontalOffset)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn KeyTipVerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyTipVerticalOffset)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetKeyTipVerticalOffset)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn KeyTipTarget(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyTipTarget)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetKeyTipTarget<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetKeyTipTarget)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusKeyboardNavigation(
        &self,
    ) -> ::windows::core::Result<super::Input::XYFocusKeyboardNavigationMode> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::Input::XYFocusKeyboardNavigationMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusKeyboardNavigation)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::XYFocusKeyboardNavigationMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusKeyboardNavigation(
        &self,
        value: super::Input::XYFocusKeyboardNavigationMode,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetXYFocusKeyboardNavigation)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusUpNavigationStrategy(
        &self,
    ) -> ::windows::core::Result<super::Input::XYFocusNavigationStrategy> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::Input::XYFocusNavigationStrategy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusUpNavigationStrategy)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::XYFocusNavigationStrategy>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusUpNavigationStrategy(
        &self,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetXYFocusUpNavigationStrategy)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusDownNavigationStrategy(
        &self,
    ) -> ::windows::core::Result<super::Input::XYFocusNavigationStrategy> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::Input::XYFocusNavigationStrategy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusDownNavigationStrategy)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::XYFocusNavigationStrategy>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusDownNavigationStrategy(
        &self,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetXYFocusDownNavigationStrategy)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusLeftNavigationStrategy(
        &self,
    ) -> ::windows::core::Result<super::Input::XYFocusNavigationStrategy> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::Input::XYFocusNavigationStrategy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusLeftNavigationStrategy)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::XYFocusNavigationStrategy>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusLeftNavigationStrategy(
        &self,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetXYFocusLeftNavigationStrategy)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusRightNavigationStrategy(
        &self,
    ) -> ::windows::core::Result<super::Input::XYFocusNavigationStrategy> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::Input::XYFocusNavigationStrategy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusRightNavigationStrategy)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::XYFocusNavigationStrategy>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusRightNavigationStrategy(
        &self,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetXYFocusRightNavigationStrategy)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyboardAccelerators(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IVector<super::Input::KeyboardAccelerator>,
    > {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyboardAccelerators)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IVector<
                super::Input::KeyboardAccelerator,
            >>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn KeyboardAcceleratorPlacementTarget(
        &self,
    ) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyboardAcceleratorPlacementTarget)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetKeyboardAcceleratorPlacementTarget<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetKeyboardAcceleratorPlacementTarget)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyboardAcceleratorPlacementMode(
        &self,
    ) -> ::windows::core::Result<super::Input::KeyboardAcceleratorPlacementMode> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::Input::KeyboardAcceleratorPlacementMode =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyboardAcceleratorPlacementMode)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyboardAcceleratorPlacementMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyboardAcceleratorPlacementMode(
        &self,
        value: super::Input::KeyboardAcceleratorPlacementMode,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetKeyboardAcceleratorPlacementMode)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn HighContrastAdjustment(
        &self,
    ) -> ::windows::core::Result<super::ElementHighContrastAdjustment> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::ElementHighContrastAdjustment = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HighContrastAdjustment)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ElementHighContrastAdjustment>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetHighContrastAdjustment(
        &self,
        value: super::ElementHighContrastAdjustment,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetHighContrastAdjustment)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn TabFocusNavigation(
        &self,
    ) -> ::windows::core::Result<super::Input::KeyboardNavigationMode> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::Input::KeyboardNavigationMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TabFocusNavigation)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyboardNavigationMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetTabFocusNavigation(
        &self,
        value: super::Input::KeyboardNavigationMode,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetTabFocusNavigation)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn OpacityTransition(&self) -> ::windows::core::Result<super::ScalarTransition> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OpacityTransition)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ScalarTransition>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetOpacityTransition<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::ScalarTransition>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetOpacityTransition)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Translation(&self) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector3> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Translation)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetTranslation<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Numerics::Vector3>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetTranslation)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn TranslationTransition(&self) -> ::windows::core::Result<super::Vector3Transition> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TranslationTransition)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Vector3Transition>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetTranslationTransition<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::Vector3Transition>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetTranslationTransition)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Rotation(&self) -> ::windows::core::Result<f32> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Rotation)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetRotation(&self, value: f32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetRotation)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RotationTransition(&self) -> ::windows::core::Result<super::ScalarTransition> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RotationTransition)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ScalarTransition>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetRotationTransition<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::ScalarTransition>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetRotationTransition)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Scale(&self) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector3> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Scale)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetScale<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Numerics::Vector3>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetScale)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ScaleTransition(&self) -> ::windows::core::Result<super::Vector3Transition> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ScaleTransition)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Vector3Transition>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetScaleTransition<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::Vector3Transition>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetScaleTransition)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn TransformMatrix(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Numerics::Matrix4x4> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Matrix4x4 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TransformMatrix)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Matrix4x4>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetTransformMatrix<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Numerics::Matrix4x4>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetTransformMatrix)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn CenterPoint(&self) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector3> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CenterPoint)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetCenterPoint<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Numerics::Vector3>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetCenterPoint)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RotationAxis(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector3> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RotationAxis)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetRotationAxis<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Numerics::Vector3>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetRotationAxis)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ActualOffset(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector3> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ActualOffset)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ActualSize(&self) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector2> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector2 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ActualSize)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector2>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn XamlRoot(&self) -> ::windows::core::Result<super::XamlRoot> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XamlRoot)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::XamlRoot>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetXamlRoot<'a, Param0: ::windows::core::IntoParam<'a, super::XamlRoot>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetXamlRoot)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Shadow(&self) -> ::windows::core::Result<super::Media::Shadow> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Shadow)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Shadow>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetShadow<'a, Param0: ::windows::core::IntoParam<'a, super::Media::Shadow>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetShadow)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RasterizationScale(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RasterizationScale)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetRasterizationScale(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetRasterizationScale)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FocusState(&self) -> ::windows::core::Result<super::FocusState> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::FocusState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FocusState)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::FocusState>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn UseSystemFocusVisuals(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UseSystemFocusVisuals)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetUseSystemFocusVisuals(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetUseSystemFocusVisuals)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn XYFocusLeft(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusLeft)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetXYFocusLeft<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetXYFocusLeft)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn XYFocusRight(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusRight)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetXYFocusRight<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetXYFocusRight)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn XYFocusUp(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusUp)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetXYFocusUp<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetXYFocusUp)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn XYFocusDown(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusDown)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetXYFocusDown<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetXYFocusDown)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn IsTabStop(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsTabStop)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetIsTabStop(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsTabStop)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn TabIndex(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TabIndex)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetTabIndex(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetTabIndex)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyUp<'a, Param0: ::windows::core::IntoParam<'a, super::Input::KeyEventHandler>>(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyUp)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveKeyUp<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveKeyUp)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyDown<'a, Param0: ::windows::core::IntoParam<'a, super::Input::KeyEventHandler>>(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyDown)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveKeyDown<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveKeyDown)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GotFocus<'a, Param0: ::windows::core::IntoParam<'a, super::RoutedEventHandler>>(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GotFocus)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveGotFocus<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveGotFocus)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn LostFocus<'a, Param0: ::windows::core::IntoParam<'a, super::RoutedEventHandler>>(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LostFocus)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveLostFocus<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveLostFocus)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn DragStarting<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                super::UIElement,
                super::DragStartingEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DragStarting)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveDragStarting<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveDragStarting)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn DropCompleted<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                super::UIElement,
                super::DropCompletedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DropCompleted)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveDropCompleted<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveDropCompleted)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn CharacterReceived<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                super::UIElement,
                super::Input::CharacterReceivedRoutedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CharacterReceived)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveCharacterReceived<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveCharacterReceived)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn DragEnter<'a, Param0: ::windows::core::IntoParam<'a, super::DragEventHandler>>(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DragEnter)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveDragEnter<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveDragEnter)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn DragLeave<'a, Param0: ::windows::core::IntoParam<'a, super::DragEventHandler>>(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DragLeave)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveDragLeave<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveDragLeave)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn DragOver<'a, Param0: ::windows::core::IntoParam<'a, super::DragEventHandler>>(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DragOver)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveDragOver<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveDragOver)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Drop<'a, Param0: ::windows::core::IntoParam<'a, super::DragEventHandler>>(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Drop)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveDrop<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveDrop)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerPressed<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::Input::PointerEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointerPressed)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemovePointerPressed<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemovePointerPressed)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerMoved<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::Input::PointerEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointerMoved)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemovePointerMoved<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemovePointerMoved)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerReleased<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::Input::PointerEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointerReleased)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemovePointerReleased<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemovePointerReleased)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerEntered<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::Input::PointerEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointerEntered)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemovePointerEntered<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemovePointerEntered)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerExited<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::Input::PointerEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointerExited)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemovePointerExited<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemovePointerExited)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerCaptureLost<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::Input::PointerEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointerCaptureLost)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemovePointerCaptureLost<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemovePointerCaptureLost)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerCanceled<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::Input::PointerEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointerCanceled)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemovePointerCanceled<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemovePointerCanceled)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerWheelChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::Input::PointerEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointerWheelChanged)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemovePointerWheelChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemovePointerWheelChanged)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn Tapped<'a, Param0: ::windows::core::IntoParam<'a, super::Input::TappedEventHandler>>(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Tapped)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveTapped<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveTapped)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn DoubleTapped<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::Input::DoubleTappedEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DoubleTapped)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveDoubleTapped<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveDoubleTapped)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn Holding<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::Input::HoldingEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Holding)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveHolding<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveHolding)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ContextRequested<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                super::UIElement,
                super::Input::ContextRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContextRequested)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveContextRequested<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveContextRequested)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ContextCanceled<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<super::UIElement, super::RoutedEventArgs>,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContextCanceled)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveContextCanceled<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveContextCanceled)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn RightTapped<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::Input::RightTappedEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RightTapped)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveRightTapped<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveRightTapped)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationStarting<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::Input::ManipulationStartingEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ManipulationStarting)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveManipulationStarting<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveManipulationStarting)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationInertiaStarting<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::Input::ManipulationInertiaStartingEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ManipulationInertiaStarting)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveManipulationInertiaStarting<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveManipulationInertiaStarting)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationStarted<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::Input::ManipulationStartedEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ManipulationStarted)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveManipulationStarted<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveManipulationStarted)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationDelta<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::Input::ManipulationDeltaEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ManipulationDelta)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveManipulationDelta<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveManipulationDelta)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationCompleted<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::Input::ManipulationCompletedEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ManipulationCompleted)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveManipulationCompleted<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveManipulationCompleted)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayRequested<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                super::UIElement,
                super::Input::AccessKeyDisplayRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyDisplayRequested)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveAccessKeyDisplayRequested<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveAccessKeyDisplayRequested)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                super::UIElement,
                super::Input::AccessKeyDisplayDismissedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyDisplayDismissed)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveAccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveAccessKeyDisplayDismissed)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyInvoked<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                super::UIElement,
                super::Input::AccessKeyInvokedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyInvoked)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveAccessKeyInvoked<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveAccessKeyInvoked)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ProcessKeyboardAccelerators<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                super::UIElement,
                super::Input::ProcessKeyboardAcceleratorEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProcessKeyboardAccelerators)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveProcessKeyboardAccelerators<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveProcessKeyboardAccelerators)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn GettingFocus<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                super::UIElement,
                super::Input::GettingFocusEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GettingFocus)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveGettingFocus<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveGettingFocus)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn LosingFocus<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                super::UIElement,
                super::Input::LosingFocusEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LosingFocus)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveLosingFocus<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveLosingFocus)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn NoFocusCandidateFound<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                super::UIElement,
                super::Input::NoFocusCandidateFoundEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NoFocusCandidateFound)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveNoFocusCandidateFound<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveNoFocusCandidateFound)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PreviewKeyDown<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::Input::KeyEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PreviewKeyDown)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemovePreviewKeyDown<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemovePreviewKeyDown)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PreviewKeyUp<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::Input::KeyEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PreviewKeyUp)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemovePreviewKeyUp<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemovePreviewKeyUp)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn BringIntoViewRequested<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                super::UIElement,
                super::BringIntoViewRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BringIntoViewRequested)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveBringIntoViewRequested<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveBringIntoViewRequested)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Measure<'a, Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Size>>(
        &self,
        availablesize: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).Measure)(
                ::core::mem::transmute_copy(this),
                availablesize.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Arrange<'a, Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Rect>>(
        &self,
        finalrect: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).Arrange)(
                ::core::mem::transmute_copy(this),
                finalrect.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn CapturePointer<'a, Param0: ::windows::core::IntoParam<'a, super::Input::Pointer>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CapturePointer)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ReleasePointerCapture<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::Input::Pointer>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).ReleasePointerCapture)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ReleasePointerCaptures(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).ReleasePointerCaptures)(
                ::core::mem::transmute_copy(this),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn AddHandler<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::RoutedEvent>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        routedevent: Param0,
        handler: Param1,
        handledeventstoo: bool,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).AddHandler)(
                ::core::mem::transmute_copy(this),
                routedevent.into_param().abi(),
                handler.into_param().abi(),
                handledeventstoo,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveHandler<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::RoutedEvent>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        routedevent: Param0,
        handler: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveHandler)(
                ::core::mem::transmute_copy(this),
                routedevent.into_param().abi(),
                handler.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn TransformToVisual<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(
        &self,
        visual: Param0,
    ) -> ::windows::core::Result<super::Media::GeneralTransform> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TransformToVisual)(
                ::core::mem::transmute_copy(this),
                visual.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::Media::GeneralTransform>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn InvalidateMeasure(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).InvalidateMeasure)(
                ::core::mem::transmute_copy(this),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn InvalidateArrange(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).InvalidateArrange)(
                ::core::mem::transmute_copy(this),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn UpdateLayout(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).UpdateLayout)(::core::mem::transmute_copy(
                this,
            ))
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn CancelDirectManipulations(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CancelDirectManipulations)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Input\"`*"]
    #[cfg(feature = "UI_Input")]
    pub fn StartDragAsync<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::Input::PointerPoint>,
    >(
        &self,
        pointerpoint: Param0,
    ) -> ::windows::core::Result<
        ::windows::Foundation::IAsyncOperation<
            ::windows::ApplicationModel::DataTransfer::DataPackageOperation,
        >,
    > {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartDragAsync)(
                ::core::mem::transmute_copy(this),
                pointerpoint.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IAsyncOperation<
                ::windows::ApplicationModel::DataTransfer::DataPackageOperation,
            >>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn StartBringIntoView(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).StartBringIntoView)(
                ::core::mem::transmute_copy(this),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn StartBringIntoViewWithOptions<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::BringIntoViewOptions>,
    >(
        &self,
        options: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).StartBringIntoViewWithOptions)(
                ::core::mem::transmute_copy(this),
                options.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn TryInvokeKeyboardAccelerator<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::Input::ProcessKeyboardAcceleratorEventArgs>,
    >(
        &self,
        args: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).TryInvokeKeyboardAccelerator)(
                ::core::mem::transmute_copy(this),
                args.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Focus(&self, value: super::FocusState) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Focus)(
                ::core::mem::transmute_copy(this),
                value,
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn StartAnimation<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::Composition::ICompositionAnimationBase>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).StartAnimation)(
                ::core::mem::transmute_copy(this),
                animation.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn StopAnimation<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::Composition::ICompositionAnimationBase>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).StopAnimation)(
                ::core::mem::transmute_copy(this),
                animation.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Input\"`*"]
    #[cfg(feature = "UI_Input")]
    pub fn ProtectedCursor(&self) -> ::windows::core::Result<super::super::Input::InputCursor> {
        let this = &::windows::core::Interface::cast::<super::IUIElementProtected>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProtectedCursor)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Input::InputCursor>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Input\"`*"]
    #[cfg(feature = "UI_Input")]
    pub fn SetProtectedCursor<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::Input::InputCursor>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElementProtected>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetProtectedCursor)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn GetVisualInternal(&self) -> ::windows::core::Result<super::super::Composition::Visual> {
        let this =
            &::windows::core::Interface::cast::<super::super::Composition::IVisualElement2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetVisualInternal)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Composition::Visual>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IGlyphsStatics<R, F: FnOnce(&IGlyphsStatics) -> ::windows::core::Result<R>>(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Glyphs, IGlyphsStatics> =
            ::windows::core::FactoryCache::from_library(b"Microsoft.UI.Xaml.dll\0");
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Glyphs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Glyphs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Glyphs {}
impl ::core::fmt::Debug for Glyphs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Glyphs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Glyphs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.Glyphs;{0fbf8cfe-18e7-5e45-9fa3-d2d0927958f4})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for Glyphs {
    type Vtable = IGlyphs_Vtbl;
    const IID: ::windows::core::GUID = <IGlyphs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Glyphs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Glyphs";
}
impl ::core::convert::From<Glyphs> for ::windows::core::IUnknown {
    fn from(value: Glyphs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Glyphs> for ::windows::core::IUnknown {
    fn from(value: &Glyphs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Glyphs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Glyphs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Glyphs> for ::windows::core::IInspectable {
    fn from(value: Glyphs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Glyphs> for ::windows::core::IInspectable {
    fn from(value: &Glyphs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Glyphs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Glyphs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<Glyphs> for super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: Glyphs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&Glyphs> for super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &Glyphs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IAnimationObject> for Glyphs {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IAnimationObject> for &Glyphs {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::Composition::IAnimationObject>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<Glyphs> for super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: Glyphs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&Glyphs> for super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: &Glyphs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IVisualElement> for Glyphs {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IVisualElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IVisualElement> for &Glyphs {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::Composition::IVisualElement>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<Glyphs> for super::super::Composition::IVisualElement2 {
    type Error = ::windows::core::Error;
    fn try_from(value: Glyphs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&Glyphs> for super::super::Composition::IVisualElement2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &Glyphs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IVisualElement2> for Glyphs {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IVisualElement2> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IVisualElement2> for &Glyphs {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IVisualElement2> {
        ::core::convert::TryInto::<super::super::Composition::IVisualElement2>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<Glyphs> for super::FrameworkElement {
    fn from(value: Glyphs) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Glyphs> for super::FrameworkElement {
    fn from(value: &Glyphs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::FrameworkElement> for Glyphs {
    fn into_param(self) -> ::windows::core::Param<'a, super::FrameworkElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::FrameworkElement> for &Glyphs {
    fn into_param(self) -> ::windows::core::Param<'a, super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::FrameworkElement>::into(self))
    }
}
impl ::core::convert::From<Glyphs> for super::UIElement {
    fn from(value: Glyphs) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Glyphs> for super::UIElement {
    fn from(value: &Glyphs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::UIElement> for Glyphs {
    fn into_param(self) -> ::windows::core::Param<'a, super::UIElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::UIElement> for &Glyphs {
    fn into_param(self) -> ::windows::core::Param<'a, super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::UIElement>::into(self))
    }
}
impl ::core::convert::From<Glyphs> for super::DependencyObject {
    fn from(value: Glyphs) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Glyphs> for super::DependencyObject {
    fn from(value: &Glyphs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for Glyphs {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &Glyphs {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for Glyphs {}
unsafe impl ::core::marker::Sync for Glyphs {}
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct Hyperlink(::windows::core::IUnknown);
impl Hyperlink {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            Hyperlink,
            ::windows::core::IActivationFactory,
        > = ::windows::core::FactoryCache::from_library(b"Microsoft.UI.Xaml.dll\0");
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ClearValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).ClearValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ReadLocalValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReadLocalValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetAnimationBaseValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAnimationBaseValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RegisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::core::Result<i64> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RegisterPropertyChangedCallback)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                callback.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn UnregisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).UnregisterPropertyChangedCallback)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Dispatcher(&self) -> ::windows::core::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Dispatcher)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Dispatching\"`*"]
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DispatcherQueue)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn NavigateUri(&self) -> ::windows::core::Result<::windows::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NavigateUri)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetNavigateUri<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Uri>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetNavigateUri)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn UnderlineStyle(&self) -> ::windows::core::Result<UnderlineStyle> {
        let this = self;
        unsafe {
            let mut result__: UnderlineStyle = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UnderlineStyle)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<UnderlineStyle>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetUnderlineStyle(&self, value: UnderlineStyle) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetUnderlineStyle)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn XYFocusLeft(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusLeft)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetXYFocusLeft<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetXYFocusLeft)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn XYFocusRight(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusRight)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetXYFocusRight<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetXYFocusRight)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn XYFocusUp(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusUp)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetXYFocusUp<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetXYFocusUp)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn XYFocusDown(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusDown)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetXYFocusDown<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetXYFocusDown)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ElementSoundMode(&self) -> ::windows::core::Result<super::ElementSoundMode> {
        let this = self;
        unsafe {
            let mut result__: super::ElementSoundMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ElementSoundMode)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ElementSoundMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetElementSoundMode(
        &self,
        value: super::ElementSoundMode,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetElementSoundMode)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FocusState(&self) -> ::windows::core::Result<super::FocusState> {
        let this = self;
        unsafe {
            let mut result__: super::FocusState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FocusState)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::FocusState>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusUpNavigationStrategy(
        &self,
    ) -> ::windows::core::Result<super::Input::XYFocusNavigationStrategy> {
        let this = self;
        unsafe {
            let mut result__: super::Input::XYFocusNavigationStrategy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusUpNavigationStrategy)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::XYFocusNavigationStrategy>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusUpNavigationStrategy(
        &self,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetXYFocusUpNavigationStrategy)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusDownNavigationStrategy(
        &self,
    ) -> ::windows::core::Result<super::Input::XYFocusNavigationStrategy> {
        let this = self;
        unsafe {
            let mut result__: super::Input::XYFocusNavigationStrategy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusDownNavigationStrategy)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::XYFocusNavigationStrategy>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusDownNavigationStrategy(
        &self,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetXYFocusDownNavigationStrategy)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusLeftNavigationStrategy(
        &self,
    ) -> ::windows::core::Result<super::Input::XYFocusNavigationStrategy> {
        let this = self;
        unsafe {
            let mut result__: super::Input::XYFocusNavigationStrategy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusLeftNavigationStrategy)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::XYFocusNavigationStrategy>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusLeftNavigationStrategy(
        &self,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetXYFocusLeftNavigationStrategy)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusRightNavigationStrategy(
        &self,
    ) -> ::windows::core::Result<super::Input::XYFocusNavigationStrategy> {
        let this = self;
        unsafe {
            let mut result__: super::Input::XYFocusNavigationStrategy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusRightNavigationStrategy)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::XYFocusNavigationStrategy>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusRightNavigationStrategy(
        &self,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetXYFocusRightNavigationStrategy)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn IsTabStop(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsTabStop)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetIsTabStop(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsTabStop)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn TabIndex(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TabIndex)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetTabIndex(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetTabIndex)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Click<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<Hyperlink, HyperlinkClickEventArgs>,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Click)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveClick<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveClick)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GotFocus<'a, Param0: ::windows::core::IntoParam<'a, super::RoutedEventHandler>>(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GotFocus)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveGotFocus<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveGotFocus)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn LostFocus<'a, Param0: ::windows::core::IntoParam<'a, super::RoutedEventHandler>>(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LostFocus)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveLostFocus<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveLostFocus)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Focus(&self, value: super::FocusState) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Focus)(
                ::core::mem::transmute_copy(this),
                value,
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn NavigateUriProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NavigateUriProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn UnderlineStyleProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UnderlineStyleProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn XYFocusLeftProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusLeftProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn XYFocusRightProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusRightProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn XYFocusUpProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusUpProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn XYFocusDownProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusDownProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ElementSoundModeProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ElementSoundModeProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FocusStateProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FocusStateProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn XYFocusUpNavigationStrategyProperty(
    ) -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusUpNavigationStrategyProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn XYFocusDownNavigationStrategyProperty(
    ) -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusDownNavigationStrategyProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn XYFocusLeftNavigationStrategyProperty(
    ) -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusLeftNavigationStrategyProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn XYFocusRightNavigationStrategyProperty(
    ) -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusRightNavigationStrategyProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn IsTabStopProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsTabStopProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn TabIndexProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TabIndexProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Inlines(&self) -> ::windows::core::Result<InlineCollection> {
        let this = &::windows::core::Interface::cast::<ISpan>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Inlines)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InlineCollection>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetInlines<'a, Param0: ::windows::core::IntoParam<'a, InlineCollection>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISpan>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetInlines)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontSize(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontSize)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFontSize(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontSize)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows::core::Result<super::Media::FontFamily> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontFamily)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFontFamily<'a, Param0: ::windows::core::IntoParam<'a, super::Media::FontFamily>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontFamily)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontWeight(&self) -> ::windows::core::Result<::windows::UI::Text::FontWeight> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontWeight = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontWeight)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFontWeight<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::UI::Text::FontWeight>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontWeight)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontStyle(&self) -> ::windows::core::Result<::windows::UI::Text::FontStyle> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStyle = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontStyle)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStyle>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontStyle)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontStretch(&self) -> ::windows::core::Result<::windows::UI::Text::FontStretch> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStretch = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontStretch)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStretch>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontStretch)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn CharacterSpacing(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CharacterSpacing)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetCharacterSpacing)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Foreground(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Foreground)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetForeground<'a, Param0: ::windows::core::IntoParam<'a, super::Media::Brush>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetForeground)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Language)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetLanguage)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsTextScaleFactorEnabled)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsTextScaleFactorEnabled)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn TextDecorations(&self) -> ::windows::core::Result<::windows::UI::Text::TextDecorations> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::TextDecorations = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TextDecorations)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::TextDecorations>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetTextDecorations)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ContentStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContentStart)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ContentEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContentEnd)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ElementStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ElementStart)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ElementEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ElementEnd)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn AllowFocusOnInteraction(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AllowFocusOnInteraction)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetAllowFocusOnInteraction)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn AccessKey(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKey)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetAccessKey<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetAccessKey)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExitDisplayModeOnAccessKeyInvoked)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetExitDisplayModeOnAccessKeyInvoked)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn IsAccessKeyScope(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsAccessKeyScope)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsAccessKeyScope)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn AccessKeyScopeOwner(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyScopeOwner)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetAccessKeyScopeOwner<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetAccessKeyScopeOwner)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::core::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::Input::KeyTipPlacementMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyTipPlacementMode)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetKeyTipPlacementMode)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyTipHorizontalOffset)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetKeyTipHorizontalOffset)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn KeyTipVerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyTipVerticalOffset)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetKeyTipVerticalOffset)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn XamlRoot(&self) -> ::windows::core::Result<super::XamlRoot> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XamlRoot)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::XamlRoot>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetXamlRoot<'a, Param0: ::windows::core::IntoParam<'a, super::XamlRoot>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetXamlRoot)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayRequested<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyDisplayRequested)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveAccessKeyDisplayRequested<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveAccessKeyDisplayRequested)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayDismissedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyDisplayDismissed)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveAccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveAccessKeyDisplayDismissed)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyInvoked<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyInvokedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyInvoked)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveAccessKeyInvoked<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveAccessKeyInvoked)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FindName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindName)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHyperlinkStatics<R, F: FnOnce(&IHyperlinkStatics) -> ::windows::core::Result<R>>(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Hyperlink, IHyperlinkStatics> =
            ::windows::core::FactoryCache::from_library(b"Microsoft.UI.Xaml.dll\0");
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Hyperlink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Hyperlink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Hyperlink {}
impl ::core::fmt::Debug for Hyperlink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Hyperlink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Hyperlink {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.Hyperlink;{ac09bd16-cdfa-54c2-8d03-a474181545b1})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for Hyperlink {
    type Vtable = IHyperlink_Vtbl;
    const IID: ::windows::core::GUID = <IHyperlink as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Hyperlink {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Hyperlink";
}
impl ::core::convert::From<Hyperlink> for ::windows::core::IUnknown {
    fn from(value: Hyperlink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Hyperlink> for ::windows::core::IUnknown {
    fn from(value: &Hyperlink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Hyperlink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Hyperlink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Hyperlink> for ::windows::core::IInspectable {
    fn from(value: Hyperlink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Hyperlink> for ::windows::core::IInspectable {
    fn from(value: &Hyperlink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Hyperlink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Hyperlink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Hyperlink> for Span {
    fn from(value: Hyperlink) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Hyperlink> for Span {
    fn from(value: &Hyperlink) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Span> for Hyperlink {
    fn into_param(self) -> ::windows::core::Param<'a, Span> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Span> for &Hyperlink {
    fn into_param(self) -> ::windows::core::Param<'a, Span> {
        ::windows::core::Param::Owned(::core::convert::Into::<Span>::into(self))
    }
}
impl ::core::convert::From<Hyperlink> for Inline {
    fn from(value: Hyperlink) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Hyperlink> for Inline {
    fn from(value: &Hyperlink) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Inline> for Hyperlink {
    fn into_param(self) -> ::windows::core::Param<'a, Inline> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Inline> for &Hyperlink {
    fn into_param(self) -> ::windows::core::Param<'a, Inline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Inline>::into(self))
    }
}
impl ::core::convert::From<Hyperlink> for TextElement {
    fn from(value: Hyperlink) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Hyperlink> for TextElement {
    fn from(value: &Hyperlink) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for Hyperlink {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for &Hyperlink {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<TextElement>::into(self))
    }
}
impl ::core::convert::From<Hyperlink> for super::DependencyObject {
    fn from(value: Hyperlink) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Hyperlink> for super::DependencyObject {
    fn from(value: &Hyperlink) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for Hyperlink {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &Hyperlink {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for Hyperlink {}
unsafe impl ::core::marker::Sync for Hyperlink {}
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct HyperlinkClickEventArgs(::windows::core::IUnknown);
impl HyperlinkClickEventArgs {
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn OriginalSource(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OriginalSource)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
impl ::core::clone::Clone for HyperlinkClickEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HyperlinkClickEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HyperlinkClickEventArgs {}
impl ::core::fmt::Debug for HyperlinkClickEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HyperlinkClickEventArgs")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HyperlinkClickEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Documents.HyperlinkClickEventArgs;{f8f89552-873d-5ef5-82bf-c79a9509b07c})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for HyperlinkClickEventArgs {
    type Vtable = IHyperlinkClickEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        <IHyperlinkClickEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HyperlinkClickEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.HyperlinkClickEventArgs";
}
impl ::core::convert::From<HyperlinkClickEventArgs> for ::windows::core::IUnknown {
    fn from(value: HyperlinkClickEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HyperlinkClickEventArgs> for ::windows::core::IUnknown {
    fn from(value: &HyperlinkClickEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HyperlinkClickEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a HyperlinkClickEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HyperlinkClickEventArgs> for ::windows::core::IInspectable {
    fn from(value: HyperlinkClickEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HyperlinkClickEventArgs> for ::windows::core::IInspectable {
    fn from(value: &HyperlinkClickEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HyperlinkClickEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a HyperlinkClickEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HyperlinkClickEventArgs> for super::RoutedEventArgs {
    fn from(value: HyperlinkClickEventArgs) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&HyperlinkClickEventArgs> for super::RoutedEventArgs {
    fn from(value: &HyperlinkClickEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for HyperlinkClickEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for &HyperlinkClickEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
unsafe impl ::core::marker::Send for HyperlinkClickEventArgs {}
unsafe impl ::core::marker::Sync for HyperlinkClickEventArgs {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBlock(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBlock {
    type Vtable = IBlock_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x8149d507_672f_5fd5_a10a_351389ba9659);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBlock_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub TextAlignment: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::TextAlignment,
    ) -> ::windows::core::HRESULT,
    pub SetTextAlignment: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::TextAlignment,
    ) -> ::windows::core::HRESULT,
    pub HorizontalTextAlignment: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::TextAlignment,
    ) -> ::windows::core::HRESULT,
    pub SetHorizontalTextAlignment: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::TextAlignment,
    ) -> ::windows::core::HRESULT,
    pub LineHeight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows::core::HRESULT,
    pub SetLineHeight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows::core::HRESULT,
    pub LineStackingStrategy: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::LineStackingStrategy,
    ) -> ::windows::core::HRESULT,
    pub SetLineStackingStrategy: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::LineStackingStrategy,
    ) -> ::windows::core::HRESULT,
    pub Margin: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::Thickness,
    ) -> ::windows::core::HRESULT,
    pub SetMargin: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::Thickness,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBlockFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBlockFactory {
    type Vtable = IBlockFactory_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x21bd671c_33e2_56ef_be37_a128e898452c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBlockFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBlockStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBlockStatics {
    type Vtable = IBlockStatics_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x830feedf_9aa6_56cd_983e_055500171b45);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBlockStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub TextAlignmentProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub HorizontalTextAlignmentProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub LineHeightProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub LineStackingStrategyProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub MarginProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBold(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBold {
    type Vtable = IBold_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x241a5f5a_c164_597f_b0db_fac7431297f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBold_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGlyphs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGlyphs {
    type Vtable = IGlyphs_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x0fbf8cfe_18e7_5e45_9fa3_d2d0927958f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlyphs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub UnicodeString: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub SetUnicodeString: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub Indices: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub SetIndices: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub FontUri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub SetFontUri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub StyleSimulations: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::Media::StyleSimulations,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    StyleSimulations: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetStyleSimulations: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::Media::StyleSimulations,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetStyleSimulations: usize,
    pub FontRenderingEmSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows::core::HRESULT,
    pub SetFontRenderingEmSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows::core::HRESULT,
    pub OriginX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows::core::HRESULT,
    pub SetOriginX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows::core::HRESULT,
    pub OriginY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows::core::HRESULT,
    pub SetOriginY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Fill: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Fill: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetFill: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetFill: usize,
    pub IsColorFontEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetIsColorFontEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub ColorFontPaletteIndex: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub SetColorFontPaletteIndex: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGlyphsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGlyphsStatics {
    type Vtable = IGlyphsStatics_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x8d9e241a_3e0e_5100_8ede_e008034ff8ae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlyphsStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub UnicodeStringProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub IndicesProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub FontUriProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub StyleSimulationsProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub FontRenderingEmSizeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub OriginXProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub OriginYProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub FillProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub IsColorFontEnabledProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub ColorFontPaletteIndexProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHyperlink(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHyperlink {
    type Vtable = IHyperlink_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xac09bd16_cdfa_54c2_8d03_a474181545b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHyperlink_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub NavigateUri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub SetNavigateUri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub UnderlineStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut UnderlineStyle,
    ) -> ::windows::core::HRESULT,
    pub SetUnderlineStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: UnderlineStyle,
    ) -> ::windows::core::HRESULT,
    pub XYFocusLeft: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub SetXYFocusLeft: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub XYFocusRight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub SetXYFocusRight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub XYFocusUp: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub SetXYFocusUp: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub XYFocusDown: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub SetXYFocusDown: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub ElementSoundMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::ElementSoundMode,
    ) -> ::windows::core::HRESULT,
    pub SetElementSoundMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::ElementSoundMode,
    ) -> ::windows::core::HRESULT,
    pub FocusState: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::FocusState,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub XYFocusUpNavigationStrategy: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    XYFocusUpNavigationStrategy: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub SetXYFocusUpNavigationStrategy: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    SetXYFocusUpNavigationStrategy: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub XYFocusDownNavigationStrategy: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    XYFocusDownNavigationStrategy: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub SetXYFocusDownNavigationStrategy: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::Input::XYFocusNavigationStrategy,
    )
        -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    SetXYFocusDownNavigationStrategy: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub XYFocusLeftNavigationStrategy: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    XYFocusLeftNavigationStrategy: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub SetXYFocusLeftNavigationStrategy: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::Input::XYFocusNavigationStrategy,
    )
        -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    SetXYFocusLeftNavigationStrategy: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub XYFocusRightNavigationStrategy: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    XYFocusRightNavigationStrategy: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub SetXYFocusRightNavigationStrategy: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::Input::XYFocusNavigationStrategy,
    )
        -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    SetXYFocusRightNavigationStrategy: usize,
    pub IsTabStop: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetIsTabStop: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub TabIndex: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub SetTabIndex: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows::core::HRESULT,
    pub Click: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveClick: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub GotFocus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveGotFocus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub LostFocus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveLostFocus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub Focus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::FocusState,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHyperlinkClickEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHyperlinkClickEventArgs {
    type Vtable = IHyperlinkClickEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xf8f89552_873d_5ef5_82bf_c79a9509b07c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHyperlinkClickEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHyperlinkStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHyperlinkStatics {
    type Vtable = IHyperlinkStatics_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xe13598f4_7bc7_5ab9_885b_70f32f8c9531);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHyperlinkStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub NavigateUriProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub UnderlineStyleProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub XYFocusLeftProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub XYFocusRightProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub XYFocusUpProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub XYFocusDownProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub ElementSoundModeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub FocusStateProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub XYFocusUpNavigationStrategyProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    )
        -> ::windows::core::HRESULT,
    pub XYFocusDownNavigationStrategyProperty:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::core::RawPtr,
        ) -> ::windows::core::HRESULT,
    pub XYFocusLeftNavigationStrategyProperty:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::core::RawPtr,
        ) -> ::windows::core::HRESULT,
    pub XYFocusRightNavigationStrategyProperty:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::core::RawPtr,
        ) -> ::windows::core::HRESULT,
    pub IsTabStopProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub TabIndexProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInline(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInline {
    type Vtable = IInline_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x813d427a_8980_5a79_a8fa_f27919cfb24f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInline_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInlineFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInlineFactory {
    type Vtable = IInlineFactory_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xfd253a36_fa2b_5b30_89a8_9f577871ec07);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInlineFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInlineUIContainer(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInlineUIContainer {
    type Vtable = IInlineUIContainer_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xd529bef6_c05a_5bad_85e8_640127cf86f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInlineUIContainer_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Child: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub SetChild: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IItalic(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IItalic {
    type Vtable = IItalic_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xca3cbebd_7a8d_5d7a_8fdf_538e8a680f6c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IItalic_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILineBreak(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILineBreak {
    type Vtable = ILineBreak_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x09307599_7cc2_5f54_b106_728620c16f76);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineBreak_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IParagraph(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IParagraph {
    type Vtable = IParagraph_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x9ed64c77_329d_502f_a257_f58398edab51);
}
#[repr(C)]
#[doc(hidden)]
pub struct IParagraph_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Inlines: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub TextIndent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows::core::HRESULT,
    pub SetTextIndent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IParagraphStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IParagraphStatics {
    type Vtable = IParagraphStatics_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x4eb89ab1_66c8_5fc0_aa5f_48c8092ceb5f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IParagraphStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub TextIndentProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRun(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRun {
    type Vtable = IRun_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x1f905239_37cb_590b_9132_3ffb7741906e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRun_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Text: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub SetText: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub FlowDirection: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::FlowDirection,
    ) -> ::windows::core::HRESULT,
    pub SetFlowDirection: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::FlowDirection,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRunStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRunStatics {
    type Vtable = IRunStatics_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x051b3c5b_7600_51a5_80c5_93eb50fd684f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRunStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub FlowDirectionProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpan(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpan {
    type Vtable = ISpan_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x91b93d4d_4e28_57b9_bffb_3566c2a3c2a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpan_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Inlines: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub SetInlines: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpanFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpanFactory {
    type Vtable = ISpanFactory_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xa6e87c16_c175_55c8_bbd3_ce40f9d0a680);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpanFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextElement(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextElement {
    type Vtable = ITextElement_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xa122ba22_833f_5220_a47e_6cd507531abe);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextElement_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub FontSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows::core::HRESULT,
    pub SetFontSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub FontFamily: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    FontFamily: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetFontFamily: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetFontFamily: usize,
    pub FontWeight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Text::FontWeight,
    ) -> ::windows::core::HRESULT,
    pub SetFontWeight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::UI::Text::FontWeight,
    ) -> ::windows::core::HRESULT,
    pub FontStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Text::FontStyle,
    ) -> ::windows::core::HRESULT,
    pub SetFontStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows::core::HRESULT,
    pub FontStretch: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Text::FontStretch,
    ) -> ::windows::core::HRESULT,
    pub SetFontStretch: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows::core::HRESULT,
    pub CharacterSpacing: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub SetCharacterSpacing: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Foreground: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Foreground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetForeground: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetForeground: usize,
    pub Language: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub IsTextScaleFactorEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetIsTextScaleFactorEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub TextDecorations: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Text::TextDecorations,
    ) -> ::windows::core::HRESULT,
    pub SetTextDecorations: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows::core::HRESULT,
    pub ContentStart: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub ContentEnd: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub ElementStart: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub ElementEnd: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub AllowFocusOnInteraction: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetAllowFocusOnInteraction: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub AccessKey: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub SetAccessKey: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub ExitDisplayModeOnAccessKeyInvoked: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    )
        -> ::windows::core::HRESULT,
    pub SetExitDisplayModeOnAccessKeyInvoked: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    )
        -> ::windows::core::HRESULT,
    pub IsAccessKeyScope: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetIsAccessKeyScope: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub AccessKeyScopeOwner: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub SetAccessKeyScopeOwner: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub KeyTipPlacementMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::Input::KeyTipPlacementMode,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    KeyTipPlacementMode: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub SetKeyTipPlacementMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    SetKeyTipPlacementMode: usize,
    pub KeyTipHorizontalOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows::core::HRESULT,
    pub SetKeyTipHorizontalOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows::core::HRESULT,
    pub KeyTipVerticalOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows::core::HRESULT,
    pub SetKeyTipVerticalOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows::core::HRESULT,
    pub XamlRoot: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub SetXamlRoot: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub AccessKeyDisplayRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    AccessKeyDisplayRequested: usize,
    pub RemoveAccessKeyDisplayRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub AccessKeyDisplayDismissed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    AccessKeyDisplayDismissed: usize,
    pub RemoveAccessKeyDisplayDismissed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub AccessKeyInvoked: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    AccessKeyInvoked: usize,
    pub RemoveAccessKeyInvoked: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub FindName: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextElementFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextElementFactory {
    type Vtable = ITextElementFactory_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xdf51fb95_a5e6_5b16_8e88_9f7cbfa234b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextElementFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextElementOverrides(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextElementOverrides {
    type Vtable = ITextElementOverrides_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x41b01380_e49f_5fda_8c72_acc1ac1e91df);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextElementOverrides_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub OnDisconnectVisualChildren:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextElementStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextElementStatics {
    type Vtable = ITextElementStatics_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xc9b55919_e1fe_5acd_bac7_c9d7f413b35c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextElementStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub FontSizeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub FontFamilyProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub FontWeightProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub FontStyleProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub FontStretchProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub CharacterSpacingProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub ForegroundProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub LanguageProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub IsTextScaleFactorEnabledProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    )
        -> ::windows::core::HRESULT,
    pub TextDecorationsProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub AllowFocusOnInteractionProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub AccessKeyProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub ExitDisplayModeOnAccessKeyInvokedProperty:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::core::RawPtr,
        ) -> ::windows::core::HRESULT,
    pub IsAccessKeyScopeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub AccessKeyScopeOwnerProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub KeyTipPlacementModeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub KeyTipHorizontalOffsetProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub KeyTipVerticalOffsetProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextHighlighter(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextHighlighter {
    type Vtable = ITextHighlighter_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xb756e861_1d2b_5f6f_81fd_c51a5bc068ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextHighlighter_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Ranges: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Foreground: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Foreground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetForeground: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetForeground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Background: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Background: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetBackground: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetBackground: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextHighlighterBase(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextHighlighterBase {
    type Vtable = ITextHighlighterBase_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x5c21aaf0_3a17_5468_8aac_be14db0ed8c1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextHighlighterBase_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextHighlighterBaseFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextHighlighterBaseFactory {
    type Vtable = ITextHighlighterBaseFactory_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xe091e461_53ab_599e_aaea_800adc72da4f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextHighlighterBaseFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextHighlighterFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextHighlighterFactory {
    type Vtable = ITextHighlighterFactory_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x69c7311f_c019_5b93_b511_81418543bab7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextHighlighterFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextHighlighterStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextHighlighterStatics {
    type Vtable = ITextHighlighterStatics_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x4975047a_87ad_51a2_977c_e771de4f4035);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextHighlighterStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub ForegroundProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub BackgroundProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextPointer(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextPointer {
    type Vtable = ITextPointer_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x842eb385_ee41_5930_979b_438fa7525a51);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextPointer_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Parent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub VisualParent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub LogicalDirection: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut LogicalDirection,
    ) -> ::windows::core::HRESULT,
    pub Offset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub GetCharacterRect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        direction: LogicalDirection,
        result__: *mut ::windows::Foundation::Rect,
    ) -> ::windows::core::HRESULT,
    pub GetPositionAtOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        offset: i32,
        direction: LogicalDirection,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITypography(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITypography {
    type Vtable = ITypography_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xfa27e2e3_be5e_5d21_9a5e_90cf102af828);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypography_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITypographyStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITypographyStatics {
    type Vtable = ITypographyStatics_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x55fe4535_2125_533a_ada8_27be2b9e1193);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypographyStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub AnnotationAlternatesProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetAnnotationAlternates: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub SetAnnotationAlternates: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        value: i32,
    ) -> ::windows::core::HRESULT,
    pub EastAsianExpertFormsProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetEastAsianExpertForms: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetEastAsianExpertForms: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub EastAsianLanguageProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetEastAsianLanguage: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        result__: *mut super::FontEastAsianLanguage,
    ) -> ::windows::core::HRESULT,
    pub SetEastAsianLanguage: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        value: super::FontEastAsianLanguage,
    ) -> ::windows::core::HRESULT,
    pub EastAsianWidthsProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetEastAsianWidths: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        result__: *mut super::FontEastAsianWidths,
    ) -> ::windows::core::HRESULT,
    pub SetEastAsianWidths: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        value: super::FontEastAsianWidths,
    ) -> ::windows::core::HRESULT,
    pub StandardLigaturesProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetStandardLigatures: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetStandardLigatures: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub ContextualLigaturesProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetContextualLigatures: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetContextualLigatures: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub DiscretionaryLigaturesProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetDiscretionaryLigatures: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetDiscretionaryLigatures: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub HistoricalLigaturesProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetHistoricalLigatures: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetHistoricalLigatures: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub StandardSwashesProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetStandardSwashes: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub SetStandardSwashes: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        value: i32,
    ) -> ::windows::core::HRESULT,
    pub ContextualSwashesProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetContextualSwashes: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub SetContextualSwashes: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        value: i32,
    ) -> ::windows::core::HRESULT,
    pub ContextualAlternatesProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetContextualAlternates: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetContextualAlternates: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub StylisticAlternatesProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetStylisticAlternates: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub SetStylisticAlternates: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        value: i32,
    ) -> ::windows::core::HRESULT,
    pub StylisticSet1Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetStylisticSet1: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetStylisticSet1: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub StylisticSet2Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetStylisticSet2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetStylisticSet2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub StylisticSet3Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetStylisticSet3: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetStylisticSet3: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub StylisticSet4Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetStylisticSet4: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetStylisticSet4: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub StylisticSet5Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetStylisticSet5: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetStylisticSet5: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub StylisticSet6Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetStylisticSet6: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetStylisticSet6: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub StylisticSet7Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetStylisticSet7: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetStylisticSet7: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub StylisticSet8Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetStylisticSet8: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetStylisticSet8: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub StylisticSet9Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetStylisticSet9: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetStylisticSet9: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub StylisticSet10Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetStylisticSet10: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetStylisticSet10: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub StylisticSet11Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetStylisticSet11: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetStylisticSet11: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub StylisticSet12Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetStylisticSet12: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetStylisticSet12: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub StylisticSet13Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetStylisticSet13: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetStylisticSet13: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub StylisticSet14Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetStylisticSet14: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetStylisticSet14: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub StylisticSet15Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetStylisticSet15: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetStylisticSet15: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub StylisticSet16Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetStylisticSet16: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetStylisticSet16: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub StylisticSet17Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetStylisticSet17: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetStylisticSet17: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub StylisticSet18Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetStylisticSet18: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetStylisticSet18: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub StylisticSet19Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetStylisticSet19: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetStylisticSet19: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub StylisticSet20Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetStylisticSet20: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetStylisticSet20: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub CapitalsProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetCapitals: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        result__: *mut super::FontCapitals,
    ) -> ::windows::core::HRESULT,
    pub SetCapitals: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        value: super::FontCapitals,
    ) -> ::windows::core::HRESULT,
    pub CapitalSpacingProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetCapitalSpacing: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetCapitalSpacing: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub KerningProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetKerning: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetKerning: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub CaseSensitiveFormsProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetCaseSensitiveForms: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetCaseSensitiveForms: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub HistoricalFormsProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetHistoricalForms: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetHistoricalForms: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub FractionProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetFraction: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        result__: *mut super::FontFraction,
    ) -> ::windows::core::HRESULT,
    pub SetFraction: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        value: super::FontFraction,
    ) -> ::windows::core::HRESULT,
    pub NumeralStyleProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetNumeralStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        result__: *mut super::FontNumeralStyle,
    ) -> ::windows::core::HRESULT,
    pub SetNumeralStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        value: super::FontNumeralStyle,
    ) -> ::windows::core::HRESULT,
    pub NumeralAlignmentProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetNumeralAlignment: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        result__: *mut super::FontNumeralAlignment,
    ) -> ::windows::core::HRESULT,
    pub SetNumeralAlignment: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        value: super::FontNumeralAlignment,
    ) -> ::windows::core::HRESULT,
    pub SlashedZeroProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetSlashedZero: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetSlashedZero: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub MathematicalGreekProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetMathematicalGreek: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetMathematicalGreek: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub VariantsProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetVariants: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        result__: *mut super::FontVariants,
    ) -> ::windows::core::HRESULT,
    pub SetVariants: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: ::windows::core::RawPtr,
        value: super::FontVariants,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUnderline(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUnderline {
    type Vtable = IUnderline_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x68aaec6e_ea71_5ed2_b83e_91684b25efb9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUnderline_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
}
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct Inline(::windows::core::IUnknown);
impl Inline {
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ClearValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).ClearValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ReadLocalValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReadLocalValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetAnimationBaseValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAnimationBaseValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RegisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::core::Result<i64> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RegisterPropertyChangedCallback)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                callback.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn UnregisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).UnregisterPropertyChangedCallback)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Dispatcher(&self) -> ::windows::core::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Dispatcher)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Dispatching\"`*"]
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DispatcherQueue)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontSize(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontSize)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFontSize(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontSize)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows::core::Result<super::Media::FontFamily> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontFamily)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFontFamily<'a, Param0: ::windows::core::IntoParam<'a, super::Media::FontFamily>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontFamily)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontWeight(&self) -> ::windows::core::Result<::windows::UI::Text::FontWeight> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontWeight = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontWeight)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFontWeight<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::UI::Text::FontWeight>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontWeight)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontStyle(&self) -> ::windows::core::Result<::windows::UI::Text::FontStyle> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStyle = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontStyle)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStyle>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontStyle)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontStretch(&self) -> ::windows::core::Result<::windows::UI::Text::FontStretch> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStretch = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontStretch)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStretch>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontStretch)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn CharacterSpacing(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CharacterSpacing)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetCharacterSpacing)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Foreground(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Foreground)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetForeground<'a, Param0: ::windows::core::IntoParam<'a, super::Media::Brush>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetForeground)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Language)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetLanguage)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsTextScaleFactorEnabled)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsTextScaleFactorEnabled)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn TextDecorations(&self) -> ::windows::core::Result<::windows::UI::Text::TextDecorations> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::TextDecorations = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TextDecorations)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::TextDecorations>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetTextDecorations)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ContentStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContentStart)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ContentEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContentEnd)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ElementStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ElementStart)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ElementEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ElementEnd)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn AllowFocusOnInteraction(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AllowFocusOnInteraction)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetAllowFocusOnInteraction)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn AccessKey(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKey)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetAccessKey<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetAccessKey)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExitDisplayModeOnAccessKeyInvoked)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetExitDisplayModeOnAccessKeyInvoked)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn IsAccessKeyScope(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsAccessKeyScope)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsAccessKeyScope)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn AccessKeyScopeOwner(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyScopeOwner)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetAccessKeyScopeOwner<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetAccessKeyScopeOwner)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::core::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::Input::KeyTipPlacementMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyTipPlacementMode)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetKeyTipPlacementMode)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyTipHorizontalOffset)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetKeyTipHorizontalOffset)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn KeyTipVerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyTipVerticalOffset)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetKeyTipVerticalOffset)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn XamlRoot(&self) -> ::windows::core::Result<super::XamlRoot> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XamlRoot)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::XamlRoot>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetXamlRoot<'a, Param0: ::windows::core::IntoParam<'a, super::XamlRoot>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetXamlRoot)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayRequested<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyDisplayRequested)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveAccessKeyDisplayRequested<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveAccessKeyDisplayRequested)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayDismissedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyDisplayDismissed)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveAccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveAccessKeyDisplayDismissed)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyInvoked<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyInvokedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyInvoked)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveAccessKeyInvoked<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveAccessKeyInvoked)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FindName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindName)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
impl ::core::clone::Clone for Inline {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Inline {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Inline {}
impl ::core::fmt::Debug for Inline {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Inline").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Inline {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.Inline;{813d427a-8980-5a79-a8fa-f27919cfb24f})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for Inline {
    type Vtable = IInline_Vtbl;
    const IID: ::windows::core::GUID = <IInline as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Inline {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Inline";
}
impl ::core::convert::From<Inline> for ::windows::core::IUnknown {
    fn from(value: Inline) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Inline> for ::windows::core::IUnknown {
    fn from(value: &Inline) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Inline {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Inline {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Inline> for ::windows::core::IInspectable {
    fn from(value: Inline) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Inline> for ::windows::core::IInspectable {
    fn from(value: &Inline) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Inline {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Inline {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Inline> for TextElement {
    fn from(value: Inline) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Inline> for TextElement {
    fn from(value: &Inline) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for Inline {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for &Inline {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<TextElement>::into(self))
    }
}
impl ::core::convert::From<Inline> for super::DependencyObject {
    fn from(value: Inline) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Inline> for super::DependencyObject {
    fn from(value: &Inline) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for Inline {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &Inline {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for Inline {}
unsafe impl ::core::marker::Sync for Inline {}
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct InlineCollection(::windows::core::IUnknown);
impl InlineCollection {
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn First(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Collections::IIterator<Inline>> {
        let this = &::windows::core::Interface::cast::<
            ::windows::Foundation::Collections::IIterable<Inline>,
        >(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).First)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IIterator<Inline>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<Inline> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAt)(
                ::core::mem::transmute_copy(this),
                index,
                &mut result__,
            )
            .from_abi::<Inline>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Size)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetView(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Collections::IVectorView<Inline>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetView)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IVectorView<Inline>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, Inline>>(
        &self,
        value: Param0,
        index: &mut u32,
    ) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IndexOf)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
                index,
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, Inline>>(
        &self,
        index: u32,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetAt)(
                ::core::mem::transmute_copy(this),
                index,
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, Inline>>(
        &self,
        index: u32,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).InsertAt)(
                ::core::mem::transmute_copy(this),
                index,
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveAt)(
                ::core::mem::transmute_copy(this),
                index,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Append<'a, Param0: ::windows::core::IntoParam<'a, Inline>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).Append)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveAtEnd)(::core::mem::transmute_copy(
                this,
            ))
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).Clear)(::core::mem::transmute_copy(this)).ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetMany(
        &self,
        startindex: u32,
        items: &mut [::core::option::Option<Inline>],
    ) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetMany)(
                ::core::mem::transmute_copy(this),
                startindex,
                items.len() as u32,
                ::core::mem::transmute_copy(&items),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ReplaceAll(
        &self,
        items: &[::core::option::Option<Inline>],
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).ReplaceAll)(
                ::core::mem::transmute_copy(this),
                items.len() as u32,
                ::core::mem::transmute(items.as_ptr()),
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for InlineCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InlineCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InlineCollection {}
impl ::core::fmt::Debug for InlineCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InlineCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InlineCollection {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Documents.InlineCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Microsoft.UI.Xaml.Documents.Inline;{813d427a-8980-5a79-a8fa-f27919cfb24f})))" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for InlineCollection {
    type Vtable = ::windows::Foundation::Collections::IVector_Vtbl<Inline>;
    const IID: ::windows::core::GUID =
        <::windows::Foundation::Collections::IVector<Inline> as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InlineCollection {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.InlineCollection";
}
impl ::core::iter::IntoIterator for InlineCollection {
    type Item = Inline;
    type IntoIter = ::windows::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
impl ::core::iter::IntoIterator for &InlineCollection {
    type Item = Inline;
    type IntoIter = ::windows::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::windows::Foundation::Collections::VectorIterator::new(
            ::core::convert::TryInto::try_into(self).ok(),
        )
    }
}
impl ::core::convert::From<InlineCollection> for ::windows::core::IUnknown {
    fn from(value: InlineCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InlineCollection> for ::windows::core::IUnknown {
    fn from(value: &InlineCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InlineCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InlineCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InlineCollection> for ::windows::core::IInspectable {
    fn from(value: InlineCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InlineCollection> for ::windows::core::IInspectable {
    fn from(value: &InlineCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InlineCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InlineCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<InlineCollection>
    for ::windows::Foundation::Collections::IIterable<Inline>
{
    type Error = ::windows::core::Error;
    fn try_from(value: InlineCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InlineCollection>
    for ::windows::Foundation::Collections::IIterable<Inline>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &InlineCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::Collections::IIterable<Inline>>
    for InlineCollection
{
    fn into_param(
        self,
    ) -> ::windows::core::Param<'a, ::windows::Foundation::Collections::IIterable<Inline>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::Collections::IIterable<Inline>>
    for &InlineCollection
{
    fn into_param(
        self,
    ) -> ::windows::core::Param<'a, ::windows::Foundation::Collections::IIterable<Inline>> {
        ::core::convert::TryInto::<::windows::Foundation::Collections::IIterable<Inline>>::try_into(
            self,
        )
        .map(::windows::core::Param::Owned)
        .unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<InlineCollection>
    for ::windows::Foundation::Collections::IVector<Inline>
{
    type Error = ::windows::core::Error;
    fn try_from(value: InlineCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InlineCollection>
    for ::windows::Foundation::Collections::IVector<Inline>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &InlineCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::Collections::IVector<Inline>>
    for InlineCollection
{
    fn into_param(
        self,
    ) -> ::windows::core::Param<'a, ::windows::Foundation::Collections::IVector<Inline>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::Collections::IVector<Inline>>
    for &InlineCollection
{
    fn into_param(
        self,
    ) -> ::windows::core::Param<'a, ::windows::Foundation::Collections::IVector<Inline>> {
        ::core::convert::TryInto::<::windows::Foundation::Collections::IVector<Inline>>::try_into(
            self,
        )
        .map(::windows::core::Param::Owned)
        .unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for InlineCollection {}
unsafe impl ::core::marker::Sync for InlineCollection {}
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct InlineUIContainer(::windows::core::IUnknown);
impl InlineUIContainer {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            InlineUIContainer,
            ::windows::core::IActivationFactory,
        > = ::windows::core::FactoryCache::from_library(b"Microsoft.UI.Xaml.dll\0");
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ClearValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).ClearValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ReadLocalValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReadLocalValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetAnimationBaseValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAnimationBaseValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RegisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::core::Result<i64> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RegisterPropertyChangedCallback)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                callback.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn UnregisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).UnregisterPropertyChangedCallback)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Dispatcher(&self) -> ::windows::core::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Dispatcher)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Dispatching\"`*"]
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DispatcherQueue)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Child(&self) -> ::windows::core::Result<super::UIElement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Child)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::UIElement>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetChild<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetChild)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontSize(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontSize)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFontSize(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontSize)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows::core::Result<super::Media::FontFamily> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontFamily)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFontFamily<'a, Param0: ::windows::core::IntoParam<'a, super::Media::FontFamily>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontFamily)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontWeight(&self) -> ::windows::core::Result<::windows::UI::Text::FontWeight> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontWeight = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontWeight)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFontWeight<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::UI::Text::FontWeight>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontWeight)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontStyle(&self) -> ::windows::core::Result<::windows::UI::Text::FontStyle> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStyle = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontStyle)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStyle>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontStyle)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontStretch(&self) -> ::windows::core::Result<::windows::UI::Text::FontStretch> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStretch = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontStretch)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStretch>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontStretch)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn CharacterSpacing(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CharacterSpacing)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetCharacterSpacing)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Foreground(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Foreground)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetForeground<'a, Param0: ::windows::core::IntoParam<'a, super::Media::Brush>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetForeground)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Language)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetLanguage)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsTextScaleFactorEnabled)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsTextScaleFactorEnabled)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn TextDecorations(&self) -> ::windows::core::Result<::windows::UI::Text::TextDecorations> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::TextDecorations = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TextDecorations)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::TextDecorations>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetTextDecorations)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ContentStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContentStart)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ContentEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContentEnd)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ElementStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ElementStart)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ElementEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ElementEnd)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn AllowFocusOnInteraction(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AllowFocusOnInteraction)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetAllowFocusOnInteraction)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn AccessKey(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKey)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetAccessKey<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetAccessKey)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExitDisplayModeOnAccessKeyInvoked)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetExitDisplayModeOnAccessKeyInvoked)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn IsAccessKeyScope(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsAccessKeyScope)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsAccessKeyScope)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn AccessKeyScopeOwner(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyScopeOwner)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetAccessKeyScopeOwner<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetAccessKeyScopeOwner)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::core::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::Input::KeyTipPlacementMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyTipPlacementMode)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetKeyTipPlacementMode)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyTipHorizontalOffset)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetKeyTipHorizontalOffset)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn KeyTipVerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyTipVerticalOffset)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetKeyTipVerticalOffset)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn XamlRoot(&self) -> ::windows::core::Result<super::XamlRoot> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XamlRoot)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::XamlRoot>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetXamlRoot<'a, Param0: ::windows::core::IntoParam<'a, super::XamlRoot>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetXamlRoot)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayRequested<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyDisplayRequested)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveAccessKeyDisplayRequested<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveAccessKeyDisplayRequested)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayDismissedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyDisplayDismissed)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveAccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveAccessKeyDisplayDismissed)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyInvoked<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyInvokedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyInvoked)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveAccessKeyInvoked<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveAccessKeyInvoked)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FindName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindName)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
impl ::core::clone::Clone for InlineUIContainer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InlineUIContainer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InlineUIContainer {}
impl ::core::fmt::Debug for InlineUIContainer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InlineUIContainer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InlineUIContainer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.InlineUIContainer;{d529bef6-c05a-5bad-85e8-640127cf86f5})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for InlineUIContainer {
    type Vtable = IInlineUIContainer_Vtbl;
    const IID: ::windows::core::GUID = <IInlineUIContainer as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InlineUIContainer {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.InlineUIContainer";
}
impl ::core::convert::From<InlineUIContainer> for ::windows::core::IUnknown {
    fn from(value: InlineUIContainer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InlineUIContainer> for ::windows::core::IUnknown {
    fn from(value: &InlineUIContainer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InlineUIContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InlineUIContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InlineUIContainer> for ::windows::core::IInspectable {
    fn from(value: InlineUIContainer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InlineUIContainer> for ::windows::core::IInspectable {
    fn from(value: &InlineUIContainer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InlineUIContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InlineUIContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InlineUIContainer> for Inline {
    fn from(value: InlineUIContainer) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&InlineUIContainer> for Inline {
    fn from(value: &InlineUIContainer) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Inline> for InlineUIContainer {
    fn into_param(self) -> ::windows::core::Param<'a, Inline> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Inline> for &InlineUIContainer {
    fn into_param(self) -> ::windows::core::Param<'a, Inline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Inline>::into(self))
    }
}
impl ::core::convert::From<InlineUIContainer> for TextElement {
    fn from(value: InlineUIContainer) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&InlineUIContainer> for TextElement {
    fn from(value: &InlineUIContainer) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for InlineUIContainer {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for &InlineUIContainer {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<TextElement>::into(self))
    }
}
impl ::core::convert::From<InlineUIContainer> for super::DependencyObject {
    fn from(value: InlineUIContainer) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&InlineUIContainer> for super::DependencyObject {
    fn from(value: &InlineUIContainer) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for InlineUIContainer {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &InlineUIContainer {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for InlineUIContainer {}
unsafe impl ::core::marker::Sync for InlineUIContainer {}
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct Italic(::windows::core::IUnknown);
impl Italic {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            Italic,
            ::windows::core::IActivationFactory,
        > = ::windows::core::FactoryCache::from_library(b"Microsoft.UI.Xaml.dll\0");
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ClearValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).ClearValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ReadLocalValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReadLocalValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetAnimationBaseValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAnimationBaseValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RegisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::core::Result<i64> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RegisterPropertyChangedCallback)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                callback.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn UnregisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).UnregisterPropertyChangedCallback)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Dispatcher(&self) -> ::windows::core::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Dispatcher)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Dispatching\"`*"]
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DispatcherQueue)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Inlines(&self) -> ::windows::core::Result<InlineCollection> {
        let this = &::windows::core::Interface::cast::<ISpan>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Inlines)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InlineCollection>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetInlines<'a, Param0: ::windows::core::IntoParam<'a, InlineCollection>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISpan>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetInlines)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontSize(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontSize)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFontSize(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontSize)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows::core::Result<super::Media::FontFamily> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontFamily)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFontFamily<'a, Param0: ::windows::core::IntoParam<'a, super::Media::FontFamily>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontFamily)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontWeight(&self) -> ::windows::core::Result<::windows::UI::Text::FontWeight> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontWeight = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontWeight)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFontWeight<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::UI::Text::FontWeight>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontWeight)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontStyle(&self) -> ::windows::core::Result<::windows::UI::Text::FontStyle> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStyle = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontStyle)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStyle>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontStyle)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontStretch(&self) -> ::windows::core::Result<::windows::UI::Text::FontStretch> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStretch = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontStretch)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStretch>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontStretch)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn CharacterSpacing(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CharacterSpacing)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetCharacterSpacing)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Foreground(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Foreground)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetForeground<'a, Param0: ::windows::core::IntoParam<'a, super::Media::Brush>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetForeground)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Language)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetLanguage)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsTextScaleFactorEnabled)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsTextScaleFactorEnabled)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn TextDecorations(&self) -> ::windows::core::Result<::windows::UI::Text::TextDecorations> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::TextDecorations = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TextDecorations)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::TextDecorations>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetTextDecorations)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ContentStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContentStart)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ContentEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContentEnd)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ElementStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ElementStart)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ElementEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ElementEnd)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn AllowFocusOnInteraction(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AllowFocusOnInteraction)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetAllowFocusOnInteraction)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn AccessKey(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKey)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetAccessKey<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetAccessKey)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExitDisplayModeOnAccessKeyInvoked)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetExitDisplayModeOnAccessKeyInvoked)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn IsAccessKeyScope(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsAccessKeyScope)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsAccessKeyScope)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn AccessKeyScopeOwner(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyScopeOwner)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetAccessKeyScopeOwner<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetAccessKeyScopeOwner)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::core::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::Input::KeyTipPlacementMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyTipPlacementMode)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetKeyTipPlacementMode)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyTipHorizontalOffset)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetKeyTipHorizontalOffset)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn KeyTipVerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyTipVerticalOffset)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetKeyTipVerticalOffset)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn XamlRoot(&self) -> ::windows::core::Result<super::XamlRoot> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XamlRoot)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::XamlRoot>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetXamlRoot<'a, Param0: ::windows::core::IntoParam<'a, super::XamlRoot>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetXamlRoot)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayRequested<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyDisplayRequested)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveAccessKeyDisplayRequested<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveAccessKeyDisplayRequested)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayDismissedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyDisplayDismissed)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveAccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveAccessKeyDisplayDismissed)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyInvoked<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyInvokedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyInvoked)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveAccessKeyInvoked<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveAccessKeyInvoked)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FindName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindName)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
impl ::core::clone::Clone for Italic {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Italic {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Italic {}
impl ::core::fmt::Debug for Italic {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Italic").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Italic {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.Italic;{ca3cbebd-7a8d-5d7a-8fdf-538e8a680f6c})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for Italic {
    type Vtable = IItalic_Vtbl;
    const IID: ::windows::core::GUID = <IItalic as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Italic {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Italic";
}
impl ::core::convert::From<Italic> for ::windows::core::IUnknown {
    fn from(value: Italic) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Italic> for ::windows::core::IUnknown {
    fn from(value: &Italic) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Italic {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Italic {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Italic> for ::windows::core::IInspectable {
    fn from(value: Italic) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Italic> for ::windows::core::IInspectable {
    fn from(value: &Italic) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Italic {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Italic {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Italic> for Span {
    fn from(value: Italic) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Italic> for Span {
    fn from(value: &Italic) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Span> for Italic {
    fn into_param(self) -> ::windows::core::Param<'a, Span> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Span> for &Italic {
    fn into_param(self) -> ::windows::core::Param<'a, Span> {
        ::windows::core::Param::Owned(::core::convert::Into::<Span>::into(self))
    }
}
impl ::core::convert::From<Italic> for Inline {
    fn from(value: Italic) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Italic> for Inline {
    fn from(value: &Italic) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Inline> for Italic {
    fn into_param(self) -> ::windows::core::Param<'a, Inline> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Inline> for &Italic {
    fn into_param(self) -> ::windows::core::Param<'a, Inline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Inline>::into(self))
    }
}
impl ::core::convert::From<Italic> for TextElement {
    fn from(value: Italic) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Italic> for TextElement {
    fn from(value: &Italic) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for Italic {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for &Italic {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<TextElement>::into(self))
    }
}
impl ::core::convert::From<Italic> for super::DependencyObject {
    fn from(value: Italic) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Italic> for super::DependencyObject {
    fn from(value: &Italic) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for Italic {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &Italic {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for Italic {}
unsafe impl ::core::marker::Sync for Italic {}
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct LineBreak(::windows::core::IUnknown);
impl LineBreak {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            LineBreak,
            ::windows::core::IActivationFactory,
        > = ::windows::core::FactoryCache::from_library(b"Microsoft.UI.Xaml.dll\0");
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ClearValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).ClearValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ReadLocalValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReadLocalValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetAnimationBaseValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAnimationBaseValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RegisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::core::Result<i64> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RegisterPropertyChangedCallback)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                callback.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn UnregisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).UnregisterPropertyChangedCallback)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Dispatcher(&self) -> ::windows::core::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Dispatcher)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Dispatching\"`*"]
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DispatcherQueue)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontSize(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontSize)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFontSize(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontSize)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows::core::Result<super::Media::FontFamily> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontFamily)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFontFamily<'a, Param0: ::windows::core::IntoParam<'a, super::Media::FontFamily>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontFamily)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontWeight(&self) -> ::windows::core::Result<::windows::UI::Text::FontWeight> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontWeight = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontWeight)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFontWeight<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::UI::Text::FontWeight>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontWeight)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontStyle(&self) -> ::windows::core::Result<::windows::UI::Text::FontStyle> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStyle = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontStyle)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStyle>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontStyle)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontStretch(&self) -> ::windows::core::Result<::windows::UI::Text::FontStretch> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStretch = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontStretch)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStretch>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontStretch)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn CharacterSpacing(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CharacterSpacing)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetCharacterSpacing)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Foreground(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Foreground)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetForeground<'a, Param0: ::windows::core::IntoParam<'a, super::Media::Brush>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetForeground)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Language)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetLanguage)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsTextScaleFactorEnabled)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsTextScaleFactorEnabled)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn TextDecorations(&self) -> ::windows::core::Result<::windows::UI::Text::TextDecorations> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::TextDecorations = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TextDecorations)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::TextDecorations>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetTextDecorations)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ContentStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContentStart)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ContentEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContentEnd)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ElementStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ElementStart)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ElementEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ElementEnd)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn AllowFocusOnInteraction(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AllowFocusOnInteraction)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetAllowFocusOnInteraction)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn AccessKey(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKey)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetAccessKey<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetAccessKey)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExitDisplayModeOnAccessKeyInvoked)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetExitDisplayModeOnAccessKeyInvoked)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn IsAccessKeyScope(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsAccessKeyScope)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsAccessKeyScope)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn AccessKeyScopeOwner(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyScopeOwner)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetAccessKeyScopeOwner<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetAccessKeyScopeOwner)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::core::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::Input::KeyTipPlacementMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyTipPlacementMode)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetKeyTipPlacementMode)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyTipHorizontalOffset)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetKeyTipHorizontalOffset)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn KeyTipVerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyTipVerticalOffset)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetKeyTipVerticalOffset)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn XamlRoot(&self) -> ::windows::core::Result<super::XamlRoot> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XamlRoot)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::XamlRoot>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetXamlRoot<'a, Param0: ::windows::core::IntoParam<'a, super::XamlRoot>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetXamlRoot)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayRequested<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyDisplayRequested)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveAccessKeyDisplayRequested<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveAccessKeyDisplayRequested)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayDismissedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyDisplayDismissed)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveAccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveAccessKeyDisplayDismissed)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyInvoked<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyInvokedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyInvoked)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveAccessKeyInvoked<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveAccessKeyInvoked)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FindName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindName)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
impl ::core::clone::Clone for LineBreak {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LineBreak {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LineBreak {}
impl ::core::fmt::Debug for LineBreak {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineBreak").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LineBreak {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.LineBreak;{09307599-7cc2-5f54-b106-728620c16f76})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LineBreak {
    type Vtable = ILineBreak_Vtbl;
    const IID: ::windows::core::GUID = <ILineBreak as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LineBreak {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.LineBreak";
}
impl ::core::convert::From<LineBreak> for ::windows::core::IUnknown {
    fn from(value: LineBreak) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LineBreak> for ::windows::core::IUnknown {
    fn from(value: &LineBreak) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LineBreak {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LineBreak {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LineBreak> for ::windows::core::IInspectable {
    fn from(value: LineBreak) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LineBreak> for ::windows::core::IInspectable {
    fn from(value: &LineBreak) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LineBreak {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LineBreak {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LineBreak> for Inline {
    fn from(value: LineBreak) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&LineBreak> for Inline {
    fn from(value: &LineBreak) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Inline> for LineBreak {
    fn into_param(self) -> ::windows::core::Param<'a, Inline> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Inline> for &LineBreak {
    fn into_param(self) -> ::windows::core::Param<'a, Inline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Inline>::into(self))
    }
}
impl ::core::convert::From<LineBreak> for TextElement {
    fn from(value: LineBreak) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&LineBreak> for TextElement {
    fn from(value: &LineBreak) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for LineBreak {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for &LineBreak {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<TextElement>::into(self))
    }
}
impl ::core::convert::From<LineBreak> for super::DependencyObject {
    fn from(value: LineBreak) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&LineBreak> for super::DependencyObject {
    fn from(value: &LineBreak) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for LineBreak {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &LineBreak {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for LineBreak {}
unsafe impl ::core::marker::Sync for LineBreak {}
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LogicalDirection(pub i32);
impl LogicalDirection {
    pub const Backward: Self = Self(0i32);
    pub const Forward: Self = Self(1i32);
}
impl ::core::marker::Copy for LogicalDirection {}
impl ::core::clone::Clone for LogicalDirection {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LogicalDirection {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for LogicalDirection {
    type Abi = Self;
}
impl ::core::fmt::Debug for LogicalDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LogicalDirection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LogicalDirection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Documents.LogicalDirection;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct Paragraph(::windows::core::IUnknown);
impl Paragraph {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            Paragraph,
            ::windows::core::IActivationFactory,
        > = ::windows::core::FactoryCache::from_library(b"Microsoft.UI.Xaml.dll\0");
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn TextAlignment(&self) -> ::windows::core::Result<super::TextAlignment> {
        let this = &::windows::core::Interface::cast::<IBlock>(self)?;
        unsafe {
            let mut result__: super::TextAlignment = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TextAlignment)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::TextAlignment>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetTextAlignment(&self, value: super::TextAlignment) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBlock>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetTextAlignment)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn HorizontalTextAlignment(&self) -> ::windows::core::Result<super::TextAlignment> {
        let this = &::windows::core::Interface::cast::<IBlock>(self)?;
        unsafe {
            let mut result__: super::TextAlignment = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HorizontalTextAlignment)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::TextAlignment>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetHorizontalTextAlignment(
        &self,
        value: super::TextAlignment,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBlock>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetHorizontalTextAlignment)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn LineHeight(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IBlock>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LineHeight)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetLineHeight(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBlock>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetLineHeight)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn LineStackingStrategy(&self) -> ::windows::core::Result<super::LineStackingStrategy> {
        let this = &::windows::core::Interface::cast::<IBlock>(self)?;
        unsafe {
            let mut result__: super::LineStackingStrategy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LineStackingStrategy)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::LineStackingStrategy>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetLineStackingStrategy(
        &self,
        value: super::LineStackingStrategy,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBlock>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetLineStackingStrategy)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Margin(&self) -> ::windows::core::Result<super::Thickness> {
        let this = &::windows::core::Interface::cast::<IBlock>(self)?;
        unsafe {
            let mut result__: super::Thickness = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Margin)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Thickness>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetMargin<'a, Param0: ::windows::core::IntoParam<'a, super::Thickness>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBlock>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetMargin)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ClearValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).ClearValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ReadLocalValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReadLocalValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetAnimationBaseValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAnimationBaseValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RegisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::core::Result<i64> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RegisterPropertyChangedCallback)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                callback.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn UnregisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).UnregisterPropertyChangedCallback)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Dispatcher(&self) -> ::windows::core::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Dispatcher)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Dispatching\"`*"]
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DispatcherQueue)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Inlines(&self) -> ::windows::core::Result<InlineCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Inlines)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InlineCollection>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn TextIndent(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TextIndent)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetTextIndent(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetTextIndent)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn TextIndentProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IParagraphStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TextIndentProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontSize(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontSize)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFontSize(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontSize)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows::core::Result<super::Media::FontFamily> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontFamily)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFontFamily<'a, Param0: ::windows::core::IntoParam<'a, super::Media::FontFamily>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontFamily)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontWeight(&self) -> ::windows::core::Result<::windows::UI::Text::FontWeight> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontWeight = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontWeight)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFontWeight<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::UI::Text::FontWeight>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontWeight)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontStyle(&self) -> ::windows::core::Result<::windows::UI::Text::FontStyle> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStyle = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontStyle)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStyle>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontStyle)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontStretch(&self) -> ::windows::core::Result<::windows::UI::Text::FontStretch> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStretch = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontStretch)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStretch>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontStretch)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn CharacterSpacing(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CharacterSpacing)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetCharacterSpacing)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Foreground(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Foreground)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetForeground<'a, Param0: ::windows::core::IntoParam<'a, super::Media::Brush>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetForeground)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Language)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetLanguage)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsTextScaleFactorEnabled)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsTextScaleFactorEnabled)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn TextDecorations(&self) -> ::windows::core::Result<::windows::UI::Text::TextDecorations> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::TextDecorations = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TextDecorations)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::TextDecorations>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetTextDecorations)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ContentStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContentStart)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ContentEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContentEnd)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ElementStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ElementStart)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ElementEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ElementEnd)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn AllowFocusOnInteraction(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AllowFocusOnInteraction)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetAllowFocusOnInteraction)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn AccessKey(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKey)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetAccessKey<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetAccessKey)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExitDisplayModeOnAccessKeyInvoked)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetExitDisplayModeOnAccessKeyInvoked)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn IsAccessKeyScope(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsAccessKeyScope)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsAccessKeyScope)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn AccessKeyScopeOwner(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyScopeOwner)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetAccessKeyScopeOwner<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetAccessKeyScopeOwner)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::core::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::Input::KeyTipPlacementMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyTipPlacementMode)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetKeyTipPlacementMode)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyTipHorizontalOffset)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetKeyTipHorizontalOffset)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn KeyTipVerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyTipVerticalOffset)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetKeyTipVerticalOffset)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn XamlRoot(&self) -> ::windows::core::Result<super::XamlRoot> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XamlRoot)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::XamlRoot>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetXamlRoot<'a, Param0: ::windows::core::IntoParam<'a, super::XamlRoot>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetXamlRoot)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayRequested<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyDisplayRequested)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveAccessKeyDisplayRequested<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveAccessKeyDisplayRequested)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayDismissedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyDisplayDismissed)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveAccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveAccessKeyDisplayDismissed)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyInvoked<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyInvokedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyInvoked)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveAccessKeyInvoked<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveAccessKeyInvoked)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FindName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindName)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IParagraphStatics<R, F: FnOnce(&IParagraphStatics) -> ::windows::core::Result<R>>(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Paragraph, IParagraphStatics> =
            ::windows::core::FactoryCache::from_library(b"Microsoft.UI.Xaml.dll\0");
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Paragraph {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Paragraph {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Paragraph {}
impl ::core::fmt::Debug for Paragraph {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Paragraph").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Paragraph {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.Paragraph;{9ed64c77-329d-502f-a257-f58398edab51})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for Paragraph {
    type Vtable = IParagraph_Vtbl;
    const IID: ::windows::core::GUID = <IParagraph as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Paragraph {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Paragraph";
}
impl ::core::convert::From<Paragraph> for ::windows::core::IUnknown {
    fn from(value: Paragraph) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Paragraph> for ::windows::core::IUnknown {
    fn from(value: &Paragraph) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Paragraph {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Paragraph {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Paragraph> for ::windows::core::IInspectable {
    fn from(value: Paragraph) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Paragraph> for ::windows::core::IInspectable {
    fn from(value: &Paragraph) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Paragraph {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Paragraph {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Paragraph> for Block {
    fn from(value: Paragraph) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Paragraph> for Block {
    fn from(value: &Paragraph) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Block> for Paragraph {
    fn into_param(self) -> ::windows::core::Param<'a, Block> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Block> for &Paragraph {
    fn into_param(self) -> ::windows::core::Param<'a, Block> {
        ::windows::core::Param::Owned(::core::convert::Into::<Block>::into(self))
    }
}
impl ::core::convert::From<Paragraph> for TextElement {
    fn from(value: Paragraph) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Paragraph> for TextElement {
    fn from(value: &Paragraph) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for Paragraph {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for &Paragraph {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<TextElement>::into(self))
    }
}
impl ::core::convert::From<Paragraph> for super::DependencyObject {
    fn from(value: Paragraph) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Paragraph> for super::DependencyObject {
    fn from(value: &Paragraph) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for Paragraph {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &Paragraph {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for Paragraph {}
unsafe impl ::core::marker::Sync for Paragraph {}
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct Run(::windows::core::IUnknown);
impl Run {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Run, ::windows::core::IActivationFactory> =
            ::windows::core::FactoryCache::from_library(b"Microsoft.UI.Xaml.dll\0");
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ClearValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).ClearValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ReadLocalValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReadLocalValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetAnimationBaseValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAnimationBaseValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RegisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::core::Result<i64> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RegisterPropertyChangedCallback)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                callback.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn UnregisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).UnregisterPropertyChangedCallback)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Dispatcher(&self) -> ::windows::core::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Dispatcher)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Dispatching\"`*"]
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DispatcherQueue)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Text)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetText)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FlowDirection(&self) -> ::windows::core::Result<super::FlowDirection> {
        let this = self;
        unsafe {
            let mut result__: super::FlowDirection = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FlowDirection)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::FlowDirection>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFlowDirection(&self, value: super::FlowDirection) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFlowDirection)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FlowDirectionProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IRunStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FlowDirectionProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontSize(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontSize)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFontSize(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontSize)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows::core::Result<super::Media::FontFamily> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontFamily)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFontFamily<'a, Param0: ::windows::core::IntoParam<'a, super::Media::FontFamily>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontFamily)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontWeight(&self) -> ::windows::core::Result<::windows::UI::Text::FontWeight> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontWeight = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontWeight)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFontWeight<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::UI::Text::FontWeight>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontWeight)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontStyle(&self) -> ::windows::core::Result<::windows::UI::Text::FontStyle> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStyle = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontStyle)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStyle>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontStyle)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontStretch(&self) -> ::windows::core::Result<::windows::UI::Text::FontStretch> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStretch = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontStretch)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStretch>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontStretch)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn CharacterSpacing(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CharacterSpacing)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetCharacterSpacing)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Foreground(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Foreground)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetForeground<'a, Param0: ::windows::core::IntoParam<'a, super::Media::Brush>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetForeground)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Language)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetLanguage)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsTextScaleFactorEnabled)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsTextScaleFactorEnabled)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn TextDecorations(&self) -> ::windows::core::Result<::windows::UI::Text::TextDecorations> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::TextDecorations = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TextDecorations)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::TextDecorations>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetTextDecorations)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ContentStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContentStart)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ContentEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContentEnd)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ElementStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ElementStart)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ElementEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ElementEnd)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn AllowFocusOnInteraction(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AllowFocusOnInteraction)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetAllowFocusOnInteraction)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn AccessKey(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKey)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetAccessKey<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetAccessKey)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExitDisplayModeOnAccessKeyInvoked)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetExitDisplayModeOnAccessKeyInvoked)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn IsAccessKeyScope(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsAccessKeyScope)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsAccessKeyScope)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn AccessKeyScopeOwner(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyScopeOwner)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetAccessKeyScopeOwner<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetAccessKeyScopeOwner)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::core::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::Input::KeyTipPlacementMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyTipPlacementMode)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetKeyTipPlacementMode)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyTipHorizontalOffset)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetKeyTipHorizontalOffset)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn KeyTipVerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyTipVerticalOffset)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetKeyTipVerticalOffset)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn XamlRoot(&self) -> ::windows::core::Result<super::XamlRoot> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XamlRoot)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::XamlRoot>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetXamlRoot<'a, Param0: ::windows::core::IntoParam<'a, super::XamlRoot>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetXamlRoot)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayRequested<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyDisplayRequested)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveAccessKeyDisplayRequested<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveAccessKeyDisplayRequested)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayDismissedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyDisplayDismissed)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveAccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveAccessKeyDisplayDismissed)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyInvoked<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyInvokedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyInvoked)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveAccessKeyInvoked<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveAccessKeyInvoked)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FindName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindName)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IRunStatics<R, F: FnOnce(&IRunStatics) -> ::windows::core::Result<R>>(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Run, IRunStatics> =
            ::windows::core::FactoryCache::from_library(b"Microsoft.UI.Xaml.dll\0");
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Run {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Run {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Run {}
impl ::core::fmt::Debug for Run {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Run").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Run {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.Run;{1f905239-37cb-590b-9132-3ffb7741906e})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for Run {
    type Vtable = IRun_Vtbl;
    const IID: ::windows::core::GUID = <IRun as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Run {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Run";
}
impl ::core::convert::From<Run> for ::windows::core::IUnknown {
    fn from(value: Run) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Run> for ::windows::core::IUnknown {
    fn from(value: &Run) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Run {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Run {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Run> for ::windows::core::IInspectable {
    fn from(value: Run) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Run> for ::windows::core::IInspectable {
    fn from(value: &Run) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Run {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Run {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Run> for Inline {
    fn from(value: Run) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Run> for Inline {
    fn from(value: &Run) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Inline> for Run {
    fn into_param(self) -> ::windows::core::Param<'a, Inline> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Inline> for &Run {
    fn into_param(self) -> ::windows::core::Param<'a, Inline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Inline>::into(self))
    }
}
impl ::core::convert::From<Run> for TextElement {
    fn from(value: Run) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Run> for TextElement {
    fn from(value: &Run) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for Run {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for &Run {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<TextElement>::into(self))
    }
}
impl ::core::convert::From<Run> for super::DependencyObject {
    fn from(value: Run) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Run> for super::DependencyObject {
    fn from(value: &Run) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for Run {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &Run {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for Run {}
unsafe impl ::core::marker::Sync for Run {}
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct Span(::windows::core::IUnknown);
impl Span {
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ClearValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).ClearValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ReadLocalValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReadLocalValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetAnimationBaseValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAnimationBaseValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RegisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::core::Result<i64> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RegisterPropertyChangedCallback)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                callback.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn UnregisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).UnregisterPropertyChangedCallback)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Dispatcher(&self) -> ::windows::core::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Dispatcher)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Dispatching\"`*"]
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DispatcherQueue)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Inlines(&self) -> ::windows::core::Result<InlineCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Inlines)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InlineCollection>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetInlines<'a, Param0: ::windows::core::IntoParam<'a, InlineCollection>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetInlines)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn new() -> ::windows::core::Result<Span> {
        Self::ISpanFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(
                ::core::mem::transmute_copy(this),
                ::core::ptr::null_mut(),
                &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _,
                &mut result__,
            )
            .from_abi::<Span>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn compose<T: ::windows::core::Compose>(compose: T) -> ::windows::core::Result<Span> {
        Self::ISpanFactory(|this| unsafe {
            let (derived__, base__) = ::windows::core::Compose::compose(compose);
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(
                ::core::mem::transmute_copy(this),
                ::core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .from_abi::<Span>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontSize(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontSize)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFontSize(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontSize)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows::core::Result<super::Media::FontFamily> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontFamily)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFontFamily<'a, Param0: ::windows::core::IntoParam<'a, super::Media::FontFamily>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontFamily)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontWeight(&self) -> ::windows::core::Result<::windows::UI::Text::FontWeight> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontWeight = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontWeight)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFontWeight<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::UI::Text::FontWeight>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontWeight)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontStyle(&self) -> ::windows::core::Result<::windows::UI::Text::FontStyle> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStyle = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontStyle)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStyle>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontStyle)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontStretch(&self) -> ::windows::core::Result<::windows::UI::Text::FontStretch> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStretch = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontStretch)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStretch>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontStretch)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn CharacterSpacing(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CharacterSpacing)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetCharacterSpacing)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Foreground(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Foreground)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetForeground<'a, Param0: ::windows::core::IntoParam<'a, super::Media::Brush>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetForeground)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Language)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetLanguage)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsTextScaleFactorEnabled)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsTextScaleFactorEnabled)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn TextDecorations(&self) -> ::windows::core::Result<::windows::UI::Text::TextDecorations> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::TextDecorations = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TextDecorations)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::TextDecorations>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetTextDecorations)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ContentStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContentStart)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ContentEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContentEnd)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ElementStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ElementStart)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ElementEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ElementEnd)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn AllowFocusOnInteraction(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AllowFocusOnInteraction)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetAllowFocusOnInteraction)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn AccessKey(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKey)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetAccessKey<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetAccessKey)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExitDisplayModeOnAccessKeyInvoked)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetExitDisplayModeOnAccessKeyInvoked)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn IsAccessKeyScope(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsAccessKeyScope)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsAccessKeyScope)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn AccessKeyScopeOwner(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyScopeOwner)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetAccessKeyScopeOwner<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetAccessKeyScopeOwner)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::core::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::Input::KeyTipPlacementMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyTipPlacementMode)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetKeyTipPlacementMode)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyTipHorizontalOffset)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetKeyTipHorizontalOffset)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn KeyTipVerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyTipVerticalOffset)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetKeyTipVerticalOffset)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn XamlRoot(&self) -> ::windows::core::Result<super::XamlRoot> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XamlRoot)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::XamlRoot>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetXamlRoot<'a, Param0: ::windows::core::IntoParam<'a, super::XamlRoot>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetXamlRoot)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayRequested<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyDisplayRequested)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveAccessKeyDisplayRequested<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveAccessKeyDisplayRequested)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayDismissedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyDisplayDismissed)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveAccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveAccessKeyDisplayDismissed)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyInvoked<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyInvokedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyInvoked)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveAccessKeyInvoked<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveAccessKeyInvoked)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FindName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindName)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc(hidden)]
    pub fn ISpanFactory<R, F: FnOnce(&ISpanFactory) -> ::windows::core::Result<R>>(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Span, ISpanFactory> =
            ::windows::core::FactoryCache::from_library(b"Microsoft.UI.Xaml.dll\0");
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Span {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Span {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Span {}
impl ::core::fmt::Debug for Span {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Span").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Span {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.Span;{91b93d4d-4e28-57b9-bffb-3566c2a3c2a1})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for Span {
    type Vtable = ISpan_Vtbl;
    const IID: ::windows::core::GUID = <ISpan as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Span {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Span";
}
impl ::core::convert::From<Span> for ::windows::core::IUnknown {
    fn from(value: Span) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Span> for ::windows::core::IUnknown {
    fn from(value: &Span) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Span {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Span {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Span> for ::windows::core::IInspectable {
    fn from(value: Span) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Span> for ::windows::core::IInspectable {
    fn from(value: &Span) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Span {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Span {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Span> for Inline {
    fn from(value: Span) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Span> for Inline {
    fn from(value: &Span) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Inline> for Span {
    fn into_param(self) -> ::windows::core::Param<'a, Inline> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Inline> for &Span {
    fn into_param(self) -> ::windows::core::Param<'a, Inline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Inline>::into(self))
    }
}
impl ::core::convert::From<Span> for TextElement {
    fn from(value: Span) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Span> for TextElement {
    fn from(value: &Span) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for Span {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for &Span {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<TextElement>::into(self))
    }
}
impl ::core::convert::From<Span> for super::DependencyObject {
    fn from(value: Span) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Span> for super::DependencyObject {
    fn from(value: &Span) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for Span {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &Span {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for Span {}
unsafe impl ::core::marker::Sync for Span {}
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct TextElement(::windows::core::IUnknown);
impl TextElement {
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ClearValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).ClearValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ReadLocalValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReadLocalValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetAnimationBaseValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAnimationBaseValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RegisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::core::Result<i64> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RegisterPropertyChangedCallback)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                callback.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn UnregisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).UnregisterPropertyChangedCallback)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Dispatcher(&self) -> ::windows::core::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Dispatcher)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Dispatching\"`*"]
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DispatcherQueue)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontSize(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontSize)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFontSize(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontSize)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows::core::Result<super::Media::FontFamily> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontFamily)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFontFamily<'a, Param0: ::windows::core::IntoParam<'a, super::Media::FontFamily>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontFamily)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontWeight(&self) -> ::windows::core::Result<::windows::UI::Text::FontWeight> {
        let this = self;
        unsafe {
            let mut result__: ::windows::UI::Text::FontWeight = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontWeight)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFontWeight<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::UI::Text::FontWeight>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontWeight)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontStyle(&self) -> ::windows::core::Result<::windows::UI::Text::FontStyle> {
        let this = self;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStyle = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontStyle)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStyle>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontStyle)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontStretch(&self) -> ::windows::core::Result<::windows::UI::Text::FontStretch> {
        let this = self;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStretch = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontStretch)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStretch>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontStretch)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn CharacterSpacing(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CharacterSpacing)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetCharacterSpacing)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Foreground(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Foreground)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetForeground<'a, Param0: ::windows::core::IntoParam<'a, super::Media::Brush>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetForeground)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Language)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetLanguage)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsTextScaleFactorEnabled)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsTextScaleFactorEnabled)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn TextDecorations(&self) -> ::windows::core::Result<::windows::UI::Text::TextDecorations> {
        let this = self;
        unsafe {
            let mut result__: ::windows::UI::Text::TextDecorations = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TextDecorations)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::TextDecorations>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetTextDecorations)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ContentStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContentStart)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ContentEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContentEnd)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ElementStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ElementStart)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ElementEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ElementEnd)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn AllowFocusOnInteraction(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AllowFocusOnInteraction)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetAllowFocusOnInteraction)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn AccessKey(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKey)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetAccessKey<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetAccessKey)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExitDisplayModeOnAccessKeyInvoked)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetExitDisplayModeOnAccessKeyInvoked)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn IsAccessKeyScope(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsAccessKeyScope)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsAccessKeyScope)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn AccessKeyScopeOwner(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyScopeOwner)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetAccessKeyScopeOwner<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetAccessKeyScopeOwner)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::core::Result<super::Input::KeyTipPlacementMode> {
        let this = self;
        unsafe {
            let mut result__: super::Input::KeyTipPlacementMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyTipPlacementMode)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetKeyTipPlacementMode)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyTipHorizontalOffset)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetKeyTipHorizontalOffset)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn KeyTipVerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyTipVerticalOffset)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetKeyTipVerticalOffset)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn XamlRoot(&self) -> ::windows::core::Result<super::XamlRoot> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XamlRoot)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::XamlRoot>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetXamlRoot<'a, Param0: ::windows::core::IntoParam<'a, super::XamlRoot>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetXamlRoot)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayRequested<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyDisplayRequested)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveAccessKeyDisplayRequested<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveAccessKeyDisplayRequested)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayDismissedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyDisplayDismissed)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveAccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveAccessKeyDisplayDismissed)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyInvoked<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyInvokedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyInvoked)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveAccessKeyInvoked<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveAccessKeyInvoked)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FindName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindName)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontSizeProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontSizeProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontFamilyProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontFamilyProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontWeightProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontWeightProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontStyleProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontStyleProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontStretchProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontStretchProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn CharacterSpacingProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CharacterSpacingProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ForegroundProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ForegroundProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn LanguageProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LanguageProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn IsTextScaleFactorEnabledProperty() -> ::windows::core::Result<super::DependencyProperty>
    {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsTextScaleFactorEnabledProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn TextDecorationsProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TextDecorationsProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn AllowFocusOnInteractionProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AllowFocusOnInteractionProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn AccessKeyProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ExitDisplayModeOnAccessKeyInvokedProperty(
    ) -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExitDisplayModeOnAccessKeyInvokedProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn IsAccessKeyScopeProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsAccessKeyScopeProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn AccessKeyScopeOwnerProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyScopeOwnerProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn KeyTipPlacementModeProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyTipPlacementModeProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn KeyTipHorizontalOffsetProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyTipHorizontalOffsetProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn KeyTipVerticalOffsetProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyTipVerticalOffsetProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITextElementStatics<R, F: FnOnce(&ITextElementStatics) -> ::windows::core::Result<R>>(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TextElement, ITextElementStatics> =
            ::windows::core::FactoryCache::from_library(b"Microsoft.UI.Xaml.dll\0");
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for TextElement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TextElement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TextElement {}
impl ::core::fmt::Debug for TextElement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TextElement").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TextElement {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.TextElement;{a122ba22-833f-5220-a47e-6cd507531abe})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for TextElement {
    type Vtable = ITextElement_Vtbl;
    const IID: ::windows::core::GUID = <ITextElement as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for TextElement {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.TextElement";
}
impl ::core::convert::From<TextElement> for ::windows::core::IUnknown {
    fn from(value: TextElement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TextElement> for ::windows::core::IUnknown {
    fn from(value: &TextElement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TextElement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TextElement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TextElement> for ::windows::core::IInspectable {
    fn from(value: TextElement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TextElement> for ::windows::core::IInspectable {
    fn from(value: &TextElement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TextElement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TextElement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TextElement> for super::DependencyObject {
    fn from(value: TextElement) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&TextElement> for super::DependencyObject {
    fn from(value: &TextElement) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for TextElement {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &TextElement {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for TextElement {}
unsafe impl ::core::marker::Sync for TextElement {}
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct TextHighlighter(::windows::core::IUnknown);
impl TextHighlighter {
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Ranges(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Collections::IVector<TextRange>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Ranges)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IVector<TextRange>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Foreground(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Foreground)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetForeground<'a, Param0: ::windows::core::IntoParam<'a, super::Media::Brush>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetForeground)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Background(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Background)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetBackground<'a, Param0: ::windows::core::IntoParam<'a, super::Media::Brush>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetBackground)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn new() -> ::windows::core::Result<TextHighlighter> {
        Self::ITextHighlighterFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(
                ::core::mem::transmute_copy(this),
                ::core::ptr::null_mut(),
                &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _,
                &mut result__,
            )
            .from_abi::<TextHighlighter>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn compose<T: ::windows::core::Compose>(
        compose: T,
    ) -> ::windows::core::Result<TextHighlighter> {
        Self::ITextHighlighterFactory(|this| unsafe {
            let (derived__, base__) = ::windows::core::Compose::compose(compose);
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(
                ::core::mem::transmute_copy(this),
                ::core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .from_abi::<TextHighlighter>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ForegroundProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextHighlighterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ForegroundProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn BackgroundProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextHighlighterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BackgroundProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITextHighlighterFactory<
        R,
        F: FnOnce(&ITextHighlighterFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TextHighlighter, ITextHighlighterFactory> =
            ::windows::core::FactoryCache::from_library(b"Microsoft.UI.Xaml.dll\0");
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn ITextHighlighterStatics<
        R,
        F: FnOnce(&ITextHighlighterStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TextHighlighter, ITextHighlighterStatics> =
            ::windows::core::FactoryCache::from_library(b"Microsoft.UI.Xaml.dll\0");
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for TextHighlighter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TextHighlighter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TextHighlighter {}
impl ::core::fmt::Debug for TextHighlighter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TextHighlighter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TextHighlighter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.TextHighlighter;{b756e861-1d2b-5f6f-81fd-c51a5bc068ff})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for TextHighlighter {
    type Vtable = ITextHighlighter_Vtbl;
    const IID: ::windows::core::GUID = <ITextHighlighter as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for TextHighlighter {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.TextHighlighter";
}
impl ::core::convert::From<TextHighlighter> for ::windows::core::IUnknown {
    fn from(value: TextHighlighter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TextHighlighter> for ::windows::core::IUnknown {
    fn from(value: &TextHighlighter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TextHighlighter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TextHighlighter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TextHighlighter> for ::windows::core::IInspectable {
    fn from(value: TextHighlighter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TextHighlighter> for ::windows::core::IInspectable {
    fn from(value: &TextHighlighter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TextHighlighter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TextHighlighter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TextHighlighter {}
unsafe impl ::core::marker::Sync for TextHighlighter {}
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct TextHighlighterBase(::windows::core::IUnknown);
impl TextHighlighterBase {
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ClearValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).ClearValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ReadLocalValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReadLocalValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetAnimationBaseValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAnimationBaseValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RegisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::core::Result<i64> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RegisterPropertyChangedCallback)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                callback.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn UnregisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).UnregisterPropertyChangedCallback)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Dispatcher(&self) -> ::windows::core::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Dispatcher)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Dispatching\"`*"]
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DispatcherQueue)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
}
impl ::core::clone::Clone for TextHighlighterBase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TextHighlighterBase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TextHighlighterBase {}
impl ::core::fmt::Debug for TextHighlighterBase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TextHighlighterBase").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TextHighlighterBase {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Documents.TextHighlighterBase;{5c21aaf0-3a17-5468-8aac-be14db0ed8c1})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for TextHighlighterBase {
    type Vtable = ITextHighlighterBase_Vtbl;
    const IID: ::windows::core::GUID = <ITextHighlighterBase as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for TextHighlighterBase {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.TextHighlighterBase";
}
impl ::core::convert::From<TextHighlighterBase> for ::windows::core::IUnknown {
    fn from(value: TextHighlighterBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TextHighlighterBase> for ::windows::core::IUnknown {
    fn from(value: &TextHighlighterBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TextHighlighterBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TextHighlighterBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TextHighlighterBase> for ::windows::core::IInspectable {
    fn from(value: TextHighlighterBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TextHighlighterBase> for ::windows::core::IInspectable {
    fn from(value: &TextHighlighterBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TextHighlighterBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TextHighlighterBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TextHighlighterBase> for super::DependencyObject {
    fn from(value: TextHighlighterBase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&TextHighlighterBase> for super::DependencyObject {
    fn from(value: &TextHighlighterBase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for TextHighlighterBase {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &TextHighlighterBase {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for TextHighlighterBase {}
unsafe impl ::core::marker::Sync for TextHighlighterBase {}
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct TextPointer(::windows::core::IUnknown);
impl TextPointer {
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Parent(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Parent)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn VisualParent(&self) -> ::windows::core::Result<super::FrameworkElement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VisualParent)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::FrameworkElement>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn LogicalDirection(&self) -> ::windows::core::Result<LogicalDirection> {
        let this = self;
        unsafe {
            let mut result__: LogicalDirection = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LogicalDirection)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<LogicalDirection>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Offset(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Offset)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetCharacterRect(
        &self,
        direction: LogicalDirection,
    ) -> ::windows::core::Result<::windows::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetCharacterRect)(
                ::core::mem::transmute_copy(this),
                direction,
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetPositionAtOffset(
        &self,
        offset: i32,
        direction: LogicalDirection,
    ) -> ::windows::core::Result<TextPointer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetPositionAtOffset)(
                ::core::mem::transmute_copy(this),
                offset,
                direction,
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
}
impl ::core::clone::Clone for TextPointer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TextPointer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TextPointer {}
impl ::core::fmt::Debug for TextPointer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TextPointer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TextPointer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.TextPointer;{842eb385-ee41-5930-979b-438fa7525a51})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for TextPointer {
    type Vtable = ITextPointer_Vtbl;
    const IID: ::windows::core::GUID = <ITextPointer as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for TextPointer {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.TextPointer";
}
impl ::core::convert::From<TextPointer> for ::windows::core::IUnknown {
    fn from(value: TextPointer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TextPointer> for ::windows::core::IUnknown {
    fn from(value: &TextPointer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TextPointer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TextPointer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TextPointer> for ::windows::core::IInspectable {
    fn from(value: TextPointer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TextPointer> for ::windows::core::IInspectable {
    fn from(value: &TextPointer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TextPointer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TextPointer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TextPointer {}
unsafe impl ::core::marker::Sync for TextPointer {}
#[repr(C)]
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
pub struct TextRange {
    pub StartIndex: i32,
    pub Length: i32,
}
impl ::core::marker::Copy for TextRange {}
impl ::core::clone::Clone for TextRange {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TextRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TextRange")
            .field("StartIndex", &self.StartIndex)
            .field("Length", &self.Length)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for TextRange {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for TextRange {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"struct(Microsoft.UI.Xaml.Documents.TextRange;i4;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for TextRange {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            ::windows::core::memcmp(
                self as *const _ as _,
                other as *const _ as _,
                core::mem::size_of::<TextRange>(),
            ) == 0
        }
    }
}
impl ::core::cmp::Eq for TextRange {}
impl ::core::default::Default for TextRange {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct Typography(::windows::core::IUnknown);
impl Typography {
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn AnnotationAlternatesProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AnnotationAlternatesProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetAnnotationAlternates<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<i32> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAnnotationAlternates)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetAnnotationAlternates<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: i32,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).SetAnnotationAlternates)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn EastAsianExpertFormsProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EastAsianExpertFormsProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetEastAsianExpertForms<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetEastAsianExpertForms)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetEastAsianExpertForms<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).SetEastAsianExpertForms)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn EastAsianLanguageProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EastAsianLanguageProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetEastAsianLanguage<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<super::FontEastAsianLanguage> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: super::FontEastAsianLanguage = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetEastAsianLanguage)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::FontEastAsianLanguage>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetEastAsianLanguage<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: super::FontEastAsianLanguage,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).SetEastAsianLanguage)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn EastAsianWidthsProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EastAsianWidthsProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetEastAsianWidths<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<super::FontEastAsianWidths> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: super::FontEastAsianWidths = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetEastAsianWidths)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::FontEastAsianWidths>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetEastAsianWidths<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: super::FontEastAsianWidths,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).SetEastAsianWidths)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn StandardLigaturesProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StandardLigaturesProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetStandardLigatures<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStandardLigatures)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetStandardLigatures<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).SetStandardLigatures)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ContextualLigaturesProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContextualLigaturesProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetContextualLigatures<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetContextualLigatures)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetContextualLigatures<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).SetContextualLigatures)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn DiscretionaryLigaturesProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DiscretionaryLigaturesProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetDiscretionaryLigatures<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDiscretionaryLigatures)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetDiscretionaryLigatures<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).SetDiscretionaryLigatures)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn HistoricalLigaturesProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HistoricalLigaturesProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetHistoricalLigatures<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetHistoricalLigatures)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetHistoricalLigatures<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).SetHistoricalLigatures)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn StandardSwashesProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StandardSwashesProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetStandardSwashes<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<i32> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStandardSwashes)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetStandardSwashes<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: i32,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).SetStandardSwashes)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ContextualSwashesProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContextualSwashesProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetContextualSwashes<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<i32> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetContextualSwashes)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetContextualSwashes<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: i32,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).SetContextualSwashes)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ContextualAlternatesProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContextualAlternatesProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetContextualAlternates<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetContextualAlternates)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetContextualAlternates<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).SetContextualAlternates)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn StylisticAlternatesProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StylisticAlternatesProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetStylisticAlternates<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<i32> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStylisticAlternates)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetStylisticAlternates<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: i32,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).SetStylisticAlternates)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn StylisticSet1Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StylisticSet1Property)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetStylisticSet1<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStylisticSet1)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetStylisticSet1<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).SetStylisticSet1)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn StylisticSet2Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StylisticSet2Property)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetStylisticSet2<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStylisticSet2)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetStylisticSet2<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).SetStylisticSet2)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn StylisticSet3Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StylisticSet3Property)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetStylisticSet3<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStylisticSet3)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetStylisticSet3<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).SetStylisticSet3)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn StylisticSet4Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StylisticSet4Property)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetStylisticSet4<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStylisticSet4)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetStylisticSet4<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).SetStylisticSet4)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn StylisticSet5Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StylisticSet5Property)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetStylisticSet5<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStylisticSet5)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetStylisticSet5<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).SetStylisticSet5)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn StylisticSet6Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StylisticSet6Property)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetStylisticSet6<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStylisticSet6)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetStylisticSet6<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).SetStylisticSet6)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn StylisticSet7Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StylisticSet7Property)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetStylisticSet7<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStylisticSet7)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetStylisticSet7<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).SetStylisticSet7)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn StylisticSet8Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StylisticSet8Property)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetStylisticSet8<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStylisticSet8)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetStylisticSet8<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).SetStylisticSet8)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn StylisticSet9Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StylisticSet9Property)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetStylisticSet9<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStylisticSet9)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetStylisticSet9<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).SetStylisticSet9)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn StylisticSet10Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StylisticSet10Property)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetStylisticSet10<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStylisticSet10)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetStylisticSet10<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).SetStylisticSet10)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn StylisticSet11Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StylisticSet11Property)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetStylisticSet11<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStylisticSet11)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetStylisticSet11<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).SetStylisticSet11)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn StylisticSet12Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StylisticSet12Property)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetStylisticSet12<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStylisticSet12)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetStylisticSet12<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).SetStylisticSet12)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn StylisticSet13Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StylisticSet13Property)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetStylisticSet13<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStylisticSet13)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetStylisticSet13<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).SetStylisticSet13)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn StylisticSet14Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StylisticSet14Property)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetStylisticSet14<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStylisticSet14)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetStylisticSet14<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).SetStylisticSet14)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn StylisticSet15Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StylisticSet15Property)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetStylisticSet15<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStylisticSet15)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetStylisticSet15<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).SetStylisticSet15)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn StylisticSet16Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StylisticSet16Property)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetStylisticSet16<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStylisticSet16)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetStylisticSet16<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).SetStylisticSet16)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn StylisticSet17Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StylisticSet17Property)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetStylisticSet17<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStylisticSet17)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetStylisticSet17<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).SetStylisticSet17)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn StylisticSet18Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StylisticSet18Property)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetStylisticSet18<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStylisticSet18)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetStylisticSet18<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).SetStylisticSet18)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn StylisticSet19Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StylisticSet19Property)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetStylisticSet19<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStylisticSet19)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetStylisticSet19<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).SetStylisticSet19)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn StylisticSet20Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StylisticSet20Property)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetStylisticSet20<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStylisticSet20)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetStylisticSet20<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).SetStylisticSet20)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn CapitalsProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CapitalsProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetCapitals<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::core::Result<super::FontCapitals> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: super::FontCapitals = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetCapitals)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::FontCapitals>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetCapitals<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
        value: super::FontCapitals,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).SetCapitals)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn CapitalSpacingProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CapitalSpacingProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetCapitalSpacing<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetCapitalSpacing)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetCapitalSpacing<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).SetCapitalSpacing)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn KerningProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KerningProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetKerning<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetKerning)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetKerning<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).SetKerning)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn CaseSensitiveFormsProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CaseSensitiveFormsProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetCaseSensitiveForms<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetCaseSensitiveForms)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetCaseSensitiveForms<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).SetCaseSensitiveForms)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn HistoricalFormsProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HistoricalFormsProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetHistoricalForms<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetHistoricalForms)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetHistoricalForms<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).SetHistoricalForms)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FractionProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FractionProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetFraction<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::core::Result<super::FontFraction> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: super::FontFraction = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFraction)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::FontFraction>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFraction<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
        value: super::FontFraction,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).SetFraction)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn NumeralStyleProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NumeralStyleProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetNumeralStyle<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::core::Result<super::FontNumeralStyle> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: super::FontNumeralStyle = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetNumeralStyle)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::FontNumeralStyle>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetNumeralStyle<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
        value: super::FontNumeralStyle,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).SetNumeralStyle)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn NumeralAlignmentProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NumeralAlignmentProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetNumeralAlignment<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<super::FontNumeralAlignment> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: super::FontNumeralAlignment = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetNumeralAlignment)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::FontNumeralAlignment>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetNumeralAlignment<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: super::FontNumeralAlignment,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).SetNumeralAlignment)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SlashedZeroProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SlashedZeroProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetSlashedZero<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetSlashedZero)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetSlashedZero<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).SetSlashedZero)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn MathematicalGreekProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MathematicalGreekProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetMathematicalGreek<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetMathematicalGreek)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetMathematicalGreek<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).SetMathematicalGreek)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn VariantsProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VariantsProperty)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetVariants<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::core::Result<super::FontVariants> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: super::FontVariants = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetVariants)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::FontVariants>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetVariants<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
        value: super::FontVariants,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).SetVariants)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc(hidden)]
    pub fn ITypographyStatics<R, F: FnOnce(&ITypographyStatics) -> ::windows::core::Result<R>>(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Typography, ITypographyStatics> =
            ::windows::core::FactoryCache::from_library(b"Microsoft.UI.Xaml.dll\0");
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Typography {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Typography {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Typography {}
impl ::core::fmt::Debug for Typography {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Typography").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Typography {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.Typography;{fa27e2e3-be5e-5d21-9a5e-90cf102af828})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for Typography {
    type Vtable = ITypography_Vtbl;
    const IID: ::windows::core::GUID = <ITypography as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Typography {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Typography";
}
impl ::core::convert::From<Typography> for ::windows::core::IUnknown {
    fn from(value: Typography) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Typography> for ::windows::core::IUnknown {
    fn from(value: &Typography) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Typography {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Typography {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Typography> for ::windows::core::IInspectable {
    fn from(value: Typography) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Typography> for ::windows::core::IInspectable {
    fn from(value: &Typography) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Typography {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Typography {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for Typography {}
unsafe impl ::core::marker::Sync for Typography {}
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct Underline(::windows::core::IUnknown);
impl Underline {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            Underline,
            ::windows::core::IActivationFactory,
        > = ::windows::core::FactoryCache::from_library(b"Microsoft.UI.Xaml.dll\0");
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ClearValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).ClearValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ReadLocalValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReadLocalValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetAnimationBaseValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAnimationBaseValue)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RegisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::core::Result<i64> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RegisterPropertyChangedCallback)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                callback.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn UnregisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).UnregisterPropertyChangedCallback)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Dispatcher(&self) -> ::windows::core::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Dispatcher)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Dispatching\"`*"]
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DispatcherQueue)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Inlines(&self) -> ::windows::core::Result<InlineCollection> {
        let this = &::windows::core::Interface::cast::<ISpan>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Inlines)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InlineCollection>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetInlines<'a, Param0: ::windows::core::IntoParam<'a, InlineCollection>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISpan>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetInlines)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontSize(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontSize)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFontSize(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontSize)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows::core::Result<super::Media::FontFamily> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontFamily)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFontFamily<'a, Param0: ::windows::core::IntoParam<'a, super::Media::FontFamily>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontFamily)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontWeight(&self) -> ::windows::core::Result<::windows::UI::Text::FontWeight> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontWeight = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontWeight)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFontWeight<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::UI::Text::FontWeight>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontWeight)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontStyle(&self) -> ::windows::core::Result<::windows::UI::Text::FontStyle> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStyle = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontStyle)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStyle>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontStyle)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontStretch(&self) -> ::windows::core::Result<::windows::UI::Text::FontStretch> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStretch = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontStretch)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStretch>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFontStretch)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn CharacterSpacing(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CharacterSpacing)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetCharacterSpacing)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Foreground(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Foreground)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetForeground<'a, Param0: ::windows::core::IntoParam<'a, super::Media::Brush>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetForeground)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Language)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetLanguage)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsTextScaleFactorEnabled)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsTextScaleFactorEnabled)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn TextDecorations(&self) -> ::windows::core::Result<::windows::UI::Text::TextDecorations> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::TextDecorations = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TextDecorations)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::TextDecorations>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetTextDecorations)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ContentStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContentStart)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ContentEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContentEnd)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ElementStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ElementStart)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ElementEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ElementEnd)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn AllowFocusOnInteraction(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AllowFocusOnInteraction)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetAllowFocusOnInteraction)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn AccessKey(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKey)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetAccessKey<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetAccessKey)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExitDisplayModeOnAccessKeyInvoked)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetExitDisplayModeOnAccessKeyInvoked)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn IsAccessKeyScope(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsAccessKeyScope)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsAccessKeyScope)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn AccessKeyScopeOwner(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyScopeOwner)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetAccessKeyScopeOwner<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetAccessKeyScopeOwner)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::core::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::Input::KeyTipPlacementMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyTipPlacementMode)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetKeyTipPlacementMode)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyTipHorizontalOffset)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetKeyTipHorizontalOffset)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn KeyTipVerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyTipVerticalOffset)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetKeyTipVerticalOffset)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn XamlRoot(&self) -> ::windows::core::Result<super::XamlRoot> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XamlRoot)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::XamlRoot>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetXamlRoot<'a, Param0: ::windows::core::IntoParam<'a, super::XamlRoot>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetXamlRoot)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayRequested<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyDisplayRequested)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveAccessKeyDisplayRequested<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveAccessKeyDisplayRequested)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayDismissedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyDisplayDismissed)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveAccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveAccessKeyDisplayDismissed)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyInvoked<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyInvokedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyInvoked)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn RemoveAccessKeyInvoked<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveAccessKeyInvoked)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FindName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindName)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
impl ::core::clone::Clone for Underline {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Underline {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Underline {}
impl ::core::fmt::Debug for Underline {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Underline").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Underline {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.Underline;{68aaec6e-ea71-5ed2-b83e-91684b25efb9})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for Underline {
    type Vtable = IUnderline_Vtbl;
    const IID: ::windows::core::GUID = <IUnderline as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Underline {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Underline";
}
impl ::core::convert::From<Underline> for ::windows::core::IUnknown {
    fn from(value: Underline) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Underline> for ::windows::core::IUnknown {
    fn from(value: &Underline) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Underline {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Underline {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Underline> for ::windows::core::IInspectable {
    fn from(value: Underline) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Underline> for ::windows::core::IInspectable {
    fn from(value: &Underline) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Underline {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Underline {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Underline> for Span {
    fn from(value: Underline) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Underline> for Span {
    fn from(value: &Underline) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Span> for Underline {
    fn into_param(self) -> ::windows::core::Param<'a, Span> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Span> for &Underline {
    fn into_param(self) -> ::windows::core::Param<'a, Span> {
        ::windows::core::Param::Owned(::core::convert::Into::<Span>::into(self))
    }
}
impl ::core::convert::From<Underline> for Inline {
    fn from(value: Underline) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Underline> for Inline {
    fn from(value: &Underline) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Inline> for Underline {
    fn into_param(self) -> ::windows::core::Param<'a, Inline> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Inline> for &Underline {
    fn into_param(self) -> ::windows::core::Param<'a, Inline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Inline>::into(self))
    }
}
impl ::core::convert::From<Underline> for TextElement {
    fn from(value: Underline) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Underline> for TextElement {
    fn from(value: &Underline) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for Underline {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for &Underline {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<TextElement>::into(self))
    }
}
impl ::core::convert::From<Underline> for super::DependencyObject {
    fn from(value: Underline) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Underline> for super::DependencyObject {
    fn from(value: &Underline) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for Underline {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &Underline {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for Underline {}
unsafe impl ::core::marker::Sync for Underline {}
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UnderlineStyle(pub i32);
impl UnderlineStyle {
    pub const None: Self = Self(0i32);
    pub const Single: Self = Self(1i32);
}
impl ::core::marker::Copy for UnderlineStyle {}
impl ::core::clone::Clone for UnderlineStyle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UnderlineStyle {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UnderlineStyle {
    type Abi = Self;
}
impl ::core::fmt::Debug for UnderlineStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UnderlineStyle").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UnderlineStyle {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Documents.UnderlineStyle;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
