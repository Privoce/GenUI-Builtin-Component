use makepad_widgets::*;

mod breadcrumb;
mod button;
mod checkbox;
mod collapse;
mod colors;
mod divider;
mod drop_down;
mod file_upload;
mod icon;
mod image;
mod input;
mod label;
mod link;
mod loading;
mod menu;
mod notification;
mod popup;
mod progress;
mod radio;
pub mod register;
mod router;
mod select;
mod shader;
mod svg;
mod tab;
mod tabbar;
mod table;
mod tag;
mod toggle;
mod tool_btn;
mod view;
mod window;

pub use breadcrumb::*;
pub use button::*;
pub use checkbox::*;
pub use collapse::*;
pub use colors::*;
pub use divider::*;
pub use drop_down::*;
pub use file_upload::*;
pub use icon::*;
pub use image::*;
pub use input::*;
pub use label::*;
pub use link::*;
pub use loading::*;
pub use menu::*;
pub use popup::*;
pub use progress::*;
pub use radio::*;
pub use router::*;
pub use select::*;
pub use shader::*;
pub use svg::*;
pub use tab::*;
pub use tabbar::*;
pub use table::*;
pub use tag::*;
pub use toggle::*;
pub use tool_btn::*;
pub use view::*;
pub use window::*;

live_design! {
    link gen_components;
    use link::gen_theme::*;
    use link::gen_base::*;
    use link::widgets::*;
    use link::shaders::*;
    // components ------------------------------------------------------------------------------------------------------------
    pub GLabel = <GLabelBase>{}
    pub GLink = <GLinkBase>{}
    pub GButton = <GButtonBase>{
        slot: <GLabel>{
            text: "GButton"
        }
    }
    pub GView = <GViewBase>{}
    // ## GHLayout
    // A horizontal layout component use ViewBase
    // layout don't have border, background color, border-radius, ... (but you can add if you want)
    pub GHLayout = <GViewBase>{
        flow: Right,
        padding: 0,
        border_radius: 0,
        border_width: 0,
        background_visible: false,
        spacing: 0,
        margin: 0,
    }
    // ## GVLayout
    // A vertical layout component use ViewBase
    pub GVLayout = <GViewBase>{
        flow: Down,
        padding: 0,
        border_radius: 0,
        border_width: 0,
        background_visible: false,
        spacing: 0,
        margin: 0,
    }
    pub GColor = <GColorBase>{
        item: <GView>{
            height: 40.0,
            width: 40.0,
            border_radius: 0.0,
        },
        header: <GVLayout>{
            height: 72.0,
            spacing: 8.0,
            theme_name = <GLabel>{
                font_size: 12.0,
            }
            theme_main = <GLabel>{
                font_size: 10.0,
            }
            padding: 16.0,
        }
        colors: <GHLayout>{
            height: 40.0,
            border_radius: 0.0,
        }
    }
    // ## GScrollBar
    // A scroll bar component use ScrollBarBase, it is a single scroll bar
    pub GScrollBar = <ScrollBarBase> {
        bar_size: 10.0,
        bar_side_margin: 3.0
        min_handle_size: 20.0
        draw_bar: {
            instance pressed: 0.0
            instance hover: 0.0

            instance color: (DARK_OPACITY_50)
            instance color_hover: (DARK_OPACITY_25)
            instance color_pressed: (DARK_OPACITY_75)

            uniform bar_width: 6.0
            uniform border_radius: 1.5

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                if self.is_vertical > 0.5 {
                    sdf.box(
                        1.,
                        self.rect_size.y * self.norm_scroll,
                        self.bar_width,
                        self.rect_size.y * self.norm_handle,
                        self.border_radius
                    );
                }
                else {
                    sdf.box(
                        self.rect_size.x * self.norm_scroll,
                        1.,
                        self.rect_size.x * self.norm_handle,
                        self.bar_width,
                        self.border_radius
                    );
                }
                return sdf.fill(mix(
                    self.color,
                    mix(
                        self.color_hover,
                        self.color_pressed,
                        self.pressed
                    ),
                    self.hover
                ));
            }
        }
        animator: {
            hover = {
                default: off
                off = {
                    from: {all: Forward {duration: 0.1}}
                    apply: {
                        draw_bar: {pressed: 0.0, hover: 0.0}
                    }
                }

                on = {
                    cursor: Default,
                    from: {
                        all: Forward {duration: 0.1}
                        pressed: Forward {duration: 0.01}
                    }
                    apply: {
                        draw_bar: {
                            pressed: 0.0,
                            hover: [{time: 0.0, value: 1.0}],
                        }
                    }
                }

                pressed = {
                    cursor: Default,
                    from: {all: Snap}
                    apply: {
                        draw_bar: {
                            pressed: 1.0,
                            hover: 1.0,
                        }
                    }
                }
            }
        }
    }
    // ## GScrollBars
    // A scroll bars component use ScrollBarsBase, it has two scroll bars (x, y)
    // It often use in a View
    pub GScrollBars = <ScrollBarsBase> {
        show_scroll_x: true,
        show_scroll_y: true,
        scroll_bar_x: <GScrollBar> {}
        scroll_bar_y: <GScrollBar> {}
    }
    pub GRadio = <GRadioBase>{}
    pub GRadioGroup = <GRadioGroupBase>{}
    pub GCheckbox = <GCheckboxBase>{}
    pub GCheckboxGroup = <GCheckboxGroupBase>{}
    pub GToggle = <GToggleBase>{}
    pub GSvg = <GSvgBase>{}
    pub GIcon = <GIconBase>{}
    pub GToolButton = <GToolButtonBase>{}
    pub GImage = <GImageBase>{}
    pub GInput = <GInputBase>{}
    pub GShader = <GShaderBase>{}
    pub GDivider = <GDividerBase>{}
    pub GPopupContainer = <GPopupContainerBase>{}
    pub GPopup = <GPopupBase>{
        height: Fill,
        width: Fill,
        mode: Popup,
        container: <GPopupContainer>{}
    }
    pub GToolTip = <GPopup>{
        mode: ToolTip,
        draw_popup: {
            instance inset: vec4(0.0, 0.0, 0.0, 0.0);

            fn pixel(self) -> vec4{
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let spacing = 6.0;
                let w = self.rect_size.x - (self.inset.x + self.inset.z + self.border_width * 2.0);
                let h = self.rect_size.y - (self.inset.y + self.inset.w + self.border_width * 2.0);
                let center = vec2((self.pos.x + w) * 0.5, (self.pos.y + h)  * 0.5);
                let start_p = vec2(self.inset.x + self.border_width, self.inset.y + self.border_width);
                // let quarter_w = w * 0.25;
                let quarter_w = self.angle_offset;
                // let quarter_h = h * 0.25;
                let quarter_h = self.angle_offset;
                match self.position{
                    Position::Left => {
                        let end_w = self.inset.x + self.border_width + w - spacing;
                        sdf.box(
                            start_p.x,
                            start_p.y,
                            w - spacing,
                            h,
                            max(1.0, self.border_radius)
                        );

                        sdf.move_to(end_w - 0.4, center.y - spacing * 0.7);
                        sdf.line_to(end_w + spacing - 0.4, center.y);
                        sdf.line_to(end_w - 0.4, center.y + spacing * 0.7);
                        sdf.close_path();
                    }
                    Position::LeftTop => {
                        let end_w = self.inset.x + self.border_width + w - spacing;
                        sdf.box(
                            start_p.x,
                            start_p.y,
                            w - spacing,
                            h,
                            max(1.0, self.border_radius)
                        );
                        sdf.move_to(end_w - 0.4, start_p.y + quarter_h - spacing * 0.7);
                        sdf.line_to(end_w + spacing - 0.4, start_p.y + quarter_h);
                        sdf.line_to(end_w - 0.4, start_p.y + quarter_h + spacing * 0.7);
                        sdf.close_path();
                    }
                    Position::LeftBottom => {
                        let end_w = self.inset.x + self.border_width + w - spacing;
                        sdf.box(
                            start_p.x,
                            start_p.y,
                            w - spacing,
                            h,
                            max(1.0, self.border_radius)
                        );
                        sdf.move_to(end_w - 0.4, start_p.y + h - quarter_h - spacing * 0.7);
                        sdf.line_to(end_w + spacing - 0.4, start_p.y + h - quarter_h);
                        sdf.line_to(end_w - 0.4, start_p.y + h - quarter_h + spacing * 0.7);
                        sdf.close_path();
                    }
                    Position::Right => {
                        let start_w = self.inset.x + self.border_width + spacing;
                        sdf.box(
                            self.inset.x + self.border_width + spacing,
                            self.inset.y + self.border_width,
                            w - spacing,
                            h,
                            max(1.0, self.border_radius)
                        );

                        sdf.move_to(start_w - spacing, center.y);
                        sdf.line_to(start_w + 0.4, center.y - spacing * 0.7);
                        sdf.line_to(start_w + 0.4, center.y + spacing * 0.7);
                        sdf.close_path();
                    }
                    Position::RightTop => {
                        let start_w = self.inset.x + self.border_width + spacing;
                        sdf.box(
                            self.inset.x + self.border_width + spacing,
                            self.inset.y + self.border_width,
                            w - spacing,
                            h,
                            max(1.0, self.border_radius)
                        );
                        sdf.move_to(start_w - spacing, start_p.y + quarter_h);
                        sdf.line_to(start_w + 0.4, start_p.y + quarter_h - spacing * 0.7);
                        sdf.line_to(start_w + 0.4, start_p.y + quarter_h + spacing * 0.7);
                        sdf.close_path();
                    }
                    Position::RightBottom => {
                        let start_w = self.inset.x + self.border_width + spacing;
                        sdf.box(
                            self.inset.x + self.border_width + spacing,
                            self.inset.y + self.border_width,
                            w - spacing,
                            h,
                            max(1.0, self.border_radius)
                        );
                        sdf.move_to(start_w - spacing, start_p.y + h - quarter_h);
                        sdf.line_to(start_w + 0.4, start_p.y + h - quarter_h - spacing * 0.7);
                        sdf.line_to(start_w + 0.4, start_p.y + h - quarter_h + spacing * 0.7);
                        sdf.close_path();
                    }
                    Position::Top => {
                        let end_h =  self.inset.y + self.border_width + h - spacing;
                        sdf.box(
                            self.inset.x + self.border_width,
                            self.inset.y + self.border_width,
                            w,
                            h - spacing,
                            max(1.0, self.border_radius)
                        );
                        sdf.move_to(center.x, end_h + spacing - 0.4);
                        sdf.line_to(center.x - spacing * 0.7, end_h - 0.4);
                        sdf.line_to(center.x + spacing * 0.7, end_h - 0.4);
                        sdf.close_path();

                    }
                    Position::TopLeft => {
                        let end_h =  self.inset.y + self.border_width + h - spacing;
                        sdf.box(
                            self.inset.x + self.border_width,
                            self.inset.y + self.border_width,
                            w,
                            h - spacing,
                            max(1.0, self.border_radius)
                        );
                        sdf.move_to(start_p.x + quarter_w, end_h + spacing - 0.4);
                        sdf.line_to(start_p.x + quarter_w - spacing * 0.7, end_h - 0.4);
                        sdf.line_to(start_p.x + quarter_w + spacing * 0.7, end_h - 0.4);
                        sdf.close_path();
                    }
                    Position::TopRight => {
                        let end_h =  self.inset.y + self.border_width + h - spacing;
                        sdf.box(
                            self.inset.x + self.border_width,
                            self.inset.y + self.border_width,
                            w,
                            h - spacing,
                            max(1.0, self.border_radius)
                        );
                        sdf.move_to(start_p.x + w - quarter_w, end_h + spacing - 0.4);
                        sdf.line_to(start_p.x + w - quarter_w - spacing * 0.7, end_h - 0.4);
                        sdf.line_to(start_p.x + w - quarter_w + spacing * 0.7, end_h - 0.4);
                        sdf.close_path();
                    }
                    Position::Bottom => {
                        sdf.box(
                            self.inset.x + self.border_width,
                            self.inset.y + self.border_width + spacing,
                            w,
                            h - spacing,
                            max(1.0, self.border_radius)
                        );

                        sdf.move_to(center.x - spacing * 0.7, spacing + 0.4);
                        sdf.line_to(center.x, self.pos.y);
                        sdf.line_to(center.x + spacing * 0.7, spacing + 0.4);
                        sdf.line_to(center.x - spacing * 0.7, spacing + 0.4);
                    }
                    Position::BottomLeft => {
                        sdf.box(
                            self.inset.x + self.border_width,
                            self.inset.y + self.border_width + spacing,
                            w,
                            h - spacing,
                            max(1.0, self.border_radius)
                        );
                        sdf.move_to(start_p.x + quarter_w - spacing * 0.7, spacing + 0.4);
                        sdf.line_to(start_p.x + quarter_w, self.pos.y);
                        sdf.line_to(start_p.x + quarter_w + spacing * 0.7, spacing + 0.4);
                        sdf.line_to(start_p.x + quarter_w - spacing * 0.7, spacing + 0.4);
                    }
                    Position::BottomRight => {
                        sdf.box(
                            self.inset.x + self.border_width,
                            self.inset.y + self.border_width + spacing,
                            w,
                            h - spacing,
                            max(1.0, self.border_radius)
                        );
                        sdf.move_to(start_p.x + w - quarter_w - spacing * 0.7, spacing + 0.4);
                        sdf.line_to(start_p.x + w - quarter_w, self.pos.y);
                        sdf.line_to(start_p.x + w - quarter_w + spacing * 0.7, spacing + 0.4);
                        sdf.line_to(start_p.x + w - quarter_w - spacing * 0.7, spacing + 0.4);
                    }
                }
                if self.background_visible == 1.0 {
                    sdf.fill(self.get_background_color());
                }
                sdf.stroke(self.get_border_color(), self.border_width);
                return sdf.result;
            }
        }
        container: <GPopupContainer>{
            background_visible: false,
        }
    }
    pub GDialog = <GPopup>{
        align: <ALIGN_CENTER_WALK>{}
        mode: Dialog,
        height: All,
        width: All,
        theme: Dark,
        draw_popup: {
            // this is a mask
            fn pixel(self) -> vec4{
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.rect(self.pos.x, self.pos.y, self.rect_size.x, self.rect_size.y);
                let color = self.get_background_color();
                sdf.fill(vec4(color.r, color.g, color.b, self.opacity));
                return sdf.result;
            }
        }
        container: <GPopupContainer>{
            clip_x: false,
            clip_y: false,
            background_visible: false,
        }
    }
    pub GDrawer = <GPopup>{
        // align: <ALIGN_CENTER_WALK>{},
        mode: Drawer,
        height: All,
        width: All,
        theme: Dark,
        draw_popup: {
            // this is a mask
            fn pixel(self) -> vec4{
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.rect(self.pos.x, self.pos.y, self.rect_size.x, self.rect_size.y);
                let color = self.get_background_color();
                sdf.fill(vec4(color.r, color.g, color.b, self.opacity));

                return sdf.result;
            }
        }
        container: <GPopupContainer>{
            theme: Dark,
            clip_x: false,
            clip_y: false,
            background_visible: true,
            border_radius: 0.0
        }
    }
    pub GDropDown = <GDropDownBase>{
        height: Fit,
        width: Fit,
        padding: 0,
        border_radius: 0,
        border_width: 0,
        background_visible: false,
        spacing: 0,
        margin: 0,
        popup: <GPopup> {
            height: 160.0,
            width: 200.0,
        },
    }
    pub GProgress = <GProgressBase>{}
    pub GLoading = <GLoadingBase>{}
    pub GState404 = <GImage>{
        height: Fill,
        src: Live(dep("crate://self/resources/icons/404.png")),
        fit: Vertical
    }
    pub GState502 = <GImage>{
        height: Fill,
        src: Live(dep("crate://self/resources/icons/502.png")),
        fit: Vertical
    }
    pub GStateNoData = <GImage>{
        height: Fill,
        src: Live(dep("crate://self/resources/icons/no_data.png")),
        fit: Vertical
    }
    pub GStateNoMsg = <GImage>{
        height: Fill,
        src: Live(dep("crate://self/resources/icons/no_msg.png")),
        fit: Vertical
    }
    pub GStateNetWorkErr = <GImage>{
        height: Fill,
        src: Live(dep("crate://self/resources/icons/network_err.png")),
        fit: Vertical
    }
    pub GStateSearch = <GImage>{
        height: Fill,
        src: Live(dep("crate://self/resources/icons/searching.png")),
        fit: Vertical
    }
    pub GTag = <GTagBase>{}
    pub GBreadCrumbItem = <GBreadCrumbItemBase>{}
    pub GBreadCrumb = <GBreadCrumbBase>{
        icon = <GSvg>{
            src: dep("crate://self/resources/icons/home.svg"),
            cursor: Hand,
            animation_key: true,
            height: 16.0,
            width: 16.0,
            event_key: true,
        },
        item: <GBreadCrumbItem>{
            theme: Dark,
        }
    }
    pub GTabButton = <GTabButtonBase>{}
    pub GTabHeader = <GTabHeaderBase>{
        scroll_bars: <GScrollBars>{
            show_scroll_x: true
            show_scroll_y: false
            scroll_bar_x: {
                draw_bar: {bar_width: 3.0}
                bar_size: 4
                use_vertical_finger_scroll: true
            }
        },
        item: <GTabButton>{}
    }
    pub GTabBody = <GTabBodyBase>{}
    pub GTabPane = <GTabPaneBase>{}
    pub GTab = <GTabBase>{
        header: <GTabHeader>{
            margin: {top: 0.0, left: 0.0, bottom: 0.0, right: 0.0},
        },
        body: <GTabPane>{
            // clip_x: true,
            // clip_y: true,
        },
    }
    pub GSplitter = <Splitter>{
        draw_splitter: {
            uniform border_radius: 1.0
            uniform splitter_pad: 1.0
            uniform splitter_grabber: 60.0

            instance pressed: 0.0
            instance hover: 0.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.clear(#DDDDDD10);

                if self.is_vertical > 0.5 {
                    sdf.box(
                        self.splitter_pad,
                        self.rect_size.y * 0.5 - self.splitter_grabber * 0.5,
                        self.rect_size.x - 2.0 * self.splitter_pad,
                        self.splitter_grabber,
                        self.border_radius
                    );
                }
                else {
                    sdf.box(
                        self.rect_size.x * 0.5 - self.splitter_grabber * 0.5,
                        self.splitter_pad,
                        self.splitter_grabber,
                        self.rect_size.y - 2.0 * self.splitter_pad,
                        self.border_radius
                    );
                }
                return sdf.fill_keep(mix(
                    vec4(0.0),
                    mix(
                        vec4(0.6, 1.0, 1.0, 2.0),
                        vec4(1.0, 1.0, 1.0, 4.0),
                        self.pressed
                    ),
                    self.hover
                ));
            }
        }
    }
    pub GUpload = <GUploadBase>{
        icon: <GSvg>{
            src: dep("crate://self/resources/icons/cloud_upload.svg"),
            height: 32.0,
            width: 32.0,
        }
    }
    pub GCollapse = <GCollapseBase>{
        header: <GHLayout>{
            background_visible: true,
            height: Fit,
            padding: {left: 8.0, right: 8.0, top: 4.0, bottom: 4.0},
            align: <ALIGN_LEFT_WALK>{},
            spacing: 8.0,
            margin: 0.0,
            border_radius: 0.0,
        },
        body: <GVLayout>{
            background_visible: true,
            height: Fit,
            width: Fill,
            padding: {left: 8.0, right: 8.0, top: 4.0, bottom: 4.0},
            margin: 0.0,
            border_radius: 0.0,
        }
    }
    pub GTCell = <GTableCellBase>{
        align: <ALIGN_CENTER_WALK>{}
    }
    pub GTRow = <GTableRowBase>{
        width: Fill,
        height: 36.0,
        align: <ALIGN_LEFT_WALK>{}
    }
    pub GTBody = <GTableBodyBase>{
        height: Fit,
        width: Fill,
        flow: Down,
        align: {
            x: 0.0, y: 0.0
        }
    }
    pub GTHeader = <GTableHeaderBase>{}
    pub GTable = <GTableBase>{
        header: <GTHeader>{}
        body: <GTBody>{}
    }
    pub GVTBody = <GVTableBodyBase>{
        height: Fit,
        width: Fill,
        flow: Down,
        align: {
            x: 0.0, y: 0.0
        }
    }
    pub GWindow = <GWindowBase>{
        nav_control: <NavControl> {}
        window_bar = <GHLayout>{
            height: 32.0,
            width: Fill,
            background_color: #1F1E25,
            background_visible: true,
            align: {
                x: 0.0, y: 0.5
            }
            spacing: 0.0,
            mac_btns_wrap = <GHLayout>{
                visible: false
                height: 32.0,
                width: Fit,
                spacing: 6.0,
                align: {x: 0.0, y: 0.5},
                padding: {left: 6.0},
                close = <GToolButton> {icon_type: Close, os_type: Mac}
                max = <GToolButton> {icon_type: Max, os_type: Mac}
                min = <GToolButton> {icon_type: Min, os_type: Mac}
            }
            window_title = <GHLayout>{
                height: Fill,
                width: Fill,
                align: {x: 0.5, y: 0.5},
                spacing: 6.0,
                icon = <GImage>{
                    visible: false,
                    src: Live(dep("crate://self/resources/icons/logo.png")),
                    height: 16.0,
                    width: 16.0,
                },
                title = <GLabel>{
                    height: Fit,
                    text: "",
                    font_size: 9.0,
                },
            }
            sub = <GHLayout>{
                visible: false,
                height: Fill,
                width: Fit,
            }
            win_btns_wrap = <GHLayout>{
                visible: false
                height: 32.0,
                width: Fit,
                spacing: 0.0,
                min = <GToolButton> {icon_type: Min, os_type: Windows}
                max = <GToolButton> {icon_type: Max, os_type: Windows}
                close = <GToolButton> {icon_type: Close, os_type: Windows}
            }
            linux_btns_wrap = <GHLayout>{
                visible: false
                height: 32.0,
                width: Fit,
                spacing: 6.0,
                align: {x: 1.0, y: 0.5},
                padding: {right: 6.0},
                min = <GToolButton> {icon_type: Min, os_type: Linux}
                max = <GToolButton> {icon_type: Max, os_type: Linux}
                close = <GToolButton> {icon_type: Close, os_type: Linux}
            }
        }
    }
    // GSelectItem = <GSelectItemBase>{
    //     height: 36.0,
    //     width: Fill,
    //     flow: Right,
    //     spacing: 6.0,
    //     align: {
    //         x: 0.0,
    //         y: 0.5
    //     },
    //     left: <GHLayout>{
    //         align: {x: 0.5, y: 0.5},
    //         height: Fill,
    //         width: Fit,
    //         left_slot = <GView>{
    //             height: 10.0,
    //             width: 10.0,
    //             border_radius: 2.5,
    //             border_width: 0.0,
    //         }
    //     },
    //     center: <GHLayout>{
    //         align: {x: 0.0, y: 0.5},
    //         height: Fill,
    //         width: Fit,
    //         center_slot = <GLabel>{
    //             color: (COLOR_INFO_900),
    //             font_size: (FONT_SIZE),
    //             text: "Select Item"
    //         }
    //     }
    //     right: <GHLayout>{
    //         align: {x: 0.0, y: 0.5},
    //         height: Fill,
    //         width: Fit,
    //         right_slot = <GLabel>{
    //             color: (COLOR_INFO_900),
    //             font_size: (FONT_SIZE_SMALL)
    //             text: "sub info"
    //         }
    //     }
    // }
    pub GSelectItem = <GSelectItemBase>{}
    pub GSelectOptions = <GSelectOptionsBase>{
        height: 144.0,
        width: 180.0,
        padding: {left: 8.0, right: 8.0, top: 6.0, bottom: 6.0},
        flow: Down,
        scroll_bars: <GScrollBars>{
            show_scroll_x: false
            show_scroll_y: true
            // scroll_bar_y: {
            //     draw_bar: {bar_width: 4.0}
            //     bar_size: 6
            //     use_vertical_finger_scroll: true
            // }
        },
    }
    pub GSelect = <GSelectBase>{
        font_family: (FONT_FAMILY),
        flow: Down,
        select_item: <GSelectItem>{},
        select_options: <GSelectOptions>{}
    }
    pub GTabbarItem = <GTabbarItemBase>{
        icon_slot: <GSvg>{
            height: 18.0,
            width: 18.0,
            color: #161616,
            src: dep("crate://self/resources/icons/home.svg"),
            stroke_hover_color: #FF7043,
            stroke_focus_color: #FF7043,
            animation_key: true,
        }
        text_slot: <GLabel>{
            font_size: 8.0,
            color: #161616,
            animation_key: true,
            text: "Home",
            stroke_hover_color: #FF7043,
            stroke_focus_color: #FF7043,
        }
    }
    pub GTabbar = <GTabbarBase>{}
    pub GBarPage = <GView>{
        visible:false,
        height: Fill,
        width: Fill,
        background_visible: false,
        border_radius: 0.0,
    }
    pub GPage = <GPageBase>{
        background_visible: false,
        border_radius: 0.0,
        flow: Down,
        height: Fill,
        width: Fill,
        header = <GHLayout>{
            height: 24.0,
            padding: {
                left: 4.0,
                right: 4.0,
            },
            spacing: 8.0,
            align: {
                x: 0.5,
                y: 0.5
            },
            back_wrap = <GHLayout>{
                height: Fill,
                width: 24.0,
                align: {
                    x: 0.5,
                    y: 0.5,
                },
                back = <GIcon>{
                    cursor: Hand,
                    theme: Dark,
                    height: 18.0,
                    width: 18.0,
                    stroke_width: 1.2,
                    icon_type: Left,
                }
            }
            title_wrap = <GHLayout>{
                align: {
                    x: 0.5,
                    y: 0.5
                },
                title = <GLabel>{
                    text: ""
                }
            }
            tool_wrap = <GHLayout>{
                height: Fill,
                width: 24.0,
                align: {
                    x: 0.5,
                    y: 0.5,
                },
            }
        }
        body = <GView>{
            border_radius: 0.0,
            clip_x: true,
            clip_y: true,
            theme: Dark,
            height: Fill,
            width: Fill
            padding: 4.0,
        }
    }
    pub GNavPage = <GPage>{}
    pub GRouter = <GRouterBase>{
        background_visible: false,
        height: Fill,
        width: Fill,
        bar_pages = <GView>{
            height: Fill,
            width: Fill,
            border_radius: 0.0,
            background_visible: false,
            flow: Down,
        }
        nav_pages = <GView>{
            height: Fill,
            width: Fill,
            border_radius: 0.0,
            background_visible: false,
            flow: Down,
        }
    }
    pub GMenuItem = <GMenuItemBase>{
        height: 36.0,
        width: Fill,
        padding: {
            left: 12.0,
            right: 12.0,
            top: 4.0,
            bottom: 4.0
        },
        spacing: 12.0,
        align: {
            x: 0.0,
            y: 0.5
        },
        icon_slot: <GSvg>{
            theme: Dark,
            color: (COLOR_WHITE),
            src: dep("crate://self/resources/icons/home.svg"),
            height: 16.0,
            width: 16.0,
        }
        text_slot: <GLabel>{
            text: "Home",
            font_size: 10.0,
        }
        right: <GHLayout>{
            visible: false
        }
    }
    pub GSubMenu = <GSubMenuBase>{
        title: <GView>{
            padding: {
                left: 8.0,
                right: 8.0,
                top: 4.0,
                bottom: 4.0
            },
            height: 32.0,
            width: Fill,
            align: {
                x: 0.0,
                y: 0.5
            },
        },
        items: <GVLayout>{
            height: Fill,
            width: Fill,
            align: {
                x: 0.0,
                y: 0.0
            },
        }
    }
    pub GMenu = <GMenuBase>{
        height: Fill,
        width: 240.0,
        flow: Down,
        border_radius: 0.0,
        header: <GVLayout>{
            visible: false,
        }
        body: <GVLayout>{
            height: Fill,
            width: Fill,
        }
        footer: <GVLayout>{
            visible: false
        }
    }
}
