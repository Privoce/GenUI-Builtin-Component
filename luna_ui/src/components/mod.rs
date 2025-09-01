use makepad_widgets::*;

mod button;
mod card;
mod checkbox;
mod collapse;
mod divider;
mod drop_down;
mod image;
mod label;
mod lifecycle;
mod link;
mod live_props;
mod loading;
mod menu;
mod popup;
mod radio;
mod router;
mod select;
mod svg;
mod switch;
mod tabbar;
mod tag;
mod traits;
mod view;

pub use button::*;
pub use card::*;
pub use checkbox::*;
pub use collapse::*;
pub use divider::*;
pub use drop_down::*;
pub use image::*;
pub use label::*;
pub use lifecycle::*;
pub use link::*;
pub use live_props::*;
// pub use loading::*;
pub use menu::*;
pub use popup::*;
pub use radio::*;
pub use router::*;
// pub use select::*;
pub use svg::*;
pub use switch::*;
pub use tabbar::*;
pub use tag::*;
pub use traits::*;
pub use view::*;

use crate::component;

live_design! {
    link genui;

    // use link::genui_theme::*;
    use link::widgets::*;
    use link::shaders::*;
    use link::genui_basic::*;
    use link::theme::*;

    pub GLabel = <GLabelBase>{}

    pub GScrollBars = <ScrollBars>{}

    pub GView = <GViewBase>{
        animation_open: false,
        event_open: false,
        scroll_bars: <GScrollBars>{}
    }

    pub GHLayout = <GView> {
        prop: {
            basic: {
                height: Fill,
                width: Fill,
                flow: Right,
            }
        }
    }

    pub GVLayout = <GView> {
        prop: {
            basic: {
                height: Fill,
                width: Fill,
                flow: Down,
            }
        }
    }

    pub GButton = <GButtonBase>{
        slot: <GLabel> {
            text: "Button",
        },
    }

    pub GCard = <GCardBase>{
        prop: {
            basic: {
                container: {
                    height: 300.0,
                    width: 200.0,
                }
            }
        },
        animation_open: false,
        header: <GView> {
            <GLabel> {
                text: "Card Header"
            }
        }
        body: <GView> {
            <GLabel> {
                text: "Card Body"
            }
        }
        footer: <GView> {
            <GLabel> {
                text: "Card Footer"
            }
        }
    }

    pub GRadio = <GRadioBase> {
        extra: <GView> {
            <GLabel> {
                text: "Extra",
            }
        }
    }
    pub GRadioGroup = <GRadioGroupBase> {}

    pub GCheckbox = <GCheckboxBase> {
        extra: <GView> {
            <GLabel> {
                text: "Extra",
            }
        }
    }

    pub GCheckboxGroup = <GCheckboxGroupBase> {}

    pub GSwitch = <GSwitchBase> {}

    pub GDivider = <GDividerBase> {}

    pub GSvg = <GSvgBase> {}
    pub IconClose = <GSvg> {
        src: dep("crate://self/resources/icons/svg/close.svg")
    }
    pub IconDown = <GSvg> {
        src: dep("crate://self/resources/icons/svg/down.svg")
    }
    pub IconUp = <GSvg> {
        src: dep("crate://self/resources/icons/svg/up.svg")
    }
    pub IconLeft = <GSvg> {
        prop: {basic: {svg: {height: 16.0, width: Fit}}}
        src: dep("crate://self/resources/icons/svg/left.svg")
    }
    pub IconRight = <GSvg> {
        prop: {basic: {svg: {height: 16.0, width: Fit}}}
        src: dep("crate://self/resources/icons/svg/right.svg")
    }
    pub IconMore = <GSvg> {
        prop: {basic: {svg: {height: 16.0, width: Fit}}}
        src: dep("crate://self/resources/icons/svg/more.svg")
    }
    pub IconAll = <GSvg> {
        src: dep("crate://self/resources/icons/svg/all.svg")
    }
    pub IconChart = <GSvg> {
        src: dep("crate://self/resources/icons/svg/chart.svg")
    }
    pub IconCheck = <GSvg> {
        src: dep("crate://self/resources/icons/svg/check.svg")
    }
    pub IconCloudDownload = <GSvg> {
        src: dep("crate://self/resources/icons/svg/cloud_download.svg")
    }
    pub IconCloudUpload = <GSvg> {
        src: dep("crate://self/resources/icons/svg/cloud_upload.svg")
    }
    pub IconCut = <GSvg> {
        src: dep("crate://self/resources/icons/svg/cut.svg")
    }
    pub IconGallery = <GSvg> {
        src: dep("crate://self/resources/icons/svg/gallery.svg")
    }
    pub IconGift = <GSvg> {
        src: dep("crate://self/resources/icons/svg/gift.svg")
    }
    pub IconHome = <GSvg> {
        src: dep("crate://self/resources/icons/svg/home.svg")
    }
    pub IconLink = <GSvg> {
        src: dep("crate://self/resources/icons/svg/link.svg")
    }
    pub IconMinus = <GSvg> {
        src: dep("crate://self/resources/icons/svg/minus.svg")
    }
    pub IconPlus = <GSvg> {
        src: dep("crate://self/resources/icons/svg/plus.svg")
    }
    pub IconPowerOff = <GSvg> {
        src: dep("crate://self/resources/icons/svg/poweroff.svg")
    }
    // pub IconRedo = <GSvg> {
    //     src: dep("crate://self/resources/icons/svg/redo.svg")
    // }
    pub IconScatter = <GSvg> {
        src: dep("crate://self/resources/icons/svg/scatter.svg")
    }
    pub IconSearch = <GSvg> {
        src: dep("crate://self/resources/icons/svg/search.svg")
    }
    pub IconWaiting = <GSvg> {
        src: dep("crate://self/resources/icons/svg/waiting.svg")
    }
    pub GImage = <GImageBase> {}

    pub GPopup = <GPopupBase>{}

    pub GDialogPopup = <GPopup> {
        prop: {
            basic: {
                height: 300.0,
                width: 400.0,
            }
        }
    }

    pub GDrawerPopup = <GPopup> {}

    pub GPopoverPopup = <GPopup> {
        prop: {
            basic: {
                height: Fit,
                width: Fit,
            }
        }
    }

    pub GToolTipPopup = <GPopup> {
        prop: {
            basic: {
                height: Fit,
                width: Fit,
            }
        }
    }

    pub GPopupContainer = <GPopupContainerBase>{
        popup: <GPopup> {}
    }

    pub GDropDown = <GDropDownBase>{
        popup: <GPopupContainer>{}
    }

    pub GDialogContainer = <GPopupContainer>{
        prop: {
            basic: {
                height: All,
                width: All,
                align: {
                    x: 0.5,
                    y: 0.5,
                },
                background_visible: true,
            }
        },
        popup: <GDialogPopup> {},
        draw_popup_container: {
            // this is a mask
            fn pixel(self) -> vec4{
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.rect(self.pos.x, self.pos.y, self.rect_size.x, self.rect_size.y);
                let color = self.background_color;
                // sdf.fill(vec4(color.r, color.g, color.b, self.opacity));
                sdf.fill(color);
                return sdf.result;
            }
        }
    }

    pub GDrawerContainer = <GPopupContainer> {
        popup: <GDrawerPopup> {},
    }

    pub GPopoverContainer = <GPopupContainer> {
        prop: {
            basic: {
                height: Fit,
                width: Fit,
                background_visible: false,
            }
        },
        popup: <GPopoverPopup> {
            <GLabel> {
                text: "Popover",
            }
        },
    }

    pub GTooltipContainer = <GPopupContainer> {
        prop: {
            basic: {
                height: Fit,
                width: Fit,
                background_visible: false,
            }
        },
        popup: <GToolTipPopup> {
            <GLabel> {
                text: "Tooltip",
            }
        },
    }

    pub GDialog = <GDropDownBase>{
        mode: Dialog,
        popup: <GDialogContainer>{}
    }

    pub GDrawer = <GDropDownBase> {
        mode: Drawer,
        popup: <GDrawerContainer> {}
    }

    pub GPopover = <GDropDownBase> {
        mode: Popover,
        popup: <GPopoverContainer> {}
    }

    pub GToolTip = <GDropDownBase> {
        mode: ToolTip,
        popup: <GTooltipContainer> {}
    }

    pub GTabbarItem = <GTabbarItemBase> {}

    pub GTabbar = <GTabbarBase> {}

    // pub GVTabbar = <GVTabbarBase> {
    //     item: <GTabbarItem> {}
    // }

    pub GTag = <GTagBase> {
        icon: <GSvg>{
            prop: {basic: {svg: {width: 12.0}}}
            visible: false
        },
        text: <GLabel>{
            text: "GTag"
        },
        close: <IconClose>{
            visible: false,
            prop: {basic: {svg: {width: 12.0}}}
        }
    }

    pub GLink = <GLinkBase> {}

    pub GPage = <GPageBase> {}

    pub GNavPage = <GPage> {
        header = <GView> {
            prop: {
                basic: {
                    height: Fit,
                    width: Fill,
                    flow: Right,
                    background_visible: false,
                    border_radius: {left: 0.0, top: 0.0, right: 0.0, bottom: 0.0},
                    margin: {left: 0.0, top: 0.0, right: 0.0, bottom: 0.0},
                    padding: {left: 0.0, top: 0.0, right: 0.0, bottom: 0.0},
                }
            }
            back_icon = <GSvg> {
                prop: {
                    basic: {
                        svg: {
                            height: 18.0,
                            width: Fit,
                        },
                        container: {
                            height: 24.0,
                            width: 24.0,
                            cursor: Hand
                        }
                    }
                }
                src: dep("crate://self/resources/icons/svg/left.svg")
            }
            title_wrap = <GView> {
                prop: {
                    basic: {
                        height: 24.0,
                        width: Fill,
                        margin: {left: 0.0, top: 0.0, right: 8.0, bottom: 0.0},
                        align: {x: 0.5, y: 0.5},
                        padding: {left: 0.0, top: 0.0, right: 8.0, bottom: 0.0},
                    }
                },
                title = <GLabel> {
                    text: "Page Title",
                    mode: Bold
                }
            }
            extra_icon = <GSvg> {
                prop: {
                    basic: {
                        svg: {
                            height: 18.0,
                            width: Fit,
                        },
                        container: {
                            height: 24.0,
                            width: 24.0,
                            cursor: Hand
                        }
                    }
                }
                src: dep("crate://self/resources/icons/svg/more.svg")
            }
        }
    }
    pub GBarPage = <GView> {
        prop: {
            basic: {
                height: Fill,
                width: Fill,
                flow: Down
            }
        },
        visible: false
    }

    pub GRouter = <GRouterBase> {
        bar_pages = <GView>{
            prop: {
                basic: {
                    height: Fill,
                    width: Fill,
                    border_radius: {left: 0.0, top: 0.0, right: 0.0, bottom: 0.0},
                    background_visible: false,
                    flow: Down,
                }
            }
        }
        nav_pages = <GView>{
            prop: {
                basic: {
                    height: Fill,
                    width: Fill,
                    border_radius: {left: 0.0, top: 0.0, right: 0.0, bottom: 0.0},
                    background_visible: false,
                    flow: Down,
                }
            }
        }
    }

    pub GMenuItem = <GMenuItemBase> {
        icon: <GSvg> {
            visible: false
        }
        text: <GLabel> {
            text: "Menu Item"
        }
    }

    pub GSubMenu = <GSubMenuBase> {
        header: <GView> {
            <GLabel>{
                text: "Sub Menu",
            }
        }
        body: <GView> {}
    }

    pub GMenu = <GMenuBase> {
        header: <GView> {
            visible: false,
            <GLabel>{
                text: "Menu Header",
            }
        },
        body: <GView>{
            scroll_bars: <GScrollBars> {}
        },
        footer: <GView> {
            visible: false,
            <GLabel>{
                text: "Menu Footer",
            }
        }
    }

    pub GCollapse = <GCollapseBase> {}
}

pub fn components_register(cx: &mut Cx) {
    svg::live_design(cx);
    label::live_design(cx);
    view::live_design(cx);
    button::live_design(cx);
    card::live_design(cx);
    radio::radio_register(cx);
    checkbox::checkbox_register(cx);
    switch::live_design(cx);
    divider::live_design(cx);
    image::live_design(cx);
    popup::popup_register(cx);
    drop_down::live_design(cx);
    tabbar::tabbar_register(cx);
    tag::live_design(cx);
    link::live_design(cx);
    router::page::live_design(cx);
    router::live_design(cx);
    menu::menu_register(cx);
    collapse::live_design(cx);
}

component! {
    Label => GLabel,
    View => GView,
    Button => GButton,
    Card => GCard,
    Radio => GRadio,
    Svg => GSvg
}
