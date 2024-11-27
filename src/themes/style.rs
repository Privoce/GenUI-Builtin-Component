use makepad_widgets::*;

live_design! {
    link gen_theme;
    // globals -----------------------------------------------------
    // -------- colors ---------------------------------------------
    // each theme color has [25, 50, 100, 200, 300, 400, 500, 600, 700, 800, 900]
    // the default color is 500
    pub COLOR_WHITE = #FFFFFF;
    pub COLOR_BLACK = #000000;
    // -------- dark-opacity ---------------------------------------
    pub DARK_OPACITY_25 = #66666640;
    pub DARK_OPACITY_50 = #66666680;
    pub DARK_OPACITY_75 = #666666BF;
    // -------- color-info -----------------------------------------
    pub COLOR_INFO_25 = #FCFCFD;
    pub COLOR_INFO_50 = #F9FAFB;
    pub COLOR_INFO_100 = #F2F4F7;
    pub COLOR_INFO_200 = #EAECF0;
    pub COLOR_INFO_300 = #D0D5DD;
    pub COLOR_INFO_400 = #95A2D3;
    pub COLOR_INFO_500 = #667085;
    pub COLOR_INFO_600 = #475467;
    pub COLOR_INFO_700 = #344054;
    pub COLOR_INFO_800 = #1D2939;
    pub COLOR_INFO_900 = #101828;
    // -------- color-dark -----------------------------------------
    pub COLOR_DARK_25 = #6e7176;
    pub COLOR_DARK_50 = #5b5f64;
    pub COLOR_DARK_100 = #42464d;
    pub COLOR_DARK_200 = #3b4047;
    pub COLOR_DARK_300 = #2f333b;
    pub COLOR_DARK_400 = #282d35;
    pub COLOR_DARK_500 = #22272F;
    pub COLOR_DARK_600 = #1f242b;
    pub COLOR_DARK_700 = #1d2127;
    pub COLOR_DARK_800 = #1a1e24;
    pub COLOR_DARK_900 = #0f1115;
    // -------- color-primary --------------------------------------
    pub COLOR_PRIMARY_25 = #F5FEFF;
    pub COLOR_PRIMARY_50 = #ECFDFF;
    pub COLOR_PRIMARY_100 = #CFF9FE;
    pub COLOR_PRIMARY_200 = #A5F0FC;
    pub COLOR_PRIMARY_300 = #67E3F9;
    pub COLOR_PRIMARY_400 = #22CCEE;
    pub COLOR_PRIMARY_500 = #06AED4;
    pub COLOR_PRIMARY_600 = #088AB2;
    pub COLOR_PRIMARY_700 = #0E6F90;
    pub COLOR_PRIMARY_800 = #155B75;
    pub COLOR_PRIMARY_900 = #164C63;
    // -------- color-error ------------------------------------
    pub COLOR_ERROR_25 = #FFFBFA;
    pub COLOR_ERROR_50 = #FEF3F2;
    pub COLOR_ERROR_100 = #FEE4E2;
    pub COLOR_ERROR_200 = #FECDCA;
    pub COLOR_ERROR_300 = #FDA29B;
    pub COLOR_ERROR_400 = #F97066;
    pub COLOR_ERROR_500 = #F04438;
    pub COLOR_ERROR_600 = #D92D2D;
    pub COLOR_ERROR_700 = #B42318;
    pub COLOR_ERROR_800 = #912018;
    pub COLOR_ERROR_900 = #7A271A;
    // -------- color-warning ------------------------------------
    pub COLOR_WARNING_25 = #FFFCF5;
    pub COLOR_WARNING_50 = #FFFAEB;
    pub COLOR_WARNING_100 = #FEF0C7;
    pub COLOR_WARNING_200 = #FEDF89;
    pub COLOR_WARNING_300 = #FEC84B;
    pub COLOR_WARNING_400 = #FDB022;
    pub COLOR_WARNING_500 = #F79009;
    pub COLOR_WARNING_600 = #DC6803;
    pub COLOR_WARNING_700 = #B54708;
    pub COLOR_WARNING_800 = #93370D;
    pub COLOR_WARNING_900 = #7A2E0E;
    // -------- color-success ------------------------------------
    pub COLOR_SUCCESS_25 = #F6FEF9;
    pub COLOR_SUCCESS_50 = #ECFDF3;
    pub COLOR_SUCCESS_100 = #D1FADF;
    pub COLOR_SUCCESS_200 = #A6F4C5;
    pub COLOR_SUCCESS_300 = #6CE9A6;
    pub COLOR_SUCCESS_400 = #32D583;
    pub COLOR_SUCCESS_500 = #12B76A;
    pub COLOR_SUCCESS_600 = #039855;
    pub COLOR_SUCCESS_700 = #027A48;
    pub COLOR_SUCCESS_800 = #05603A;
    pub COLOR_SUCCESS_900 = #054F31;
    // -------- font-family ------------------------------------
    pub FONT_FAMILY = dep("crate://self/resources/font/OPPOSans-Regular.ttf");
    pub FONT_SIZE = 10.0;
    pub FONT_SIZE_SMALL = 9.0;
    // padding -----------------------------------------------------
    pub GLOBAL_PADDING = {top: 10.0, left: 16.0, bottom: 10.0, right: 16.0};
    pub GLOBAL_PADDING_SMALL = {top: 4.6, left: 9.0, bottom: 4.6, right: 9.0};
    // align -------------------------------------------------------
    pub ALIGN_CENTER_WALK = {x: 0.5, y: 0.5};
    pub ALIGN_LEFT_WALK = {x: 0.0, y: 0.5};
    pub GLOBAL_DURATION = 0.25;
}
