use makepad_widgets::*;

use crate::{
    component,
    components::{
        button::GButton, card::GCard, label::GLabel, radio::GRadio, svg::GSvg, view::GView,
    },
};

pub mod button;
pub mod card;
pub mod checkbox;
pub mod collapse;
pub mod divider;
pub mod drop_down;
pub mod image;
pub mod label;
pub mod lifecycle;
pub mod link;
pub mod live_props;
pub mod loading;
pub mod menu;
pub mod popup;
pub mod radio;
pub mod router;
pub mod select;
pub mod svg;
pub mod switch;
pub mod tabbar;
pub mod tag;
pub mod traits;
pub mod view;

use traits::Component;

live_design! {
    link gen_ui;

    // use link::luna_theme::*;
    use link::shaders::*;
    use link::genui_basic::*;
    use link::theme::*;

    pub GLabel = <GLabelBase>{}

    pub GView = <GViewBase>{
        animation_open: false,
        event_open: false,
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

    pub GTag = <GTagBase> {}

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
        }
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
        text: <GLabel> {
            text: "Menu Item"
        }
    }
}

pub fn components_register(cx: &mut Cx) {
    label::live_design(cx);
    view::live_design(cx);
    button::live_design(cx);
    card::live_design(cx);
    radio::live_design(cx);
    radio::group::live_design(cx);
    checkbox::live_design(cx);
    checkbox::group::live_design(cx);
    switch::live_design(cx);
    divider::live_design(cx);
    svg::live_design(cx);
    image::live_design(cx);
    popup::live_design(cx);
    popup::container::live_design(cx);
    drop_down::live_design(cx);
    tabbar::live_design(cx);
    tabbar::item::live_design(cx);
    // tabbar::virt::live_design(cx); TODO: not compeleted yet
    tag::live_design(cx);
    link::live_design(cx);
    router::page::live_design(cx);
    router::live_design(cx);
    menu::item::live_design(cx);
}

component! {
    Label => GLabel,
    View => GView,
    Button => GButton,
    Card => GCard,
    Radio => GRadio,
    Svg => GSvg
}
