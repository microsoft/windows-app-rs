#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct Block(pub ::windows::core::IInspectable);
impl Block {
    pub fn TextAlignment(&self) -> ::windows::core::Result<super::TextAlignment> {
        let this = self;
        unsafe {
            let mut result__: super::TextAlignment = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::TextAlignment>(result__)
        }
    }
    pub fn SetTextAlignment(&self, value: super::TextAlignment) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn HorizontalTextAlignment(&self) -> ::windows::core::Result<super::TextAlignment> {
        let this = self;
        unsafe {
            let mut result__: super::TextAlignment = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::TextAlignment>(result__)
        }
    }
    pub fn SetHorizontalTextAlignment(
        &self,
        value: super::TextAlignment,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn LineHeight(&self) -> ::windows::core::Result<f64> {
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
    pub fn SetLineHeight(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn LineStackingStrategy(&self) -> ::windows::core::Result<super::LineStackingStrategy> {
        let this = self;
        unsafe {
            let mut result__: super::LineStackingStrategy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::LineStackingStrategy>(result__)
        }
    }
    pub fn SetLineStackingStrategy(
        &self,
        value: super::LineStackingStrategy,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn Margin(&self) -> ::windows::core::Result<super::Thickness> {
        let this = self;
        unsafe {
            let mut result__: super::Thickness = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Thickness>(result__)
        }
    }
    pub fn SetMargin<'a, Param0: ::windows::core::IntoParam<'a, super::Thickness>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn TextAlignmentProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IBlockStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn HorizontalTextAlignmentProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IBlockStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn LineHeightProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IBlockStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn LineStackingStrategyProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IBlockStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn MarginProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IBlockStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
    pub fn ClearValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ReadLocalValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn FontSize(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows::core::Result<super::Media::FontFamily> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFontFamily<'a, Param0: ::windows::core::IntoParam<'a, super::Media::FontFamily>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FontWeight(&self) -> ::windows::core::Result<::windows::UI::Text::FontWeight> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontWeight = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        }
    }
    pub fn SetFontWeight<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::UI::Text::FontWeight>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FontStyle(&self) -> ::windows::core::Result<::windows::UI::Text::FontStyle> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStyle = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStyle>(result__)
        }
    }
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn FontStretch(&self) -> ::windows::core::Result<::windows::UI::Text::FontStretch> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStretch = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStretch>(result__)
        }
    }
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn CharacterSpacing(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Media"
    ))]
    pub fn Foreground(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Media"
    ))]
    pub fn SetForeground<'a, Param0: ::windows::core::IntoParam<'a, super::Media::Brush>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).20)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).22)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn TextDecorations(&self) -> ::windows::core::Result<::windows::UI::Text::TextDecorations> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::TextDecorations = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::TextDecorations>(result__)
        }
    }
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
    pub fn ContentStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
    pub fn ContentEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
    pub fn ElementStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
    pub fn ElementEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).31)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn AccessKey(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).33)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetAccessKey<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).34)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).35)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).36)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).37)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).38)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn AccessKeyScopeOwner(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).39)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetAccessKeyScopeOwner<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).40)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::core::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::Input::KeyTipPlacementMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).41)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).42)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).43)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).44)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).45)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).46)(::core::mem::transmute_copy(this), value)
                .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn XamlRoot(&self) -> ::windows::core::Result<super::XamlRoot> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).47)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::XamlRoot>(result__)
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn SetXamlRoot<'a, Param0: ::windows::core::IntoParam<'a, super::XamlRoot>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).48)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).49)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyDisplayRequested<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).50)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).51)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).52)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).53)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyInvoked<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).54)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FindName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).55)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn IBlockStatics<R, F: FnOnce(&IBlockStatics) -> ::windows::core::Result<R>>(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Block, IBlockStatics> =
            ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for Block {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.Block;{8149d507-672f-5fd5-a10a-351389ba9659})",
    );
}
unsafe impl ::windows::core::Interface for Block {
    type Vtable = IBlock_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x8149d507_672f_5fd5_a10a_351389ba9659);
}
impl ::windows::core::RuntimeName for Block {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Block";
}
impl ::core::convert::From<Block> for ::windows::core::IUnknown {
    fn from(value: Block) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Block> for ::windows::core::IUnknown {
    fn from(value: &Block) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Block {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Block {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Block> for ::windows::core::IInspectable {
    fn from(value: Block) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Block> for ::windows::core::IInspectable {
    fn from(value: &Block) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Block {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Block {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<Block> for TextElement {
    fn from(value: Block) -> Self {
        ::core::convert::Into::<TextElement>::into(&value)
    }
}
impl ::core::convert::From<&Block> for TextElement {
    fn from(value: &Block) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for Block {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<TextElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for &Block {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<TextElement>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<Block> for super::DependencyObject {
    fn from(value: Block) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&Block> for super::DependencyObject {
    fn from(value: &Block) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for Block {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &Block {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for Block {}
unsafe impl ::core::marker::Sync for Block {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct BlockCollection(pub ::windows::core::IInspectable);
impl BlockCollection {
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<Block> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                index,
                &mut result__,
            )
            .from_abi::<Block>(result__)
        }
    }
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetView(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Collections::IVectorView<Block>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IVectorView<Block>>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, Block>>(
        &self,
        value: Param0,
        index: &mut u32,
    ) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
                index,
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, Block>>(
        &self,
        index: u32,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                index,
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, Block>>(
        &self,
        index: u32,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                index,
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), index)
                .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn Append<'a, Param0: ::windows::core::IntoParam<'a, Block>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetMany(
        &self,
        startindex: u32,
        items: &mut [<Block as ::windows::core::DefaultType>::DefaultType],
    ) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(
                ::core::mem::transmute_copy(this),
                startindex,
                items.len() as u32,
                ::core::mem::transmute_copy(&items),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ReplaceAll(
        &self,
        items: &[<Block as ::windows::core::DefaultType>::DefaultType],
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).17)(
                ::core::mem::transmute_copy(this),
                items.len() as u32,
                ::core::mem::transmute(items.as_ptr()),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn First(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Collections::IIterator<Block>> {
        let this = &::windows::core::Interface::cast::<
            ::windows::Foundation::Collections::IIterable<Block>,
        >(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IIterator<Block>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for BlockCollection {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Documents.BlockCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Microsoft.UI.Xaml.Documents.Block;{8149d507-672f-5fd5-a10a-351389ba9659})))" ) ;
}
unsafe impl ::windows::core::Interface for BlockCollection {
    type Vtable = ::windows::Foundation::Collections::IVector_abi<Block>;
    const IID : :: windows :: core :: GUID = :: windows :: core :: GUID :: from_signature ( < ::windows::Foundation::Collections:: IVector :: < Block > as :: windows :: core :: RuntimeType > :: SIGNATURE ) ;
}
impl ::windows::core::RuntimeName for BlockCollection {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.BlockCollection";
}
impl ::core::convert::From<BlockCollection> for ::windows::core::IUnknown {
    fn from(value: BlockCollection) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BlockCollection> for ::windows::core::IUnknown {
    fn from(value: &BlockCollection) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BlockCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BlockCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BlockCollection> for ::windows::core::IInspectable {
    fn from(value: BlockCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BlockCollection> for ::windows::core::IInspectable {
    fn from(value: &BlockCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BlockCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BlockCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<BlockCollection> for ::windows::Foundation::Collections::IVector<Block> {
    fn from(value: BlockCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BlockCollection>
    for ::windows::Foundation::Collections::IVector<Block>
{
    fn from(value: &BlockCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::Collections::IVector<Block>>
    for BlockCollection
{
    fn into_param(
        self,
    ) -> ::windows::core::Param<'a, ::windows::Foundation::Collections::IVector<Block>> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::Collections::IVector<Block>>
    for &BlockCollection
{
    fn into_param(
        self,
    ) -> ::windows::core::Param<'a, ::windows::Foundation::Collections::IVector<Block>> {
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
unsafe impl ::core::marker::Send for BlockCollection {}
unsafe impl ::core::marker::Sync for BlockCollection {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for BlockCollection {
    type Item = Block;
    type IntoIter = ::windows::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for &BlockCollection {
    type Item = Block;
    type IntoIter = ::windows::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::windows::Foundation::Collections::VectorIterator::new(
            ::core::convert::TryInto::try_into(self).ok(),
        )
    }
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct Bold(pub ::windows::core::IInspectable);
impl Bold {
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
            Bold,
            ::windows::core::IActivationFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
    pub fn ClearValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ReadLocalValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn Inlines(&self) -> ::windows::core::Result<InlineCollection> {
        let this = &::windows::core::Interface::cast::<ISpan>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InlineCollection>(result__)
        }
    }
    pub fn SetInlines<'a, Param0: ::windows::core::IntoParam<'a, InlineCollection>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISpan>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn FontSize(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows::core::Result<super::Media::FontFamily> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFontFamily<'a, Param0: ::windows::core::IntoParam<'a, super::Media::FontFamily>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FontWeight(&self) -> ::windows::core::Result<::windows::UI::Text::FontWeight> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontWeight = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        }
    }
    pub fn SetFontWeight<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::UI::Text::FontWeight>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FontStyle(&self) -> ::windows::core::Result<::windows::UI::Text::FontStyle> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStyle = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStyle>(result__)
        }
    }
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn FontStretch(&self) -> ::windows::core::Result<::windows::UI::Text::FontStretch> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStretch = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStretch>(result__)
        }
    }
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn CharacterSpacing(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Media"
    ))]
    pub fn Foreground(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Media"
    ))]
    pub fn SetForeground<'a, Param0: ::windows::core::IntoParam<'a, super::Media::Brush>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).20)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).22)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn TextDecorations(&self) -> ::windows::core::Result<::windows::UI::Text::TextDecorations> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::TextDecorations = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::TextDecorations>(result__)
        }
    }
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
    pub fn ContentStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
    pub fn ContentEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
    pub fn ElementStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
    pub fn ElementEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).31)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn AccessKey(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).33)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetAccessKey<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).34)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).35)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).36)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).37)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).38)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn AccessKeyScopeOwner(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).39)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetAccessKeyScopeOwner<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).40)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::core::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::Input::KeyTipPlacementMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).41)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).42)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).43)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).44)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).45)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).46)(::core::mem::transmute_copy(this), value)
                .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn XamlRoot(&self) -> ::windows::core::Result<super::XamlRoot> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).47)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::XamlRoot>(result__)
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn SetXamlRoot<'a, Param0: ::windows::core::IntoParam<'a, super::XamlRoot>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).48)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).49)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyDisplayRequested<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).50)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).51)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).52)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).53)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyInvoked<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).54)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FindName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).55)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for Bold {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.Bold;{241a5f5a-c164-597f-b0db-fac7431297f2})",
    );
}
unsafe impl ::windows::core::Interface for Bold {
    type Vtable = IBold_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x241a5f5a_c164_597f_b0db_fac7431297f2);
}
impl ::windows::core::RuntimeName for Bold {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Bold";
}
impl ::core::convert::From<Bold> for ::windows::core::IUnknown {
    fn from(value: Bold) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Bold> for ::windows::core::IUnknown {
    fn from(value: &Bold) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Bold {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Bold {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Bold> for ::windows::core::IInspectable {
    fn from(value: Bold) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Bold> for ::windows::core::IInspectable {
    fn from(value: &Bold) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Bold {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Bold {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<Bold> for Span {
    fn from(value: Bold) -> Self {
        ::core::convert::Into::<Span>::into(&value)
    }
}
impl ::core::convert::From<&Bold> for Span {
    fn from(value: &Bold) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Span> for Bold {
    fn into_param(self) -> ::windows::core::Param<'a, Span> {
        ::windows::core::Param::Owned(::core::convert::Into::<Span>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Span> for &Bold {
    fn into_param(self) -> ::windows::core::Param<'a, Span> {
        ::windows::core::Param::Owned(::core::convert::Into::<Span>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<Bold> for Inline {
    fn from(value: Bold) -> Self {
        ::core::convert::Into::<Inline>::into(&value)
    }
}
impl ::core::convert::From<&Bold> for Inline {
    fn from(value: &Bold) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Inline> for Bold {
    fn into_param(self) -> ::windows::core::Param<'a, Inline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Inline>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Inline> for &Bold {
    fn into_param(self) -> ::windows::core::Param<'a, Inline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Inline>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<Bold> for TextElement {
    fn from(value: Bold) -> Self {
        ::core::convert::Into::<TextElement>::into(&value)
    }
}
impl ::core::convert::From<&Bold> for TextElement {
    fn from(value: &Bold) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for Bold {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<TextElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for &Bold {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<TextElement>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<Bold> for super::DependencyObject {
    fn from(value: Bold) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&Bold> for super::DependencyObject {
    fn from(value: &Bold) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for Bold {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &Bold {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for Bold {}
unsafe impl ::core::marker::Sync for Bold {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct Glyphs(pub ::windows::core::IInspectable);
impl Glyphs {
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
            Glyphs,
            ::windows::core::IActivationFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn UnicodeString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetUnicodeString<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
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
    pub fn Indices(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetIndices<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
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
    pub fn FontUri(&self) -> ::windows::core::Result<::windows::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Uri>(result__)
        }
    }
    pub fn SetFontUri<'a, Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Uri>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn StyleSimulations(&self) -> ::windows::core::Result<super::Media::StyleSimulations> {
        let this = self;
        unsafe {
            let mut result__: super::Media::StyleSimulations = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::StyleSimulations>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetStyleSimulations(
        &self,
        value: super::Media::StyleSimulations,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn FontRenderingEmSize(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetFontRenderingEmSize(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn OriginX(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetOriginX(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn OriginY(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetOriginY(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Media"
    ))]
    pub fn Fill(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Media"
    ))]
    pub fn SetFill<'a, Param0: ::windows::core::IntoParam<'a, super::Media::Brush>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).21)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn IsColorFontEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsColorFontEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn ColorFontPaletteIndex(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetColorFontPaletteIndex(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn UnicodeStringProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn IndicesProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn FontUriProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn StyleSimulationsProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn FontRenderingEmSizeProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn OriginXProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn OriginYProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn FillProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn IsColorFontEnabledProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ColorFontPaletteIndexProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(all(feature = "UI_Composition", feature = "UI_Dispatching"))]
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
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                propertyinfo.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Composition")]
    pub fn GetVisualInternal(&self) -> ::windows::core::Result<super::super::Composition::Visual> {
        let this =
            &::windows::core::Interface::cast::<super::super::Composition::IVisualElement2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Composition::Visual>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
    pub fn ClearValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ReadLocalValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn Triggers(&self) -> ::windows::core::Result<super::TriggerCollection> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::TriggerCollection>(result__)
        }
    }
    pub fn Resources(&self) -> ::windows::core::Result<super::ResourceDictionary> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ResourceDictionary>(result__)
        }
    }
    pub fn SetResources<'a, Param0: ::windows::core::IntoParam<'a, super::ResourceDictionary>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Tag(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn SetTag<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ActualWidth(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn ActualHeight(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn Width(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetWidth(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn Height(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetHeight(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn MinWidth(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetMinWidth(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn MaxWidth(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetMaxWidth(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn MinHeight(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetMinHeight(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn MaxHeight(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetMaxHeight(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn HorizontalAlignment(&self) -> ::windows::core::Result<super::HorizontalAlignment> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: super::HorizontalAlignment = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::HorizontalAlignment>(result__)
        }
    }
    pub fn SetHorizontalAlignment(
        &self,
        value: super::HorizontalAlignment,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn VerticalAlignment(&self) -> ::windows::core::Result<super::VerticalAlignment> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: super::VerticalAlignment = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::VerticalAlignment>(result__)
        }
    }
    pub fn SetVerticalAlignment(
        &self,
        value: super::VerticalAlignment,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn Margin(&self) -> ::windows::core::Result<super::Thickness> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: super::Thickness = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).31)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Thickness>(result__)
        }
    }
    pub fn SetMargin<'a, Param0: ::windows::core::IntoParam<'a, super::Thickness>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).32)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).33)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).34)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn BaseUri(&self) -> ::windows::core::Result<::windows::Foundation::Uri> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).35)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Uri>(result__)
        }
    }
    pub fn DataContext(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).36)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn SetDataContext<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).37)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).38)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).39)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn FocusVisualMargin(&self) -> ::windows::core::Result<super::Thickness> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: super::Thickness = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).40)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Thickness>(result__)
        }
    }
    pub fn SetFocusVisualMargin<'a, Param0: ::windows::core::IntoParam<'a, super::Thickness>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).41)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FocusVisualSecondaryThickness(&self) -> ::windows::core::Result<super::Thickness> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: super::Thickness = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).42)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Thickness>(result__)
        }
    }
    pub fn SetFocusVisualSecondaryThickness<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::Thickness>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).43)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FocusVisualPrimaryThickness(&self) -> ::windows::core::Result<super::Thickness> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: super::Thickness = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).44)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Thickness>(result__)
        }
    }
    pub fn SetFocusVisualPrimaryThickness<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::Thickness>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).45)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Media"
    ))]
    pub fn FocusVisualSecondaryBrush(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).46)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Media"
    ))]
    pub fn SetFocusVisualSecondaryBrush<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::Media::Brush>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).47)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Media"
    ))]
    pub fn FocusVisualPrimaryBrush(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).48)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Media"
    ))]
    pub fn SetFocusVisualPrimaryBrush<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::Media::Brush>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).49)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn AllowFocusWhenDisabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).50)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowFocusWhenDisabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).51)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn Style(&self) -> ::windows::core::Result<super::Style> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).52)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Style>(result__)
        }
    }
    pub fn SetStyle<'a, Param0: ::windows::core::IntoParam<'a, super::Style>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).53)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn Parent(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).54)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn FlowDirection(&self) -> ::windows::core::Result<super::FlowDirection> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: super::FlowDirection = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).55)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::FlowDirection>(result__)
        }
    }
    pub fn SetFlowDirection(&self, value: super::FlowDirection) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).56)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn RequestedTheme(&self) -> ::windows::core::Result<super::ElementTheme> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: super::ElementTheme = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).57)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ElementTheme>(result__)
        }
    }
    pub fn SetRequestedTheme(&self, value: super::ElementTheme) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).58)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn IsLoaded(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).59)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn ActualTheme(&self) -> ::windows::core::Result<super::ElementTheme> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: super::ElementTheme = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).60)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ElementTheme>(result__)
        }
    }
    pub fn Loaded<'a, Param0: ::windows::core::IntoParam<'a, super::RoutedEventHandler>>(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).61)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveLoaded<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).62)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Unloaded<'a, Param0: ::windows::core::IntoParam<'a, super::RoutedEventHandler>>(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).63)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveUnloaded<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).64)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
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
            (::windows::core::Interface::vtable(this).65)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveDataContextChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).66)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
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
            (::windows::core::Interface::vtable(this).67)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveSizeChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).68)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
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
            (::windows::core::Interface::vtable(this).69)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveLayoutUpdated<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).70)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
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
            (::windows::core::Interface::vtable(this).71)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveLoading<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).72)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
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
            (::windows::core::Interface::vtable(this).73)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveActualThemeChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).74)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
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
            (::windows::core::Interface::vtable(this).75)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveEffectiveViewportChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).76)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FindName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).77)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(all(feature = "UI_Dispatching", feature = "UI_Xaml_Data"))]
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
            (::windows::core::Interface::vtable(this).78)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                binding.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Dispatching", feature = "UI_Xaml_Data"))]
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
            (::windows::core::Interface::vtable(this).79)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::Data::BindingExpression>(result__)
        }
    }
    pub fn InvalidateViewport(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElementProtected>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok()
        }
    }
    pub fn DesiredSize(&self) -> ::windows::core::Result<::windows::Foundation::Size> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::Size = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Size>(result__)
        }
    }
    pub fn AllowDrop(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowDrop(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn Opacity(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetOpacity(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(all(feature = "UI_Dispatching", feature = "UI_Xaml_Media"))]
    pub fn Clip(&self) -> ::windows::core::Result<super::Media::RectangleGeometry> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::RectangleGeometry>(result__)
        }
    }
    #[cfg(all(feature = "UI_Dispatching", feature = "UI_Xaml_Media"))]
    pub fn SetClip<'a, Param0: ::windows::core::IntoParam<'a, super::Media::RectangleGeometry>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn RenderTransform(&self) -> ::windows::core::Result<super::Media::Transform> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Transform>(result__)
        }
    }
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
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Projection(&self) -> ::windows::core::Result<super::Media::Projection> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Projection>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetProjection<'a, Param0: ::windows::core::IntoParam<'a, super::Media::Projection>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).16)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media_Media3D")]
    pub fn Transform3D(&self) -> ::windows::core::Result<super::Media::Media3D::Transform3D> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Media3D::Transform3D>(result__)
        }
    }
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
            (::windows::core::Interface::vtable(this).18)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn RenderTransformOrigin(&self) -> ::windows::core::Result<::windows::Foundation::Point> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Point>(result__)
        }
    }
    pub fn SetRenderTransformOrigin<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Point>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).20)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn IsHitTestVisible(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsHitTestVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn Visibility(&self) -> ::windows::core::Result<super::Visibility> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::Visibility = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Visibility>(result__)
        }
    }
    pub fn SetVisibility(&self, value: super::Visibility) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn RenderSize(&self) -> ::windows::core::Result<::windows::Foundation::Size> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::Size = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Size>(result__)
        }
    }
    pub fn UseLayoutRounding(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetUseLayoutRounding(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media_Animation")]
    pub fn Transitions(
        &self,
    ) -> ::windows::core::Result<super::Media::Animation::TransitionCollection> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Animation::TransitionCollection>(result__)
        }
    }
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
            (::windows::core::Interface::vtable(this).29)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn CacheMode(&self) -> ::windows::core::Result<super::Media::CacheMode> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::CacheMode>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetCacheMode<'a, Param0: ::windows::core::IntoParam<'a, super::Media::CacheMode>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).31)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn IsTapEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).32)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsTapEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).33)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn IsDoubleTapEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).34)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsDoubleTapEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).35)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn CanDrag(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).36)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetCanDrag(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).37)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn IsRightTapEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).38)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsRightTapEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).39)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn IsHoldingEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).40)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsHoldingEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).41)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationMode(&self) -> ::windows::core::Result<super::Input::ManipulationModes> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::Input::ManipulationModes = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).42)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::ManipulationModes>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetManipulationMode(
        &self,
        value: super::Input::ManipulationModes,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).43)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(all(feature = "UI_Input", feature = "UI_Xaml_Input"))]
    pub fn PointerCaptures(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IVectorView<super::Input::Pointer>,
    > {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).44)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IVectorView<super::Input::Pointer>>(
                result__,
            )
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn ContextFlyout(
        &self,
    ) -> ::windows::core::Result<super::Controls::Primitives::FlyoutBase> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).45)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Controls::Primitives::FlyoutBase>(result__)
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn SetContextFlyout<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::Controls::Primitives::FlyoutBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).46)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn CompositeMode(&self) -> ::windows::core::Result<super::Media::ElementCompositeMode> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::Media::ElementCompositeMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).47)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::ElementCompositeMode>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetCompositeMode(
        &self,
        value: super::Media::ElementCompositeMode,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).48)(::core::mem::transmute_copy(this), value)
                .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn Lights(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Collections::IVector<super::Media::XamlLight>>
    {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).49)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IVector<super::Media::XamlLight>>(
                result__,
            )
        }
    }
    pub fn CanBeScrollAnchor(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).50)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetCanBeScrollAnchor(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).51)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).52)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).53)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).54)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).55)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn AccessKeyScopeOwner(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).56)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetAccessKeyScopeOwner<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).57)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn AccessKey(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).58)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetAccessKey<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).59)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::core::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::Input::KeyTipPlacementMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).60)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).61)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).62)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).63)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).64)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).65)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn KeyTipTarget(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).66)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetKeyTipTarget<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).67)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusKeyboardNavigation(
        &self,
    ) -> ::windows::core::Result<super::Input::XYFocusKeyboardNavigationMode> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::Input::XYFocusKeyboardNavigationMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).68)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::XYFocusKeyboardNavigationMode>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusKeyboardNavigation(
        &self,
        value: super::Input::XYFocusKeyboardNavigationMode,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).69)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusUpNavigationStrategy(
        &self,
    ) -> ::windows::core::Result<super::Input::XYFocusNavigationStrategy> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::Input::XYFocusNavigationStrategy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).70)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::XYFocusNavigationStrategy>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusUpNavigationStrategy(
        &self,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).71)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusDownNavigationStrategy(
        &self,
    ) -> ::windows::core::Result<super::Input::XYFocusNavigationStrategy> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::Input::XYFocusNavigationStrategy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).72)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::XYFocusNavigationStrategy>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusDownNavigationStrategy(
        &self,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).73)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusLeftNavigationStrategy(
        &self,
    ) -> ::windows::core::Result<super::Input::XYFocusNavigationStrategy> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::Input::XYFocusNavigationStrategy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).74)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::XYFocusNavigationStrategy>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusLeftNavigationStrategy(
        &self,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).75)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusRightNavigationStrategy(
        &self,
    ) -> ::windows::core::Result<super::Input::XYFocusNavigationStrategy> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::Input::XYFocusNavigationStrategy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).76)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::XYFocusNavigationStrategy>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusRightNavigationStrategy(
        &self,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).77)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(all(feature = "UI_Dispatching", feature = "UI_Xaml_Input"))]
    pub fn KeyboardAccelerators(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IVector<super::Input::KeyboardAccelerator>,
    > {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).78)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IVector<
                super::Input::KeyboardAccelerator,
            >>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn KeyboardAcceleratorPlacementTarget(
        &self,
    ) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).79)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetKeyboardAcceleratorPlacementTarget<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).80)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyboardAcceleratorPlacementMode(
        &self,
    ) -> ::windows::core::Result<super::Input::KeyboardAcceleratorPlacementMode> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::Input::KeyboardAcceleratorPlacementMode =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).81)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyboardAcceleratorPlacementMode>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyboardAcceleratorPlacementMode(
        &self,
        value: super::Input::KeyboardAcceleratorPlacementMode,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).82)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn HighContrastAdjustment(
        &self,
    ) -> ::windows::core::Result<super::ElementHighContrastAdjustment> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::ElementHighContrastAdjustment = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).83)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ElementHighContrastAdjustment>(result__)
        }
    }
    pub fn SetHighContrastAdjustment(
        &self,
        value: super::ElementHighContrastAdjustment,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).84)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn TabFocusNavigation(
        &self,
    ) -> ::windows::core::Result<super::Input::KeyboardNavigationMode> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::Input::KeyboardNavigationMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).85)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyboardNavigationMode>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetTabFocusNavigation(
        &self,
        value: super::Input::KeyboardNavigationMode,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).86)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn OpacityTransition(&self) -> ::windows::core::Result<super::ScalarTransition> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).87)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ScalarTransition>(result__)
        }
    }
    pub fn SetOpacityTransition<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::ScalarTransition>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).88)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Translation(&self) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector3> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).89)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    pub fn SetTranslation<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Numerics::Vector3>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).90)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn TranslationTransition(&self) -> ::windows::core::Result<super::Vector3Transition> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).91)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Vector3Transition>(result__)
        }
    }
    pub fn SetTranslationTransition<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::Vector3Transition>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).92)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Rotation(&self) -> ::windows::core::Result<f32> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).93)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    pub fn SetRotation(&self, value: f32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).94)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn RotationTransition(&self) -> ::windows::core::Result<super::ScalarTransition> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).95)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ScalarTransition>(result__)
        }
    }
    pub fn SetRotationTransition<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::ScalarTransition>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).96)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Scale(&self) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector3> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).97)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    pub fn SetScale<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Numerics::Vector3>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).98)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ScaleTransition(&self) -> ::windows::core::Result<super::Vector3Transition> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).99)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Vector3Transition>(result__)
        }
    }
    pub fn SetScaleTransition<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::Vector3Transition>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).100)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn TransformMatrix(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Numerics::Matrix4x4> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Matrix4x4 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).101)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Matrix4x4>(result__)
        }
    }
    pub fn SetTransformMatrix<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Numerics::Matrix4x4>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).102)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn CenterPoint(&self) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector3> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).103)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    pub fn SetCenterPoint<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Numerics::Vector3>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).104)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn RotationAxis(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector3> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).105)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    pub fn SetRotationAxis<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Numerics::Vector3>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).106)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ActualOffset(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector3> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).107)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    pub fn ActualSize(&self) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector2> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector2 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).108)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector2>(result__)
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn XamlRoot(&self) -> ::windows::core::Result<super::XamlRoot> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).109)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::XamlRoot>(result__)
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn SetXamlRoot<'a, Param0: ::windows::core::IntoParam<'a, super::XamlRoot>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).110)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Shadow(&self) -> ::windows::core::Result<super::Media::Shadow> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).111)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Shadow>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetShadow<'a, Param0: ::windows::core::IntoParam<'a, super::Media::Shadow>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).112)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn RasterizationScale(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).113)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetRasterizationScale(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).114)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn FocusState(&self) -> ::windows::core::Result<super::FocusState> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::FocusState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).115)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::FocusState>(result__)
        }
    }
    pub fn UseSystemFocusVisuals(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).116)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetUseSystemFocusVisuals(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).117)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn XYFocusLeft(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).118)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetXYFocusLeft<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).119)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn XYFocusRight(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).120)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetXYFocusRight<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).121)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn XYFocusUp(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).122)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetXYFocusUp<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).123)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn XYFocusDown(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).124)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetXYFocusDown<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).125)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn IsTabStop(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).126)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsTabStop(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).127)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn TabIndex(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).128)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetTabIndex(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).129)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyUp<'a, Param0: ::windows::core::IntoParam<'a, super::Input::KeyEventHandler>>(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).130)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveKeyUp<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).131)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyDown<'a, Param0: ::windows::core::IntoParam<'a, super::Input::KeyEventHandler>>(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).132)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveKeyDown<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).133)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn GotFocus<'a, Param0: ::windows::core::IntoParam<'a, super::RoutedEventHandler>>(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).134)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveGotFocus<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).135)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn LostFocus<'a, Param0: ::windows::core::IntoParam<'a, super::RoutedEventHandler>>(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).136)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveLostFocus<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).137)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).138)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveDragStarting<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).139)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).140)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveDropCompleted<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).141)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).142)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveCharacterReceived<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).143)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn DragEnter<'a, Param0: ::windows::core::IntoParam<'a, super::DragEventHandler>>(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).144)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveDragEnter<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).145)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn DragLeave<'a, Param0: ::windows::core::IntoParam<'a, super::DragEventHandler>>(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).146)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveDragLeave<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).147)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn DragOver<'a, Param0: ::windows::core::IntoParam<'a, super::DragEventHandler>>(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).148)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveDragOver<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).149)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn Drop<'a, Param0: ::windows::core::IntoParam<'a, super::DragEventHandler>>(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).150)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveDrop<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).151)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).152)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePointerPressed<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).153)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).154)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePointerMoved<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).155)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).156)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePointerReleased<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).157)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).158)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePointerEntered<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).159)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).160)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePointerExited<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).161)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).162)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePointerCaptureLost<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).163)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).164)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePointerCanceled<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).165)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).166)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePointerWheelChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).167)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn Tapped<'a, Param0: ::windows::core::IntoParam<'a, super::Input::TappedEventHandler>>(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).168)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveTapped<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).169)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).170)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveDoubleTapped<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).171)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).172)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveHolding<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).173)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).174)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveContextRequested<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).175)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).176)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveContextCanceled<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).177)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).178)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveRightTapped<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).179)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).180)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveManipulationStarting<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).181)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).182)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveManipulationInertiaStarting<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).183)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).184)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveManipulationStarted<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).185)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).186)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveManipulationDelta<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).187)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).188)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveManipulationCompleted<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).189)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).190)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyDisplayRequested<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).191)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).192)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).193)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).194)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyInvoked<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).195)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).196)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveProcessKeyboardAccelerators<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).197)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).198)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveGettingFocus<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).199)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).200)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveLosingFocus<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).201)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).202)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveNoFocusCandidateFound<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).203)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
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
            (::windows::core::Interface::vtable(this).204)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePreviewKeyDown<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).205)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
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
            (::windows::core::Interface::vtable(this).206)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePreviewKeyUp<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).207)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).208)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveBringIntoViewRequested<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).209)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Measure<'a, Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Size>>(
        &self,
        availablesize: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).210)(
                ::core::mem::transmute_copy(this),
                availablesize.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Arrange<'a, Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Rect>>(
        &self,
        finalrect: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).211)(
                ::core::mem::transmute_copy(this),
                finalrect.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Input", feature = "UI_Xaml_Input"))]
    pub fn CapturePointer<'a, Param0: ::windows::core::IntoParam<'a, super::Input::Pointer>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).212)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "UI_Input", feature = "UI_Xaml_Input"))]
    pub fn ReleasePointerCapture<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::Input::Pointer>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).213)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ReleasePointerCaptures(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).214)(::core::mem::transmute_copy(this)).ok()
        }
    }
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
            (::windows::core::Interface::vtable(this).215)(
                ::core::mem::transmute_copy(this),
                routedevent.into_param().abi(),
                handler.into_param().abi(),
                handledeventstoo,
            )
            .ok()
        }
    }
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
            (::windows::core::Interface::vtable(this).216)(
                ::core::mem::transmute_copy(this),
                routedevent.into_param().abi(),
                handler.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn TransformToVisual<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(
        &self,
        visual: Param0,
    ) -> ::windows::core::Result<super::Media::GeneralTransform> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).217)(
                ::core::mem::transmute_copy(this),
                visual.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::Media::GeneralTransform>(result__)
        }
    }
    pub fn InvalidateMeasure(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).218)(::core::mem::transmute_copy(this)).ok()
        }
    }
    pub fn InvalidateArrange(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).219)(::core::mem::transmute_copy(this)).ok()
        }
    }
    pub fn UpdateLayout(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).220)(::core::mem::transmute_copy(this)).ok()
        }
    }
    pub fn CancelDirectManipulations(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).221)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
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
            (::windows::core::Interface::vtable(this).222)(
                ::core::mem::transmute_copy(this),
                pointerpoint.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IAsyncOperation<
                ::windows::ApplicationModel::DataTransfer::DataPackageOperation,
            >>(result__)
        }
    }
    pub fn StartBringIntoView(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).223)(::core::mem::transmute_copy(this)).ok()
        }
    }
    pub fn StartBringIntoViewWithOptions<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::BringIntoViewOptions>,
    >(
        &self,
        options: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).224)(
                ::core::mem::transmute_copy(this),
                options.into_param().abi(),
            )
            .ok()
        }
    }
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
            (::windows::core::Interface::vtable(this).225)(
                ::core::mem::transmute_copy(this),
                args.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Focus(&self, value: super::FocusState) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).226)(
                ::core::mem::transmute_copy(this),
                value,
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
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
            (::windows::core::Interface::vtable(this).227)(
                ::core::mem::transmute_copy(this),
                animation.into_param().abi(),
            )
            .ok()
        }
    }
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
            (::windows::core::Interface::vtable(this).228)(
                ::core::mem::transmute_copy(this),
                animation.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Input")]
    pub fn ProtectedCursor(&self) -> ::windows::core::Result<super::super::Input::InputCursor> {
        let this = &::windows::core::Interface::cast::<super::IUIElementProtected>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Input::InputCursor>(result__)
        }
    }
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
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn IGlyphsStatics<R, F: FnOnce(&IGlyphsStatics) -> ::windows::core::Result<R>>(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Glyphs, IGlyphsStatics> =
            ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for Glyphs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.Glyphs;{0fbf8cfe-18e7-5e45-9fa3-d2d0927958f4})",
    );
}
unsafe impl ::windows::core::Interface for Glyphs {
    type Vtable = IGlyphs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x0fbf8cfe_18e7_5e45_9fa3_d2d0927958f4);
}
impl ::windows::core::RuntimeName for Glyphs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Glyphs";
}
impl ::core::convert::From<Glyphs> for ::windows::core::IUnknown {
    fn from(value: Glyphs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Glyphs> for ::windows::core::IUnknown {
    fn from(value: &Glyphs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Glyphs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Glyphs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Glyphs> for ::windows::core::IInspectable {
    fn from(value: Glyphs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Glyphs> for ::windows::core::IInspectable {
    fn from(value: &Glyphs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Glyphs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Glyphs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
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
        ::core::convert::Into::<super::FrameworkElement>::into(&value)
    }
}
impl ::core::convert::From<&Glyphs> for super::FrameworkElement {
    fn from(value: &Glyphs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::FrameworkElement> for Glyphs {
    fn into_param(self) -> ::windows::core::Param<'a, super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::FrameworkElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::FrameworkElement> for &Glyphs {
    fn into_param(self) -> ::windows::core::Param<'a, super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::FrameworkElement>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<Glyphs> for super::UIElement {
    fn from(value: Glyphs) -> Self {
        ::core::convert::Into::<super::UIElement>::into(&value)
    }
}
impl ::core::convert::From<&Glyphs> for super::UIElement {
    fn from(value: &Glyphs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::UIElement> for Glyphs {
    fn into_param(self) -> ::windows::core::Param<'a, super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::UIElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::UIElement> for &Glyphs {
    fn into_param(self) -> ::windows::core::Param<'a, super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::UIElement>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<Glyphs> for super::DependencyObject {
    fn from(value: Glyphs) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&Glyphs> for super::DependencyObject {
    fn from(value: &Glyphs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for Glyphs {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &Glyphs {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for Glyphs {}
unsafe impl ::core::marker::Sync for Glyphs {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct Hyperlink(pub ::windows::core::IInspectable);
impl Hyperlink {
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
            Hyperlink,
            ::windows::core::IActivationFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn NavigateUri(&self) -> ::windows::core::Result<::windows::Foundation::Uri> {
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
    pub fn SetNavigateUri<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Uri>,
    >(
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
    pub fn UnderlineStyle(&self) -> ::windows::core::Result<UnderlineStyle> {
        let this = self;
        unsafe {
            let mut result__: UnderlineStyle = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<UnderlineStyle>(result__)
        }
    }
    pub fn SetUnderlineStyle(&self, value: UnderlineStyle) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn XYFocusLeft(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetXYFocusLeft<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn XYFocusRight(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetXYFocusRight<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn XYFocusUp(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetXYFocusUp<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn XYFocusDown(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetXYFocusDown<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).17)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ElementSoundMode(&self) -> ::windows::core::Result<super::ElementSoundMode> {
        let this = self;
        unsafe {
            let mut result__: super::ElementSoundMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ElementSoundMode>(result__)
        }
    }
    pub fn SetElementSoundMode(
        &self,
        value: super::ElementSoundMode,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn FocusState(&self) -> ::windows::core::Result<super::FocusState> {
        let this = self;
        unsafe {
            let mut result__: super::FocusState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::FocusState>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusUpNavigationStrategy(
        &self,
    ) -> ::windows::core::Result<super::Input::XYFocusNavigationStrategy> {
        let this = self;
        unsafe {
            let mut result__: super::Input::XYFocusNavigationStrategy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::XYFocusNavigationStrategy>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusUpNavigationStrategy(
        &self,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusDownNavigationStrategy(
        &self,
    ) -> ::windows::core::Result<super::Input::XYFocusNavigationStrategy> {
        let this = self;
        unsafe {
            let mut result__: super::Input::XYFocusNavigationStrategy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::XYFocusNavigationStrategy>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusDownNavigationStrategy(
        &self,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusLeftNavigationStrategy(
        &self,
    ) -> ::windows::core::Result<super::Input::XYFocusNavigationStrategy> {
        let this = self;
        unsafe {
            let mut result__: super::Input::XYFocusNavigationStrategy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::XYFocusNavigationStrategy>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusLeftNavigationStrategy(
        &self,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusRightNavigationStrategy(
        &self,
    ) -> ::windows::core::Result<super::Input::XYFocusNavigationStrategy> {
        let this = self;
        unsafe {
            let mut result__: super::Input::XYFocusNavigationStrategy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::XYFocusNavigationStrategy>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusRightNavigationStrategy(
        &self,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn IsTabStop(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsTabStop(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn TabIndex(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).31)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetTabIndex(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(all(feature = "UI_Dispatching", feature = "UI_Xaml_Input"))]
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
            (::windows::core::Interface::vtable(this).33)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveClick<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).34)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn GotFocus<'a, Param0: ::windows::core::IntoParam<'a, super::RoutedEventHandler>>(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).35)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveGotFocus<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).36)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn LostFocus<'a, Param0: ::windows::core::IntoParam<'a, super::RoutedEventHandler>>(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).37)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveLostFocus<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).38)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Focus(&self, value: super::FocusState) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).39)(
                ::core::mem::transmute_copy(this),
                value,
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn NavigateUriProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn UnderlineStyleProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn XYFocusLeftProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn XYFocusRightProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn XYFocusUpProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn XYFocusDownProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ElementSoundModeProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn FocusStateProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn XYFocusUpNavigationStrategyProperty(
    ) -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn XYFocusDownNavigationStrategyProperty(
    ) -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn XYFocusLeftNavigationStrategyProperty(
    ) -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn XYFocusRightNavigationStrategyProperty(
    ) -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn IsTabStopProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn TabIndexProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
    pub fn ClearValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ReadLocalValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn Inlines(&self) -> ::windows::core::Result<InlineCollection> {
        let this = &::windows::core::Interface::cast::<ISpan>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InlineCollection>(result__)
        }
    }
    pub fn SetInlines<'a, Param0: ::windows::core::IntoParam<'a, InlineCollection>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISpan>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn FontSize(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows::core::Result<super::Media::FontFamily> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFontFamily<'a, Param0: ::windows::core::IntoParam<'a, super::Media::FontFamily>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FontWeight(&self) -> ::windows::core::Result<::windows::UI::Text::FontWeight> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontWeight = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        }
    }
    pub fn SetFontWeight<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::UI::Text::FontWeight>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FontStyle(&self) -> ::windows::core::Result<::windows::UI::Text::FontStyle> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStyle = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStyle>(result__)
        }
    }
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn FontStretch(&self) -> ::windows::core::Result<::windows::UI::Text::FontStretch> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStretch = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStretch>(result__)
        }
    }
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn CharacterSpacing(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Media"
    ))]
    pub fn Foreground(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Media"
    ))]
    pub fn SetForeground<'a, Param0: ::windows::core::IntoParam<'a, super::Media::Brush>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).20)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).22)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn TextDecorations(&self) -> ::windows::core::Result<::windows::UI::Text::TextDecorations> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::TextDecorations = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::TextDecorations>(result__)
        }
    }
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
    pub fn ContentStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
    pub fn ContentEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
    pub fn ElementStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
    pub fn ElementEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).31)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn AccessKey(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).33)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetAccessKey<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).34)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).35)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).36)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).37)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).38)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn AccessKeyScopeOwner(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).39)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetAccessKeyScopeOwner<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).40)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::core::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::Input::KeyTipPlacementMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).41)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).42)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).43)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).44)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).45)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).46)(::core::mem::transmute_copy(this), value)
                .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn XamlRoot(&self) -> ::windows::core::Result<super::XamlRoot> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).47)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::XamlRoot>(result__)
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn SetXamlRoot<'a, Param0: ::windows::core::IntoParam<'a, super::XamlRoot>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).48)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).49)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyDisplayRequested<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).50)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).51)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).52)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).53)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyInvoked<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).54)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FindName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).55)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn IHyperlinkStatics<R, F: FnOnce(&IHyperlinkStatics) -> ::windows::core::Result<R>>(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Hyperlink, IHyperlinkStatics> =
            ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for Hyperlink {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.Hyperlink;{ac09bd16-cdfa-54c2-8d03-a474181545b1})",
    );
}
unsafe impl ::windows::core::Interface for Hyperlink {
    type Vtable = IHyperlink_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xac09bd16_cdfa_54c2_8d03_a474181545b1);
}
impl ::windows::core::RuntimeName for Hyperlink {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Hyperlink";
}
impl ::core::convert::From<Hyperlink> for ::windows::core::IUnknown {
    fn from(value: Hyperlink) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Hyperlink> for ::windows::core::IUnknown {
    fn from(value: &Hyperlink) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Hyperlink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Hyperlink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Hyperlink> for ::windows::core::IInspectable {
    fn from(value: Hyperlink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Hyperlink> for ::windows::core::IInspectable {
    fn from(value: &Hyperlink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Hyperlink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Hyperlink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<Hyperlink> for Span {
    fn from(value: Hyperlink) -> Self {
        ::core::convert::Into::<Span>::into(&value)
    }
}
impl ::core::convert::From<&Hyperlink> for Span {
    fn from(value: &Hyperlink) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Span> for Hyperlink {
    fn into_param(self) -> ::windows::core::Param<'a, Span> {
        ::windows::core::Param::Owned(::core::convert::Into::<Span>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Span> for &Hyperlink {
    fn into_param(self) -> ::windows::core::Param<'a, Span> {
        ::windows::core::Param::Owned(::core::convert::Into::<Span>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<Hyperlink> for Inline {
    fn from(value: Hyperlink) -> Self {
        ::core::convert::Into::<Inline>::into(&value)
    }
}
impl ::core::convert::From<&Hyperlink> for Inline {
    fn from(value: &Hyperlink) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Inline> for Hyperlink {
    fn into_param(self) -> ::windows::core::Param<'a, Inline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Inline>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Inline> for &Hyperlink {
    fn into_param(self) -> ::windows::core::Param<'a, Inline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Inline>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<Hyperlink> for TextElement {
    fn from(value: Hyperlink) -> Self {
        ::core::convert::Into::<TextElement>::into(&value)
    }
}
impl ::core::convert::From<&Hyperlink> for TextElement {
    fn from(value: &Hyperlink) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for Hyperlink {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<TextElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for &Hyperlink {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<TextElement>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<Hyperlink> for super::DependencyObject {
    fn from(value: Hyperlink) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&Hyperlink> for super::DependencyObject {
    fn from(value: &Hyperlink) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for Hyperlink {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &Hyperlink {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for Hyperlink {}
unsafe impl ::core::marker::Sync for Hyperlink {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct HyperlinkClickEventArgs(pub ::windows::core::IInspectable);
impl HyperlinkClickEventArgs {
    pub fn OriginalSource(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for HyperlinkClickEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Documents.HyperlinkClickEventArgs;{f8f89552-873d-5ef5-82bf-c79a9509b07c})" ) ;
}
unsafe impl ::windows::core::Interface for HyperlinkClickEventArgs {
    type Vtable = IHyperlinkClickEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xf8f89552_873d_5ef5_82bf_c79a9509b07c);
}
impl ::windows::core::RuntimeName for HyperlinkClickEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.HyperlinkClickEventArgs";
}
impl ::core::convert::From<HyperlinkClickEventArgs> for ::windows::core::IUnknown {
    fn from(value: HyperlinkClickEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&HyperlinkClickEventArgs> for ::windows::core::IUnknown {
    fn from(value: &HyperlinkClickEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HyperlinkClickEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a HyperlinkClickEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<HyperlinkClickEventArgs> for ::windows::core::IInspectable {
    fn from(value: HyperlinkClickEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&HyperlinkClickEventArgs> for ::windows::core::IInspectable {
    fn from(value: &HyperlinkClickEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HyperlinkClickEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a HyperlinkClickEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<HyperlinkClickEventArgs> for super::RoutedEventArgs {
    fn from(value: HyperlinkClickEventArgs) -> Self {
        ::core::convert::Into::<super::RoutedEventArgs>::into(&value)
    }
}
impl ::core::convert::From<&HyperlinkClickEventArgs> for super::RoutedEventArgs {
    fn from(value: &HyperlinkClickEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for HyperlinkClickEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for &HyperlinkClickEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for HyperlinkClickEventArgs {}
unsafe impl ::core::marker::Sync for HyperlinkClickEventArgs {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IBlock(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBlock {
    type Vtable = IBlock_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x8149d507_672f_5fd5_a10a_351389ba9659);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBlock_abi(
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
        result__: *mut super::TextAlignment,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: super::TextAlignment,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut super::TextAlignment,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: super::TextAlignment,
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
        result__: *mut super::LineStackingStrategy,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: super::LineStackingStrategy,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut super::Thickness,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: super::Thickness,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBlockFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBlockFactory {
    type Vtable = IBlockFactory_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x21bd671c_33e2_56ef_be37_a128e898452c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBlockFactory_abi(
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
pub struct IBlockStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBlockStatics {
    type Vtable = IBlockStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x830feedf_9aa6_56cd_983e_055500171b45);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBlockStatics_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBold(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBold {
    type Vtable = IBold_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x241a5f5a_c164_597f_b0db_fac7431297f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBold_abi(
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
pub struct IGlyphs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGlyphs {
    type Vtable = IGlyphs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x0fbf8cfe_18e7_5e45_9fa3_d2d0927958f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlyphs_abi(
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
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut super::Media::StyleSimulations,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: super::Media::StyleSimulations,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
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
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut f64,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: f64,
    ) -> ::windows::core::HRESULT,
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Media"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Media"
    )))]
    usize,
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Media"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Media"
    )))]
    usize,
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
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: i32,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGlyphsStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGlyphsStatics {
    type Vtable = IGlyphsStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x8d9e241a_3e0e_5100_8ede_e008034ff8ae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlyphsStatics_abi(
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
pub struct IHyperlink(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IHyperlink {
    type Vtable = IHyperlink_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xac09bd16_cdfa_54c2_8d03_a474181545b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHyperlink_abi(
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
        result__: *mut UnderlineStyle,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: UnderlineStyle,
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
        value: ::windows::core::RawPtr,
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
        value: ::windows::core::RawPtr,
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
        value: ::windows::core::RawPtr,
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
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut super::ElementSoundMode,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: super::ElementSoundMode,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut super::FocusState,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))] usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))] usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))] usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))] usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))] usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))] usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))] usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))] usize,
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
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: i32,
    ) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "UI_Dispatching", feature = "UI_Xaml_Input"))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "UI_Dispatching", feature = "UI_Xaml_Input")))] usize,
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
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: super::FocusState,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHyperlinkClickEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IHyperlinkClickEventArgs {
    type Vtable = IHyperlinkClickEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xf8f89552_873d_5ef5_82bf_c79a9509b07c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHyperlinkClickEventArgs_abi(
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
pub struct IHyperlinkStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IHyperlinkStatics {
    type Vtable = IHyperlinkStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xe13598f4_7bc7_5ab9_885b_70f32f8c9531);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHyperlinkStatics_abi(
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
pub struct IInline(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInline {
    type Vtable = IInline_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x813d427a_8980_5a79_a8fa_f27919cfb24f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInline_abi(
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
pub struct IInlineFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInlineFactory {
    type Vtable = IInlineFactory_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xfd253a36_fa2b_5b30_89a8_9f577871ec07);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInlineFactory_abi(
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
pub struct IInlineUIContainer(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInlineUIContainer {
    type Vtable = IInlineUIContainer_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xd529bef6_c05a_5bad_85e8_640127cf86f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInlineUIContainer_abi(
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::core::RawPtr,
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    )))]
    usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IItalic(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IItalic {
    type Vtable = IItalic_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xca3cbebd_7a8d_5d7a_8fdf_538e8a680f6c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IItalic_abi(
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
pub struct ILineBreak(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILineBreak {
    type Vtable = ILineBreak_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x09307599_7cc2_5f54_b106_728620c16f76);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineBreak_abi(
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
pub struct IParagraph(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IParagraph {
    type Vtable = IParagraph_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x9ed64c77_329d_502f_a257_f58398edab51);
}
#[repr(C)]
#[doc(hidden)]
pub struct IParagraph_abi(
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
        result__: *mut f64,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: f64,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IParagraphStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IParagraphStatics {
    type Vtable = IParagraphStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x4eb89ab1_66c8_5fc0_aa5f_48c8092ceb5f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IParagraphStatics_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRun(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRun {
    type Vtable = IRun_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x1f905239_37cb_590b_9132_3ffb7741906e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRun_abi(
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
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut super::FlowDirection,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: super::FlowDirection,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRunStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRunStatics {
    type Vtable = IRunStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x051b3c5b_7600_51a5_80c5_93eb50fd684f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRunStatics_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpan(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpan {
    type Vtable = ISpan_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x91b93d4d_4e28_57b9_bffb_3566c2a3c2a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpan_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpanFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpanFactory {
    type Vtable = ISpanFactory_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xa6e87c16_c175_55c8_bbd3_ce40f9d0a680);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpanFactory_abi(
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
pub struct ITextElement(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITextElement {
    type Vtable = ITextElement_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xa122ba22_833f_5220_a47e_6cd507531abe);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextElement_abi(
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
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut f64,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: f64,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::UI::Text::FontWeight,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::UI::Text::FontWeight,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::UI::Text::FontStyle,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::UI::Text::FontStretch,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: i32,
    ) -> ::windows::core::HRESULT,
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Media"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Media"
    )))]
    usize,
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Media"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Media"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
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
        result__: *mut ::windows::UI::Text::TextDecorations,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows::core::HRESULT,
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    )))]
    usize,
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    )))]
    usize,
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    )))]
    usize,
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    )))]
    usize,
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
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
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
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: bool,
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
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut super::Input::KeyTipPlacementMode,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))] usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))] usize,
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::core::RawPtr,
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        token: ::windows::Foundation::EventRegistrationToken,
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        token: ::windows::Foundation::EventRegistrationToken,
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITextElementFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITextElementFactory {
    type Vtable = ITextElementFactory_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xdf51fb95_a5e6_5b16_8e88_9f7cbfa234b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextElementFactory_abi(
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
pub struct ITextElementOverrides(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITextElementOverrides {
    type Vtable = ITextElementOverrides_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x41b01380_e49f_5fda_8c72_acc1ac1e91df);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextElementOverrides_abi(
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
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITextElementStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITextElementStatics {
    type Vtable = ITextElementStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xc9b55919_e1fe_5acd_bac7_c9d7f413b35c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextElementStatics_abi(
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
pub struct ITextHighlighter(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITextHighlighter {
    type Vtable = ITextHighlighter_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xb756e861_1d2b_5f6f_81fd_c51a5bc068ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextHighlighter_abi(
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
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Media"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Media"
    )))]
    usize,
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Media"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Media"
    )))]
    usize,
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Media"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Media"
    )))]
    usize,
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Media"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Media"
    )))]
    usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITextHighlighterBase(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITextHighlighterBase {
    type Vtable = ITextHighlighterBase_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x5c21aaf0_3a17_5468_8aac_be14db0ed8c1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextHighlighterBase_abi(
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
pub struct ITextHighlighterBaseFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITextHighlighterBaseFactory {
    type Vtable = ITextHighlighterBaseFactory_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xe091e461_53ab_599e_aaea_800adc72da4f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextHighlighterBaseFactory_abi(
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
pub struct ITextHighlighterFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITextHighlighterFactory {
    type Vtable = ITextHighlighterFactory_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x69c7311f_c019_5b93_b511_81418543bab7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextHighlighterFactory_abi(
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
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Media"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        baseinterface: ::windows::core::RawPtr,
        innerinterface: *mut ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Media"
    )))]
    usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITextHighlighterStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITextHighlighterStatics {
    type Vtable = ITextHighlighterStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x4975047a_87ad_51a2_977c_e771de4f4035);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextHighlighterStatics_abi(
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
pub struct ITextPointer(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITextPointer {
    type Vtable = ITextPointer_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x842eb385_ee41_5930_979b_438fa7525a51);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextPointer_abi(
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
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut LogicalDirection,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        direction: LogicalDirection,
        result__: *mut ::windows::Foundation::Rect,
    ) -> ::windows::core::HRESULT,
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        offset: i32,
        direction: LogicalDirection,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    )))]
    usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITypography(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITypography {
    type Vtable = ITypography_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xfa27e2e3_be5e_5d21_9a5e_90cf102af828);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypography_abi(
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
pub struct ITypographyStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITypographyStatics {
    type Vtable = ITypographyStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x55fe4535_2125_533a_ada8_27be2b9e1193);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypographyStatics_abi(
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
        element: ::windows::core::RawPtr,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: i32,
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
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: bool,
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
        element: ::windows::core::RawPtr,
        result__: *mut super::FontEastAsianLanguage,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: super::FontEastAsianLanguage,
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
        element: ::windows::core::RawPtr,
        result__: *mut super::FontEastAsianWidths,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: super::FontEastAsianWidths,
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
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: bool,
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
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: bool,
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
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: bool,
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
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: bool,
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
        element: ::windows::core::RawPtr,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: i32,
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
        element: ::windows::core::RawPtr,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: i32,
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
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: bool,
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
        element: ::windows::core::RawPtr,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: i32,
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
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: bool,
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
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: bool,
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
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: bool,
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
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: bool,
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
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: bool,
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
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: bool,
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
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: bool,
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
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: bool,
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
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: bool,
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
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: bool,
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
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: bool,
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
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: bool,
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
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: bool,
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
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: bool,
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
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: bool,
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
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: bool,
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
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: bool,
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
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: bool,
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
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: bool,
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
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: bool,
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
        element: ::windows::core::RawPtr,
        result__: *mut super::FontCapitals,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: super::FontCapitals,
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
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: bool,
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
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: bool,
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
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: bool,
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
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: bool,
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
        element: ::windows::core::RawPtr,
        result__: *mut super::FontFraction,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: super::FontFraction,
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
        element: ::windows::core::RawPtr,
        result__: *mut super::FontNumeralStyle,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: super::FontNumeralStyle,
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
        element: ::windows::core::RawPtr,
        result__: *mut super::FontNumeralAlignment,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: super::FontNumeralAlignment,
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
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: bool,
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
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: bool,
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
        element: ::windows::core::RawPtr,
        result__: *mut super::FontVariants,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: super::FontVariants,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUnderline(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IUnderline {
    type Vtable = IUnderline_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x68aaec6e_ea71_5ed2_b83e_91684b25efb9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUnderline_abi(
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
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct Inline(pub ::windows::core::IInspectable);
impl Inline {
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
    pub fn ClearValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ReadLocalValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn FontSize(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows::core::Result<super::Media::FontFamily> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFontFamily<'a, Param0: ::windows::core::IntoParam<'a, super::Media::FontFamily>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FontWeight(&self) -> ::windows::core::Result<::windows::UI::Text::FontWeight> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontWeight = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        }
    }
    pub fn SetFontWeight<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::UI::Text::FontWeight>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FontStyle(&self) -> ::windows::core::Result<::windows::UI::Text::FontStyle> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStyle = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStyle>(result__)
        }
    }
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn FontStretch(&self) -> ::windows::core::Result<::windows::UI::Text::FontStretch> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStretch = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStretch>(result__)
        }
    }
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn CharacterSpacing(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Media"
    ))]
    pub fn Foreground(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Media"
    ))]
    pub fn SetForeground<'a, Param0: ::windows::core::IntoParam<'a, super::Media::Brush>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).20)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).22)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn TextDecorations(&self) -> ::windows::core::Result<::windows::UI::Text::TextDecorations> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::TextDecorations = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::TextDecorations>(result__)
        }
    }
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
    pub fn ContentStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
    pub fn ContentEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
    pub fn ElementStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
    pub fn ElementEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).31)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn AccessKey(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).33)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetAccessKey<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).34)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).35)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).36)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).37)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).38)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn AccessKeyScopeOwner(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).39)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetAccessKeyScopeOwner<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).40)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::core::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::Input::KeyTipPlacementMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).41)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).42)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).43)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).44)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).45)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).46)(::core::mem::transmute_copy(this), value)
                .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn XamlRoot(&self) -> ::windows::core::Result<super::XamlRoot> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).47)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::XamlRoot>(result__)
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn SetXamlRoot<'a, Param0: ::windows::core::IntoParam<'a, super::XamlRoot>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).48)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).49)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyDisplayRequested<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).50)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).51)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).52)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).53)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyInvoked<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).54)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FindName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).55)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for Inline {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.Inline;{813d427a-8980-5a79-a8fa-f27919cfb24f})",
    );
}
unsafe impl ::windows::core::Interface for Inline {
    type Vtable = IInline_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x813d427a_8980_5a79_a8fa_f27919cfb24f);
}
impl ::windows::core::RuntimeName for Inline {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Inline";
}
impl ::core::convert::From<Inline> for ::windows::core::IUnknown {
    fn from(value: Inline) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Inline> for ::windows::core::IUnknown {
    fn from(value: &Inline) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Inline {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Inline {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Inline> for ::windows::core::IInspectable {
    fn from(value: Inline) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Inline> for ::windows::core::IInspectable {
    fn from(value: &Inline) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Inline {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Inline {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<Inline> for TextElement {
    fn from(value: Inline) -> Self {
        ::core::convert::Into::<TextElement>::into(&value)
    }
}
impl ::core::convert::From<&Inline> for TextElement {
    fn from(value: &Inline) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for Inline {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<TextElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for &Inline {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<TextElement>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<Inline> for super::DependencyObject {
    fn from(value: Inline) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&Inline> for super::DependencyObject {
    fn from(value: &Inline) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for Inline {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &Inline {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for Inline {}
unsafe impl ::core::marker::Sync for Inline {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct InlineCollection(pub ::windows::core::IInspectable);
impl InlineCollection {
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<Inline> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                index,
                &mut result__,
            )
            .from_abi::<Inline>(result__)
        }
    }
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn GetView(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Collections::IVectorView<Inline>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IVectorView<Inline>>(result__)
        }
    }
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, Inline>>(
        &self,
        value: Param0,
        index: &mut u32,
    ) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
                index,
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, Inline>>(
        &self,
        index: u32,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                index,
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, Inline>>(
        &self,
        index: u32,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                index,
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), index)
                .ok()
        }
    }
    pub fn Append<'a, Param0: ::windows::core::IntoParam<'a, Inline>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok()
        }
    }
    pub fn GetMany(
        &self,
        startindex: u32,
        items: &mut [<Inline as ::windows::core::DefaultType>::DefaultType],
    ) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(
                ::core::mem::transmute_copy(this),
                startindex,
                items.len() as u32,
                ::core::mem::transmute_copy(&items),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn ReplaceAll(
        &self,
        items: &[<Inline as ::windows::core::DefaultType>::DefaultType],
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).17)(
                ::core::mem::transmute_copy(this),
                items.len() as u32,
                ::core::mem::transmute(items.as_ptr()),
            )
            .ok()
        }
    }
    pub fn First(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Collections::IIterator<Inline>> {
        let this = &::windows::core::Interface::cast::<
            ::windows::Foundation::Collections::IIterable<Inline>,
        >(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IIterator<Inline>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for InlineCollection {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Documents.InlineCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Microsoft.UI.Xaml.Documents.Inline;{813d427a-8980-5a79-a8fa-f27919cfb24f})))" ) ;
}
unsafe impl ::windows::core::Interface for InlineCollection {
    type Vtable = ::windows::Foundation::Collections::IVector_abi<Inline>;
    const IID : :: windows :: core :: GUID = :: windows :: core :: GUID :: from_signature ( < ::windows::Foundation::Collections:: IVector :: < Inline > as :: windows :: core :: RuntimeType > :: SIGNATURE ) ;
}
impl ::windows::core::RuntimeName for InlineCollection {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.InlineCollection";
}
impl ::core::convert::From<InlineCollection> for ::windows::core::IUnknown {
    fn from(value: InlineCollection) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InlineCollection> for ::windows::core::IUnknown {
    fn from(value: &InlineCollection) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InlineCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InlineCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InlineCollection> for ::windows::core::IInspectable {
    fn from(value: InlineCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InlineCollection> for ::windows::core::IInspectable {
    fn from(value: &InlineCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InlineCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InlineCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<InlineCollection>
    for ::windows::Foundation::Collections::IVector<Inline>
{
    fn from(value: InlineCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InlineCollection>
    for ::windows::Foundation::Collections::IVector<Inline>
{
    fn from(value: &InlineCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::Collections::IVector<Inline>>
    for InlineCollection
{
    fn into_param(
        self,
    ) -> ::windows::core::Param<'a, ::windows::Foundation::Collections::IVector<Inline>> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::Collections::IVector<Inline>>
    for &InlineCollection
{
    fn into_param(
        self,
    ) -> ::windows::core::Param<'a, ::windows::Foundation::Collections::IVector<Inline>> {
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
unsafe impl ::core::marker::Send for InlineCollection {}
unsafe impl ::core::marker::Sync for InlineCollection {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for InlineCollection {
    type Item = Inline;
    type IntoIter = ::windows::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for &InlineCollection {
    type Item = Inline;
    type IntoIter = ::windows::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::windows::Foundation::Collections::VectorIterator::new(
            ::core::convert::TryInto::try_into(self).ok(),
        )
    }
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct InlineUIContainer(pub ::windows::core::IInspectable);
impl InlineUIContainer {
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
            InlineUIContainer,
            ::windows::core::IActivationFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn Child(&self) -> ::windows::core::Result<super::UIElement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::UIElement>(result__)
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn SetChild<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(
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
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
    pub fn ClearValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ReadLocalValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn FontSize(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows::core::Result<super::Media::FontFamily> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFontFamily<'a, Param0: ::windows::core::IntoParam<'a, super::Media::FontFamily>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FontWeight(&self) -> ::windows::core::Result<::windows::UI::Text::FontWeight> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontWeight = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        }
    }
    pub fn SetFontWeight<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::UI::Text::FontWeight>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FontStyle(&self) -> ::windows::core::Result<::windows::UI::Text::FontStyle> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStyle = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStyle>(result__)
        }
    }
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn FontStretch(&self) -> ::windows::core::Result<::windows::UI::Text::FontStretch> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStretch = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStretch>(result__)
        }
    }
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn CharacterSpacing(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Media"
    ))]
    pub fn Foreground(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Media"
    ))]
    pub fn SetForeground<'a, Param0: ::windows::core::IntoParam<'a, super::Media::Brush>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).20)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).22)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn TextDecorations(&self) -> ::windows::core::Result<::windows::UI::Text::TextDecorations> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::TextDecorations = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::TextDecorations>(result__)
        }
    }
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
    pub fn ContentStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
    pub fn ContentEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
    pub fn ElementStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
    pub fn ElementEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).31)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn AccessKey(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).33)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetAccessKey<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).34)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).35)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).36)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).37)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).38)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn AccessKeyScopeOwner(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).39)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetAccessKeyScopeOwner<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).40)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::core::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::Input::KeyTipPlacementMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).41)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).42)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).43)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).44)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).45)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).46)(::core::mem::transmute_copy(this), value)
                .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn XamlRoot(&self) -> ::windows::core::Result<super::XamlRoot> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).47)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::XamlRoot>(result__)
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn SetXamlRoot<'a, Param0: ::windows::core::IntoParam<'a, super::XamlRoot>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).48)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).49)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyDisplayRequested<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).50)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).51)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).52)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).53)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyInvoked<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).54)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FindName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).55)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for InlineUIContainer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.InlineUIContainer;{d529bef6-c05a-5bad-85e8-640127cf86f5})",
    );
}
unsafe impl ::windows::core::Interface for InlineUIContainer {
    type Vtable = IInlineUIContainer_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xd529bef6_c05a_5bad_85e8_640127cf86f5);
}
impl ::windows::core::RuntimeName for InlineUIContainer {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.InlineUIContainer";
}
impl ::core::convert::From<InlineUIContainer> for ::windows::core::IUnknown {
    fn from(value: InlineUIContainer) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InlineUIContainer> for ::windows::core::IUnknown {
    fn from(value: &InlineUIContainer) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InlineUIContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InlineUIContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InlineUIContainer> for ::windows::core::IInspectable {
    fn from(value: InlineUIContainer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InlineUIContainer> for ::windows::core::IInspectable {
    fn from(value: &InlineUIContainer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InlineUIContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InlineUIContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<InlineUIContainer> for Inline {
    fn from(value: InlineUIContainer) -> Self {
        ::core::convert::Into::<Inline>::into(&value)
    }
}
impl ::core::convert::From<&InlineUIContainer> for Inline {
    fn from(value: &InlineUIContainer) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Inline> for InlineUIContainer {
    fn into_param(self) -> ::windows::core::Param<'a, Inline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Inline>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Inline> for &InlineUIContainer {
    fn into_param(self) -> ::windows::core::Param<'a, Inline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Inline>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<InlineUIContainer> for TextElement {
    fn from(value: InlineUIContainer) -> Self {
        ::core::convert::Into::<TextElement>::into(&value)
    }
}
impl ::core::convert::From<&InlineUIContainer> for TextElement {
    fn from(value: &InlineUIContainer) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for InlineUIContainer {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<TextElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for &InlineUIContainer {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<TextElement>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<InlineUIContainer> for super::DependencyObject {
    fn from(value: InlineUIContainer) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&InlineUIContainer> for super::DependencyObject {
    fn from(value: &InlineUIContainer) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for InlineUIContainer {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &InlineUIContainer {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for InlineUIContainer {}
unsafe impl ::core::marker::Sync for InlineUIContainer {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct Italic(pub ::windows::core::IInspectable);
impl Italic {
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
            Italic,
            ::windows::core::IActivationFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
    pub fn ClearValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ReadLocalValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn Inlines(&self) -> ::windows::core::Result<InlineCollection> {
        let this = &::windows::core::Interface::cast::<ISpan>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InlineCollection>(result__)
        }
    }
    pub fn SetInlines<'a, Param0: ::windows::core::IntoParam<'a, InlineCollection>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISpan>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn FontSize(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows::core::Result<super::Media::FontFamily> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFontFamily<'a, Param0: ::windows::core::IntoParam<'a, super::Media::FontFamily>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FontWeight(&self) -> ::windows::core::Result<::windows::UI::Text::FontWeight> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontWeight = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        }
    }
    pub fn SetFontWeight<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::UI::Text::FontWeight>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FontStyle(&self) -> ::windows::core::Result<::windows::UI::Text::FontStyle> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStyle = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStyle>(result__)
        }
    }
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn FontStretch(&self) -> ::windows::core::Result<::windows::UI::Text::FontStretch> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStretch = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStretch>(result__)
        }
    }
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn CharacterSpacing(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Media"
    ))]
    pub fn Foreground(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Media"
    ))]
    pub fn SetForeground<'a, Param0: ::windows::core::IntoParam<'a, super::Media::Brush>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).20)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).22)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn TextDecorations(&self) -> ::windows::core::Result<::windows::UI::Text::TextDecorations> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::TextDecorations = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::TextDecorations>(result__)
        }
    }
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
    pub fn ContentStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
    pub fn ContentEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
    pub fn ElementStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
    pub fn ElementEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).31)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn AccessKey(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).33)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetAccessKey<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).34)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).35)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).36)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).37)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).38)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn AccessKeyScopeOwner(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).39)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetAccessKeyScopeOwner<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).40)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::core::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::Input::KeyTipPlacementMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).41)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).42)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).43)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).44)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).45)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).46)(::core::mem::transmute_copy(this), value)
                .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn XamlRoot(&self) -> ::windows::core::Result<super::XamlRoot> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).47)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::XamlRoot>(result__)
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn SetXamlRoot<'a, Param0: ::windows::core::IntoParam<'a, super::XamlRoot>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).48)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).49)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyDisplayRequested<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).50)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).51)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).52)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).53)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyInvoked<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).54)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FindName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).55)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for Italic {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.Italic;{ca3cbebd-7a8d-5d7a-8fdf-538e8a680f6c})",
    );
}
unsafe impl ::windows::core::Interface for Italic {
    type Vtable = IItalic_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xca3cbebd_7a8d_5d7a_8fdf_538e8a680f6c);
}
impl ::windows::core::RuntimeName for Italic {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Italic";
}
impl ::core::convert::From<Italic> for ::windows::core::IUnknown {
    fn from(value: Italic) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Italic> for ::windows::core::IUnknown {
    fn from(value: &Italic) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Italic {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Italic {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Italic> for ::windows::core::IInspectable {
    fn from(value: Italic) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Italic> for ::windows::core::IInspectable {
    fn from(value: &Italic) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Italic {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Italic {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<Italic> for Span {
    fn from(value: Italic) -> Self {
        ::core::convert::Into::<Span>::into(&value)
    }
}
impl ::core::convert::From<&Italic> for Span {
    fn from(value: &Italic) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Span> for Italic {
    fn into_param(self) -> ::windows::core::Param<'a, Span> {
        ::windows::core::Param::Owned(::core::convert::Into::<Span>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Span> for &Italic {
    fn into_param(self) -> ::windows::core::Param<'a, Span> {
        ::windows::core::Param::Owned(::core::convert::Into::<Span>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<Italic> for Inline {
    fn from(value: Italic) -> Self {
        ::core::convert::Into::<Inline>::into(&value)
    }
}
impl ::core::convert::From<&Italic> for Inline {
    fn from(value: &Italic) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Inline> for Italic {
    fn into_param(self) -> ::windows::core::Param<'a, Inline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Inline>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Inline> for &Italic {
    fn into_param(self) -> ::windows::core::Param<'a, Inline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Inline>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<Italic> for TextElement {
    fn from(value: Italic) -> Self {
        ::core::convert::Into::<TextElement>::into(&value)
    }
}
impl ::core::convert::From<&Italic> for TextElement {
    fn from(value: &Italic) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for Italic {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<TextElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for &Italic {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<TextElement>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<Italic> for super::DependencyObject {
    fn from(value: Italic) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&Italic> for super::DependencyObject {
    fn from(value: &Italic) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for Italic {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &Italic {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for Italic {}
unsafe impl ::core::marker::Sync for Italic {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct LineBreak(pub ::windows::core::IInspectable);
impl LineBreak {
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
            LineBreak,
            ::windows::core::IActivationFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
    pub fn ClearValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ReadLocalValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn FontSize(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows::core::Result<super::Media::FontFamily> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFontFamily<'a, Param0: ::windows::core::IntoParam<'a, super::Media::FontFamily>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FontWeight(&self) -> ::windows::core::Result<::windows::UI::Text::FontWeight> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontWeight = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        }
    }
    pub fn SetFontWeight<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::UI::Text::FontWeight>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FontStyle(&self) -> ::windows::core::Result<::windows::UI::Text::FontStyle> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStyle = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStyle>(result__)
        }
    }
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn FontStretch(&self) -> ::windows::core::Result<::windows::UI::Text::FontStretch> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStretch = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStretch>(result__)
        }
    }
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn CharacterSpacing(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Media"
    ))]
    pub fn Foreground(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Media"
    ))]
    pub fn SetForeground<'a, Param0: ::windows::core::IntoParam<'a, super::Media::Brush>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).20)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).22)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn TextDecorations(&self) -> ::windows::core::Result<::windows::UI::Text::TextDecorations> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::TextDecorations = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::TextDecorations>(result__)
        }
    }
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
    pub fn ContentStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
    pub fn ContentEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
    pub fn ElementStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
    pub fn ElementEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).31)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn AccessKey(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).33)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetAccessKey<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).34)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).35)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).36)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).37)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).38)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn AccessKeyScopeOwner(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).39)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetAccessKeyScopeOwner<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).40)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::core::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::Input::KeyTipPlacementMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).41)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).42)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).43)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).44)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).45)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).46)(::core::mem::transmute_copy(this), value)
                .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn XamlRoot(&self) -> ::windows::core::Result<super::XamlRoot> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).47)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::XamlRoot>(result__)
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn SetXamlRoot<'a, Param0: ::windows::core::IntoParam<'a, super::XamlRoot>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).48)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).49)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyDisplayRequested<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).50)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).51)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).52)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).53)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyInvoked<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).54)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FindName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).55)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for LineBreak {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.LineBreak;{09307599-7cc2-5f54-b106-728620c16f76})",
    );
}
unsafe impl ::windows::core::Interface for LineBreak {
    type Vtable = ILineBreak_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x09307599_7cc2_5f54_b106_728620c16f76);
}
impl ::windows::core::RuntimeName for LineBreak {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.LineBreak";
}
impl ::core::convert::From<LineBreak> for ::windows::core::IUnknown {
    fn from(value: LineBreak) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LineBreak> for ::windows::core::IUnknown {
    fn from(value: &LineBreak) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LineBreak {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LineBreak {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LineBreak> for ::windows::core::IInspectable {
    fn from(value: LineBreak) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LineBreak> for ::windows::core::IInspectable {
    fn from(value: &LineBreak) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LineBreak {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LineBreak {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<LineBreak> for Inline {
    fn from(value: LineBreak) -> Self {
        ::core::convert::Into::<Inline>::into(&value)
    }
}
impl ::core::convert::From<&LineBreak> for Inline {
    fn from(value: &LineBreak) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Inline> for LineBreak {
    fn into_param(self) -> ::windows::core::Param<'a, Inline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Inline>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Inline> for &LineBreak {
    fn into_param(self) -> ::windows::core::Param<'a, Inline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Inline>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<LineBreak> for TextElement {
    fn from(value: LineBreak) -> Self {
        ::core::convert::Into::<TextElement>::into(&value)
    }
}
impl ::core::convert::From<&LineBreak> for TextElement {
    fn from(value: &LineBreak) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for LineBreak {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<TextElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for &LineBreak {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<TextElement>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<LineBreak> for super::DependencyObject {
    fn from(value: LineBreak) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&LineBreak> for super::DependencyObject {
    fn from(value: &LineBreak) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for LineBreak {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &LineBreak {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for LineBreak {}
unsafe impl ::core::marker::Sync for LineBreak {}
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct LogicalDirection(pub i32);
impl LogicalDirection {
    pub const Backward: LogicalDirection = LogicalDirection(0i32);
    pub const Forward: LogicalDirection = LogicalDirection(1i32);
}
impl ::core::convert::From<i32> for LogicalDirection {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for LogicalDirection {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for LogicalDirection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Documents.LogicalDirection;i4)",
    );
}
impl ::windows::core::DefaultType for LogicalDirection {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct Paragraph(pub ::windows::core::IInspectable);
impl Paragraph {
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
            Paragraph,
            ::windows::core::IActivationFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Inlines(&self) -> ::windows::core::Result<InlineCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InlineCollection>(result__)
        }
    }
    pub fn TextIndent(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetTextIndent(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn TextIndentProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IParagraphStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
    pub fn ClearValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ReadLocalValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn TextAlignment(&self) -> ::windows::core::Result<super::TextAlignment> {
        let this = &::windows::core::Interface::cast::<IBlock>(self)?;
        unsafe {
            let mut result__: super::TextAlignment = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::TextAlignment>(result__)
        }
    }
    pub fn SetTextAlignment(&self, value: super::TextAlignment) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBlock>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn HorizontalTextAlignment(&self) -> ::windows::core::Result<super::TextAlignment> {
        let this = &::windows::core::Interface::cast::<IBlock>(self)?;
        unsafe {
            let mut result__: super::TextAlignment = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::TextAlignment>(result__)
        }
    }
    pub fn SetHorizontalTextAlignment(
        &self,
        value: super::TextAlignment,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBlock>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn LineHeight(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IBlock>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetLineHeight(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBlock>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn LineStackingStrategy(&self) -> ::windows::core::Result<super::LineStackingStrategy> {
        let this = &::windows::core::Interface::cast::<IBlock>(self)?;
        unsafe {
            let mut result__: super::LineStackingStrategy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::LineStackingStrategy>(result__)
        }
    }
    pub fn SetLineStackingStrategy(
        &self,
        value: super::LineStackingStrategy,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBlock>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn Margin(&self) -> ::windows::core::Result<super::Thickness> {
        let this = &::windows::core::Interface::cast::<IBlock>(self)?;
        unsafe {
            let mut result__: super::Thickness = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Thickness>(result__)
        }
    }
    pub fn SetMargin<'a, Param0: ::windows::core::IntoParam<'a, super::Thickness>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBlock>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn FontSize(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows::core::Result<super::Media::FontFamily> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFontFamily<'a, Param0: ::windows::core::IntoParam<'a, super::Media::FontFamily>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FontWeight(&self) -> ::windows::core::Result<::windows::UI::Text::FontWeight> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontWeight = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        }
    }
    pub fn SetFontWeight<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::UI::Text::FontWeight>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FontStyle(&self) -> ::windows::core::Result<::windows::UI::Text::FontStyle> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStyle = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStyle>(result__)
        }
    }
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn FontStretch(&self) -> ::windows::core::Result<::windows::UI::Text::FontStretch> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStretch = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStretch>(result__)
        }
    }
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn CharacterSpacing(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Media"
    ))]
    pub fn Foreground(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Media"
    ))]
    pub fn SetForeground<'a, Param0: ::windows::core::IntoParam<'a, super::Media::Brush>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).20)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).22)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn TextDecorations(&self) -> ::windows::core::Result<::windows::UI::Text::TextDecorations> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::TextDecorations = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::TextDecorations>(result__)
        }
    }
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
    pub fn ContentStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
    pub fn ContentEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
    pub fn ElementStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
    pub fn ElementEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).31)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn AccessKey(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).33)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetAccessKey<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).34)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).35)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).36)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).37)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).38)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn AccessKeyScopeOwner(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).39)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetAccessKeyScopeOwner<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).40)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::core::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::Input::KeyTipPlacementMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).41)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).42)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).43)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).44)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).45)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).46)(::core::mem::transmute_copy(this), value)
                .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn XamlRoot(&self) -> ::windows::core::Result<super::XamlRoot> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).47)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::XamlRoot>(result__)
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn SetXamlRoot<'a, Param0: ::windows::core::IntoParam<'a, super::XamlRoot>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).48)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).49)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyDisplayRequested<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).50)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).51)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).52)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).53)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyInvoked<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).54)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FindName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).55)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn IParagraphStatics<R, F: FnOnce(&IParagraphStatics) -> ::windows::core::Result<R>>(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Paragraph, IParagraphStatics> =
            ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for Paragraph {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.Paragraph;{9ed64c77-329d-502f-a257-f58398edab51})",
    );
}
unsafe impl ::windows::core::Interface for Paragraph {
    type Vtable = IParagraph_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x9ed64c77_329d_502f_a257_f58398edab51);
}
impl ::windows::core::RuntimeName for Paragraph {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Paragraph";
}
impl ::core::convert::From<Paragraph> for ::windows::core::IUnknown {
    fn from(value: Paragraph) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Paragraph> for ::windows::core::IUnknown {
    fn from(value: &Paragraph) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Paragraph {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Paragraph {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Paragraph> for ::windows::core::IInspectable {
    fn from(value: Paragraph) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Paragraph> for ::windows::core::IInspectable {
    fn from(value: &Paragraph) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Paragraph {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Paragraph {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<Paragraph> for Block {
    fn from(value: Paragraph) -> Self {
        ::core::convert::Into::<Block>::into(&value)
    }
}
impl ::core::convert::From<&Paragraph> for Block {
    fn from(value: &Paragraph) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Block> for Paragraph {
    fn into_param(self) -> ::windows::core::Param<'a, Block> {
        ::windows::core::Param::Owned(::core::convert::Into::<Block>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Block> for &Paragraph {
    fn into_param(self) -> ::windows::core::Param<'a, Block> {
        ::windows::core::Param::Owned(::core::convert::Into::<Block>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<Paragraph> for TextElement {
    fn from(value: Paragraph) -> Self {
        ::core::convert::Into::<TextElement>::into(&value)
    }
}
impl ::core::convert::From<&Paragraph> for TextElement {
    fn from(value: &Paragraph) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for Paragraph {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<TextElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for &Paragraph {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<TextElement>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<Paragraph> for super::DependencyObject {
    fn from(value: Paragraph) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&Paragraph> for super::DependencyObject {
    fn from(value: &Paragraph) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for Paragraph {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &Paragraph {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for Paragraph {}
unsafe impl ::core::marker::Sync for Paragraph {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct Run(pub ::windows::core::IInspectable);
impl Run {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Run, ::windows::core::IActivationFactory> =
            ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
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
    pub fn FlowDirection(&self) -> ::windows::core::Result<super::FlowDirection> {
        let this = self;
        unsafe {
            let mut result__: super::FlowDirection = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::FlowDirection>(result__)
        }
    }
    pub fn SetFlowDirection(&self, value: super::FlowDirection) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn FlowDirectionProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IRunStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
    pub fn ClearValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ReadLocalValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn FontSize(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows::core::Result<super::Media::FontFamily> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFontFamily<'a, Param0: ::windows::core::IntoParam<'a, super::Media::FontFamily>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FontWeight(&self) -> ::windows::core::Result<::windows::UI::Text::FontWeight> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontWeight = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        }
    }
    pub fn SetFontWeight<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::UI::Text::FontWeight>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FontStyle(&self) -> ::windows::core::Result<::windows::UI::Text::FontStyle> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStyle = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStyle>(result__)
        }
    }
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn FontStretch(&self) -> ::windows::core::Result<::windows::UI::Text::FontStretch> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStretch = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStretch>(result__)
        }
    }
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn CharacterSpacing(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Media"
    ))]
    pub fn Foreground(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Media"
    ))]
    pub fn SetForeground<'a, Param0: ::windows::core::IntoParam<'a, super::Media::Brush>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).20)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).22)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn TextDecorations(&self) -> ::windows::core::Result<::windows::UI::Text::TextDecorations> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::TextDecorations = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::TextDecorations>(result__)
        }
    }
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
    pub fn ContentStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
    pub fn ContentEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
    pub fn ElementStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
    pub fn ElementEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).31)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn AccessKey(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).33)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetAccessKey<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).34)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).35)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).36)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).37)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).38)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn AccessKeyScopeOwner(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).39)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetAccessKeyScopeOwner<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).40)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::core::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::Input::KeyTipPlacementMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).41)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).42)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).43)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).44)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).45)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).46)(::core::mem::transmute_copy(this), value)
                .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn XamlRoot(&self) -> ::windows::core::Result<super::XamlRoot> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).47)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::XamlRoot>(result__)
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn SetXamlRoot<'a, Param0: ::windows::core::IntoParam<'a, super::XamlRoot>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).48)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).49)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyDisplayRequested<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).50)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).51)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).52)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).53)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyInvoked<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).54)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FindName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).55)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn IRunStatics<R, F: FnOnce(&IRunStatics) -> ::windows::core::Result<R>>(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Run, IRunStatics> =
            ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for Run {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.Run;{1f905239-37cb-590b-9132-3ffb7741906e})",
    );
}
unsafe impl ::windows::core::Interface for Run {
    type Vtable = IRun_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x1f905239_37cb_590b_9132_3ffb7741906e);
}
impl ::windows::core::RuntimeName for Run {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Run";
}
impl ::core::convert::From<Run> for ::windows::core::IUnknown {
    fn from(value: Run) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Run> for ::windows::core::IUnknown {
    fn from(value: &Run) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Run {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Run {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Run> for ::windows::core::IInspectable {
    fn from(value: Run) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Run> for ::windows::core::IInspectable {
    fn from(value: &Run) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Run {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Run {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<Run> for Inline {
    fn from(value: Run) -> Self {
        ::core::convert::Into::<Inline>::into(&value)
    }
}
impl ::core::convert::From<&Run> for Inline {
    fn from(value: &Run) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Inline> for Run {
    fn into_param(self) -> ::windows::core::Param<'a, Inline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Inline>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Inline> for &Run {
    fn into_param(self) -> ::windows::core::Param<'a, Inline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Inline>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<Run> for TextElement {
    fn from(value: Run) -> Self {
        ::core::convert::Into::<TextElement>::into(&value)
    }
}
impl ::core::convert::From<&Run> for TextElement {
    fn from(value: &Run) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for Run {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<TextElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for &Run {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<TextElement>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<Run> for super::DependencyObject {
    fn from(value: Run) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&Run> for super::DependencyObject {
    fn from(value: &Run) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for Run {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &Run {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for Run {}
unsafe impl ::core::marker::Sync for Run {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct Span(pub ::windows::core::IInspectable);
impl Span {
    pub fn Inlines(&self) -> ::windows::core::Result<InlineCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InlineCollection>(result__)
        }
    }
    pub fn SetInlines<'a, Param0: ::windows::core::IntoParam<'a, InlineCollection>>(
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
    pub fn new() -> ::windows::core::Result<Span> {
        Self::ISpanFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                ::core::ptr::null_mut(),
                &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _,
                &mut result__,
            )
            .from_abi::<Span>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
    pub fn ClearValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ReadLocalValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn FontSize(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows::core::Result<super::Media::FontFamily> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFontFamily<'a, Param0: ::windows::core::IntoParam<'a, super::Media::FontFamily>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FontWeight(&self) -> ::windows::core::Result<::windows::UI::Text::FontWeight> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontWeight = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        }
    }
    pub fn SetFontWeight<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::UI::Text::FontWeight>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FontStyle(&self) -> ::windows::core::Result<::windows::UI::Text::FontStyle> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStyle = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStyle>(result__)
        }
    }
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn FontStretch(&self) -> ::windows::core::Result<::windows::UI::Text::FontStretch> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStretch = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStretch>(result__)
        }
    }
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn CharacterSpacing(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Media"
    ))]
    pub fn Foreground(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Media"
    ))]
    pub fn SetForeground<'a, Param0: ::windows::core::IntoParam<'a, super::Media::Brush>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).20)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).22)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn TextDecorations(&self) -> ::windows::core::Result<::windows::UI::Text::TextDecorations> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::TextDecorations = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::TextDecorations>(result__)
        }
    }
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
    pub fn ContentStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
    pub fn ContentEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
    pub fn ElementStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
    pub fn ElementEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).31)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn AccessKey(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).33)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetAccessKey<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).34)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).35)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).36)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).37)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).38)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn AccessKeyScopeOwner(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).39)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetAccessKeyScopeOwner<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).40)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::core::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::Input::KeyTipPlacementMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).41)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).42)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).43)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).44)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).45)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).46)(::core::mem::transmute_copy(this), value)
                .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn XamlRoot(&self) -> ::windows::core::Result<super::XamlRoot> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).47)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::XamlRoot>(result__)
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn SetXamlRoot<'a, Param0: ::windows::core::IntoParam<'a, super::XamlRoot>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).48)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).49)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyDisplayRequested<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).50)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).51)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).52)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).53)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyInvoked<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).54)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FindName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).55)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn ISpanFactory<R, F: FnOnce(&ISpanFactory) -> ::windows::core::Result<R>>(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Span, ISpanFactory> =
            ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for Span {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.Span;{91b93d4d-4e28-57b9-bffb-3566c2a3c2a1})",
    );
}
unsafe impl ::windows::core::Interface for Span {
    type Vtable = ISpan_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x91b93d4d_4e28_57b9_bffb_3566c2a3c2a1);
}
impl ::windows::core::RuntimeName for Span {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Span";
}
impl ::core::convert::From<Span> for ::windows::core::IUnknown {
    fn from(value: Span) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Span> for ::windows::core::IUnknown {
    fn from(value: &Span) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Span {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Span {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Span> for ::windows::core::IInspectable {
    fn from(value: Span) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Span> for ::windows::core::IInspectable {
    fn from(value: &Span) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Span {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Span {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<Span> for Inline {
    fn from(value: Span) -> Self {
        ::core::convert::Into::<Inline>::into(&value)
    }
}
impl ::core::convert::From<&Span> for Inline {
    fn from(value: &Span) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Inline> for Span {
    fn into_param(self) -> ::windows::core::Param<'a, Inline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Inline>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Inline> for &Span {
    fn into_param(self) -> ::windows::core::Param<'a, Inline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Inline>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<Span> for TextElement {
    fn from(value: Span) -> Self {
        ::core::convert::Into::<TextElement>::into(&value)
    }
}
impl ::core::convert::From<&Span> for TextElement {
    fn from(value: &Span) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for Span {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<TextElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for &Span {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<TextElement>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<Span> for super::DependencyObject {
    fn from(value: Span) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&Span> for super::DependencyObject {
    fn from(value: &Span) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for Span {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &Span {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for Span {}
unsafe impl ::core::marker::Sync for Span {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct TextElement(pub ::windows::core::IInspectable);
impl TextElement {
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn FontSize(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows::core::Result<super::Media::FontFamily> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFontFamily<'a, Param0: ::windows::core::IntoParam<'a, super::Media::FontFamily>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FontWeight(&self) -> ::windows::core::Result<::windows::UI::Text::FontWeight> {
        let this = self;
        unsafe {
            let mut result__: ::windows::UI::Text::FontWeight = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        }
    }
    pub fn SetFontWeight<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::UI::Text::FontWeight>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FontStyle(&self) -> ::windows::core::Result<::windows::UI::Text::FontStyle> {
        let this = self;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStyle = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStyle>(result__)
        }
    }
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn FontStretch(&self) -> ::windows::core::Result<::windows::UI::Text::FontStretch> {
        let this = self;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStretch = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStretch>(result__)
        }
    }
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn CharacterSpacing(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Media"
    ))]
    pub fn Foreground(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Media"
    ))]
    pub fn SetForeground<'a, Param0: ::windows::core::IntoParam<'a, super::Media::Brush>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).20)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).22)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn TextDecorations(&self) -> ::windows::core::Result<::windows::UI::Text::TextDecorations> {
        let this = self;
        unsafe {
            let mut result__: ::windows::UI::Text::TextDecorations = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::TextDecorations>(result__)
        }
    }
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
    pub fn ContentStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
    pub fn ContentEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
    pub fn ElementStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
    pub fn ElementEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).31)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn AccessKey(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).33)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetAccessKey<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).34)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).35)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).36)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).37)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).38)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn AccessKeyScopeOwner(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).39)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetAccessKeyScopeOwner<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).40)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::core::Result<super::Input::KeyTipPlacementMode> {
        let this = self;
        unsafe {
            let mut result__: super::Input::KeyTipPlacementMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).41)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).42)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).43)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).44)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).45)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).46)(::core::mem::transmute_copy(this), value)
                .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn XamlRoot(&self) -> ::windows::core::Result<super::XamlRoot> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).47)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::XamlRoot>(result__)
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn SetXamlRoot<'a, Param0: ::windows::core::IntoParam<'a, super::XamlRoot>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).48)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).49)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyDisplayRequested<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).50)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).51)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).52)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).53)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyInvoked<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).54)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FindName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).55)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn FontSizeProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn FontFamilyProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn FontWeightProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn FontStyleProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn FontStretchProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn CharacterSpacingProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ForegroundProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn LanguageProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn IsTextScaleFactorEnabledProperty() -> ::windows::core::Result<super::DependencyProperty>
    {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn TextDecorationsProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn AllowFocusOnInteractionProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn AccessKeyProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ExitDisplayModeOnAccessKeyInvokedProperty(
    ) -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn IsAccessKeyScopeProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn AccessKeyScopeOwnerProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn KeyTipPlacementModeProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn KeyTipHorizontalOffsetProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn KeyTipVerticalOffsetProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
    pub fn ClearValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ReadLocalValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn ITextElementStatics<R, F: FnOnce(&ITextElementStatics) -> ::windows::core::Result<R>>(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TextElement, ITextElementStatics> =
            ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for TextElement {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.TextElement;{a122ba22-833f-5220-a47e-6cd507531abe})",
    );
}
unsafe impl ::windows::core::Interface for TextElement {
    type Vtable = ITextElement_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xa122ba22_833f_5220_a47e_6cd507531abe);
}
impl ::windows::core::RuntimeName for TextElement {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.TextElement";
}
impl ::core::convert::From<TextElement> for ::windows::core::IUnknown {
    fn from(value: TextElement) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TextElement> for ::windows::core::IUnknown {
    fn from(value: &TextElement) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TextElement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TextElement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TextElement> for ::windows::core::IInspectable {
    fn from(value: TextElement) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TextElement> for ::windows::core::IInspectable {
    fn from(value: &TextElement) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TextElement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TextElement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<TextElement> for super::DependencyObject {
    fn from(value: TextElement) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&TextElement> for super::DependencyObject {
    fn from(value: &TextElement) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for TextElement {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &TextElement {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for TextElement {}
unsafe impl ::core::marker::Sync for TextElement {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct TextHighlighter(pub ::windows::core::IInspectable);
impl TextHighlighter {
    pub fn Ranges(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Collections::IVector<TextRange>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IVector<TextRange>>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Media"
    ))]
    pub fn Foreground(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Media"
    ))]
    pub fn SetForeground<'a, Param0: ::windows::core::IntoParam<'a, super::Media::Brush>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Media"
    ))]
    pub fn Background(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Media"
    ))]
    pub fn SetBackground<'a, Param0: ::windows::core::IntoParam<'a, super::Media::Brush>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ForegroundProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextHighlighterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn BackgroundProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextHighlighterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Media"
    ))]
    pub fn new() -> ::windows::core::Result<TextHighlighter> {
        Self::ITextHighlighterFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                ::core::ptr::null_mut(),
                &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _,
                &mut result__,
            )
            .from_abi::<TextHighlighter>(result__)
        })
    }
    pub fn ITextHighlighterStatics<
        R,
        F: FnOnce(&ITextHighlighterStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TextHighlighter, ITextHighlighterStatics> =
            ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ITextHighlighterFactory<
        R,
        F: FnOnce(&ITextHighlighterFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TextHighlighter, ITextHighlighterFactory> =
            ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for TextHighlighter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.TextHighlighter;{b756e861-1d2b-5f6f-81fd-c51a5bc068ff})",
    );
}
unsafe impl ::windows::core::Interface for TextHighlighter {
    type Vtable = ITextHighlighter_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xb756e861_1d2b_5f6f_81fd_c51a5bc068ff);
}
impl ::windows::core::RuntimeName for TextHighlighter {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.TextHighlighter";
}
impl ::core::convert::From<TextHighlighter> for ::windows::core::IUnknown {
    fn from(value: TextHighlighter) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TextHighlighter> for ::windows::core::IUnknown {
    fn from(value: &TextHighlighter) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TextHighlighter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TextHighlighter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TextHighlighter> for ::windows::core::IInspectable {
    fn from(value: TextHighlighter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TextHighlighter> for ::windows::core::IInspectable {
    fn from(value: &TextHighlighter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TextHighlighter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TextHighlighter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for TextHighlighter {}
unsafe impl ::core::marker::Sync for TextHighlighter {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct TextHighlighterBase(pub ::windows::core::IInspectable);
impl TextHighlighterBase {
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
    pub fn ClearValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ReadLocalValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for TextHighlighterBase {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Documents.TextHighlighterBase;{5c21aaf0-3a17-5468-8aac-be14db0ed8c1})" ) ;
}
unsafe impl ::windows::core::Interface for TextHighlighterBase {
    type Vtable = ITextHighlighterBase_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x5c21aaf0_3a17_5468_8aac_be14db0ed8c1);
}
impl ::windows::core::RuntimeName for TextHighlighterBase {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.TextHighlighterBase";
}
impl ::core::convert::From<TextHighlighterBase> for ::windows::core::IUnknown {
    fn from(value: TextHighlighterBase) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TextHighlighterBase> for ::windows::core::IUnknown {
    fn from(value: &TextHighlighterBase) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TextHighlighterBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TextHighlighterBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TextHighlighterBase> for ::windows::core::IInspectable {
    fn from(value: TextHighlighterBase) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TextHighlighterBase> for ::windows::core::IInspectable {
    fn from(value: &TextHighlighterBase) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TextHighlighterBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TextHighlighterBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<TextHighlighterBase> for super::DependencyObject {
    fn from(value: TextHighlighterBase) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&TextHighlighterBase> for super::DependencyObject {
    fn from(value: &TextHighlighterBase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for TextHighlighterBase {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &TextHighlighterBase {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for TextHighlighterBase {}
unsafe impl ::core::marker::Sync for TextHighlighterBase {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct TextPointer(pub ::windows::core::IInspectable);
impl TextPointer {
    #[cfg(feature = "UI_Dispatching")]
    pub fn Parent(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
    pub fn VisualParent(&self) -> ::windows::core::Result<super::FrameworkElement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::FrameworkElement>(result__)
        }
    }
    pub fn LogicalDirection(&self) -> ::windows::core::Result<LogicalDirection> {
        let this = self;
        unsafe {
            let mut result__: LogicalDirection = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<LogicalDirection>(result__)
        }
    }
    pub fn Offset(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn GetCharacterRect(
        &self,
        direction: LogicalDirection,
    ) -> ::windows::core::Result<::windows::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                direction,
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Rect>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
    pub fn GetPositionAtOffset(
        &self,
        offset: i32,
        direction: LogicalDirection,
    ) -> ::windows::core::Result<TextPointer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                offset,
                direction,
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for TextPointer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.TextPointer;{842eb385-ee41-5930-979b-438fa7525a51})",
    );
}
unsafe impl ::windows::core::Interface for TextPointer {
    type Vtable = ITextPointer_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x842eb385_ee41_5930_979b_438fa7525a51);
}
impl ::windows::core::RuntimeName for TextPointer {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.TextPointer";
}
impl ::core::convert::From<TextPointer> for ::windows::core::IUnknown {
    fn from(value: TextPointer) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TextPointer> for ::windows::core::IUnknown {
    fn from(value: &TextPointer) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TextPointer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TextPointer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TextPointer> for ::windows::core::IInspectable {
    fn from(value: TextPointer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TextPointer> for ::windows::core::IInspectable {
    fn from(value: &TextPointer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TextPointer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TextPointer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for TextPointer {}
unsafe impl ::core::marker::Sync for TextPointer {}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct TextRange {
    pub StartIndex: i32,
    pub Length: i32,
}
impl TextRange {}
impl ::core::default::Default for TextRange {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for TextRange {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("TextRange")
            .field("StartIndex", &self.StartIndex)
            .field("Length", &self.Length)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TextRange {
    fn eq(&self, other: &Self) -> bool {
        self.StartIndex == other.StartIndex && self.Length == other.Length
    }
}
impl ::core::cmp::Eq for TextRange {}
unsafe impl ::windows::core::Abi for TextRange {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for TextRange {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"struct(Microsoft.UI.Xaml.Documents.TextRange;i4;i4)",
    );
}
impl ::windows::core::DefaultType for TextRange {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct Typography(pub ::windows::core::IInspectable);
impl Typography {
    #[cfg(feature = "UI_Dispatching")]
    pub fn AnnotationAlternatesProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetAnnotationAlternates<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<i32> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetAnnotationAlternates<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: i32,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn EastAsianExpertFormsProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetEastAsianExpertForms<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetEastAsianExpertForms<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn EastAsianLanguageProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetEastAsianLanguage<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<super::FontEastAsianLanguage> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: super::FontEastAsianLanguage = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::FontEastAsianLanguage>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetEastAsianLanguage<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: super::FontEastAsianLanguage,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn EastAsianWidthsProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetEastAsianWidths<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<super::FontEastAsianWidths> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: super::FontEastAsianWidths = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::FontEastAsianWidths>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetEastAsianWidths<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: super::FontEastAsianWidths,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).17)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn StandardLigaturesProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetStandardLigatures<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetStandardLigatures<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).20)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ContextualLigaturesProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetContextualLigatures<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetContextualLigatures<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).23)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DiscretionaryLigaturesProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetDiscretionaryLigatures<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetDiscretionaryLigatures<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).26)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn HistoricalLigaturesProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetHistoricalLigatures<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetHistoricalLigatures<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).29)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn StandardSwashesProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetStandardSwashes<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<i32> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).31)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetStandardSwashes<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: i32,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).32)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ContextualSwashesProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).33)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetContextualSwashes<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<i32> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).34)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetContextualSwashes<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: i32,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).35)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ContextualAlternatesProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).36)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetContextualAlternates<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).37)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetContextualAlternates<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).38)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn StylisticAlternatesProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).39)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetStylisticAlternates<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<i32> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).40)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetStylisticAlternates<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: i32,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).41)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn StylisticSet1Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).42)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetStylisticSet1<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).43)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetStylisticSet1<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).44)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn StylisticSet2Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).45)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetStylisticSet2<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).46)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetStylisticSet2<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).47)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn StylisticSet3Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).48)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetStylisticSet3<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).49)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetStylisticSet3<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).50)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn StylisticSet4Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).51)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetStylisticSet4<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).52)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetStylisticSet4<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).53)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn StylisticSet5Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).54)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetStylisticSet5<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).55)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetStylisticSet5<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).56)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn StylisticSet6Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).57)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetStylisticSet6<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).58)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetStylisticSet6<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).59)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn StylisticSet7Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).60)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetStylisticSet7<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).61)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetStylisticSet7<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).62)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn StylisticSet8Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).63)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetStylisticSet8<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).64)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetStylisticSet8<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).65)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn StylisticSet9Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).66)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetStylisticSet9<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).67)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetStylisticSet9<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).68)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn StylisticSet10Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).69)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetStylisticSet10<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).70)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetStylisticSet10<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).71)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn StylisticSet11Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).72)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetStylisticSet11<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).73)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetStylisticSet11<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).74)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn StylisticSet12Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).75)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetStylisticSet12<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).76)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetStylisticSet12<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).77)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn StylisticSet13Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).78)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetStylisticSet13<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).79)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetStylisticSet13<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).80)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn StylisticSet14Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).81)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetStylisticSet14<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).82)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetStylisticSet14<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).83)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn StylisticSet15Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).84)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetStylisticSet15<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).85)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetStylisticSet15<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).86)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn StylisticSet16Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).87)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetStylisticSet16<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).88)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetStylisticSet16<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).89)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn StylisticSet17Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).90)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetStylisticSet17<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).91)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetStylisticSet17<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).92)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn StylisticSet18Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).93)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetStylisticSet18<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).94)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetStylisticSet18<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).95)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn StylisticSet19Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).96)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetStylisticSet19<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).97)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetStylisticSet19<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).98)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn StylisticSet20Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).99)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetStylisticSet20<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).100)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetStylisticSet20<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).101)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn CapitalsProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).102)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetCapitals<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::core::Result<super::FontCapitals> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: super::FontCapitals = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).103)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::FontCapitals>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetCapitals<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
        value: super::FontCapitals,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).104)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn CapitalSpacingProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).105)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetCapitalSpacing<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).106)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetCapitalSpacing<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).107)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn KerningProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).108)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetKerning<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).109)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetKerning<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).110)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn CaseSensitiveFormsProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).111)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetCaseSensitiveForms<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).112)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetCaseSensitiveForms<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).113)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn HistoricalFormsProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).114)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetHistoricalForms<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).115)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetHistoricalForms<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).116)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn FractionProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).117)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetFraction<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::core::Result<super::FontFraction> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: super::FontFraction = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).118)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::FontFraction>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetFraction<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
        value: super::FontFraction,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).119)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn NumeralStyleProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).120)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetNumeralStyle<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::core::Result<super::FontNumeralStyle> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: super::FontNumeralStyle = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).121)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::FontNumeralStyle>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetNumeralStyle<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
        value: super::FontNumeralStyle,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).122)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn NumeralAlignmentProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).123)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetNumeralAlignment<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<super::FontNumeralAlignment> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: super::FontNumeralAlignment = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).124)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::FontNumeralAlignment>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetNumeralAlignment<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: super::FontNumeralAlignment,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).125)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SlashedZeroProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).126)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetSlashedZero<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).127)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetSlashedZero<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).128)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn MathematicalGreekProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).129)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetMathematicalGreek<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).130)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetMathematicalGreek<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).131)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn VariantsProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).132)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetVariants<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::core::Result<super::FontVariants> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: super::FontVariants = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).133)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::FontVariants>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetVariants<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
        value: super::FontVariants,
    ) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).134)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn ITypographyStatics<R, F: FnOnce(&ITypographyStatics) -> ::windows::core::Result<R>>(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Typography, ITypographyStatics> =
            ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for Typography {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.Typography;{fa27e2e3-be5e-5d21-9a5e-90cf102af828})",
    );
}
unsafe impl ::windows::core::Interface for Typography {
    type Vtable = ITypography_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xfa27e2e3_be5e_5d21_9a5e_90cf102af828);
}
impl ::windows::core::RuntimeName for Typography {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Typography";
}
impl ::core::convert::From<Typography> for ::windows::core::IUnknown {
    fn from(value: Typography) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Typography> for ::windows::core::IUnknown {
    fn from(value: &Typography) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Typography {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Typography {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Typography> for ::windows::core::IInspectable {
    fn from(value: Typography) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Typography> for ::windows::core::IInspectable {
    fn from(value: &Typography) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Typography {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Typography {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for Typography {}
unsafe impl ::core::marker::Sync for Typography {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct Underline(pub ::windows::core::IInspectable);
impl Underline {
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
            Underline,
            ::windows::core::IActivationFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
    pub fn ClearValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ReadLocalValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
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
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn Inlines(&self) -> ::windows::core::Result<InlineCollection> {
        let this = &::windows::core::Interface::cast::<ISpan>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InlineCollection>(result__)
        }
    }
    pub fn SetInlines<'a, Param0: ::windows::core::IntoParam<'a, InlineCollection>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISpan>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn FontSize(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows::core::Result<super::Media::FontFamily> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFontFamily<'a, Param0: ::windows::core::IntoParam<'a, super::Media::FontFamily>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FontWeight(&self) -> ::windows::core::Result<::windows::UI::Text::FontWeight> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontWeight = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        }
    }
    pub fn SetFontWeight<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::UI::Text::FontWeight>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FontStyle(&self) -> ::windows::core::Result<::windows::UI::Text::FontStyle> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStyle = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStyle>(result__)
        }
    }
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn FontStretch(&self) -> ::windows::core::Result<::windows::UI::Text::FontStretch> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStretch = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStretch>(result__)
        }
    }
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn CharacterSpacing(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Media"
    ))]
    pub fn Foreground(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Media"
    ))]
    pub fn SetForeground<'a, Param0: ::windows::core::IntoParam<'a, super::Media::Brush>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).20)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).22)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn TextDecorations(&self) -> ::windows::core::Result<::windows::UI::Text::TextDecorations> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::TextDecorations = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::TextDecorations>(result__)
        }
    }
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
    pub fn ContentStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
    pub fn ContentEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
    pub fn ElementStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Media"
    ))]
    pub fn ElementEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).31)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn AccessKey(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).33)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetAccessKey<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).34)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).35)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).36)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).37)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).38)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn AccessKeyScopeOwner(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).39)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetAccessKeyScopeOwner<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).40)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::core::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::Input::KeyTipPlacementMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).41)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).42)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).43)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).44)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).45)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).46)(::core::mem::transmute_copy(this), value)
                .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn XamlRoot(&self) -> ::windows::core::Result<super::XamlRoot> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).47)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::XamlRoot>(result__)
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn SetXamlRoot<'a, Param0: ::windows::core::IntoParam<'a, super::XamlRoot>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).48)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).49)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyDisplayRequested<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).50)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).51)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).52)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
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
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
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
            (::windows::core::Interface::vtable(this).53)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyInvoked<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).54)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FindName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).55)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for Underline {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.Underline;{68aaec6e-ea71-5ed2-b83e-91684b25efb9})",
    );
}
unsafe impl ::windows::core::Interface for Underline {
    type Vtable = IUnderline_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x68aaec6e_ea71_5ed2_b83e_91684b25efb9);
}
impl ::windows::core::RuntimeName for Underline {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Underline";
}
impl ::core::convert::From<Underline> for ::windows::core::IUnknown {
    fn from(value: Underline) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Underline> for ::windows::core::IUnknown {
    fn from(value: &Underline) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Underline {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Underline {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Underline> for ::windows::core::IInspectable {
    fn from(value: Underline) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Underline> for ::windows::core::IInspectable {
    fn from(value: &Underline) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Underline {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Underline {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<Underline> for Span {
    fn from(value: Underline) -> Self {
        ::core::convert::Into::<Span>::into(&value)
    }
}
impl ::core::convert::From<&Underline> for Span {
    fn from(value: &Underline) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Span> for Underline {
    fn into_param(self) -> ::windows::core::Param<'a, Span> {
        ::windows::core::Param::Owned(::core::convert::Into::<Span>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Span> for &Underline {
    fn into_param(self) -> ::windows::core::Param<'a, Span> {
        ::windows::core::Param::Owned(::core::convert::Into::<Span>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<Underline> for Inline {
    fn from(value: Underline) -> Self {
        ::core::convert::Into::<Inline>::into(&value)
    }
}
impl ::core::convert::From<&Underline> for Inline {
    fn from(value: &Underline) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Inline> for Underline {
    fn into_param(self) -> ::windows::core::Param<'a, Inline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Inline>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Inline> for &Underline {
    fn into_param(self) -> ::windows::core::Param<'a, Inline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Inline>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<Underline> for TextElement {
    fn from(value: Underline) -> Self {
        ::core::convert::Into::<TextElement>::into(&value)
    }
}
impl ::core::convert::From<&Underline> for TextElement {
    fn from(value: &Underline) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for Underline {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<TextElement>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for &Underline {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<TextElement>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<Underline> for super::DependencyObject {
    fn from(value: Underline) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&Underline> for super::DependencyObject {
    fn from(value: &Underline) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for Underline {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &Underline {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for Underline {}
unsafe impl ::core::marker::Sync for Underline {}
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct UnderlineStyle(pub i32);
impl UnderlineStyle {
    pub const None: UnderlineStyle = UnderlineStyle(0i32);
    pub const Single: UnderlineStyle = UnderlineStyle(1i32);
}
impl ::core::convert::From<i32> for UnderlineStyle {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for UnderlineStyle {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for UnderlineStyle {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Documents.UnderlineStyle;i4)",
    );
}
impl ::windows::core::DefaultType for UnderlineStyle {
    type DefaultType = Self;
}
