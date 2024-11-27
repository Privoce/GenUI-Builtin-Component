use makepad_widgets::*;

live_design! {
    // globals -----------------------------------------------------
    // -------- colors ---------------------------------------------
    // each theme color has [25, 50, 100, 200, 300, 400, 500, 600, 700, 800, 900]
    // the default color is 500
    COLOR_WHITE = #FFFFFF;
    COLOR_BLACK = #000000;
    // -------- dark-opacity ---------------------------------------
    DARK_OPACITY_25 = #66666640;
    DARK_OPACITY_50 = #66666680;
    DARK_OPACITY_75 = #666666BF;
    // -------- color-info -----------------------------------------
    COLOR_INFO_25 = #FCFCFD;
    COLOR_INFO_50 = #F9FAFB;
    COLOR_INFO_100 = #F2F4F7;
    COLOR_INFO_200 = #EAECF0;
    COLOR_INFO_300 = #D0D5DD;
    COLOR_INFO_400 = #95A2D3;
    COLOR_INFO_500 = #667085;
    COLOR_INFO_600 = #475467;
    COLOR_INFO_700 = #344054;
    COLOR_INFO_800 = #1D2939;
    COLOR_INFO_900 = #101828;
    // -------- color-dark -----------------------------------------
    COLOR_DARK_25 = #6e7176;
    COLOR_DARK_50 = #5b5f64;
    COLOR_DARK_100 = #42464d;
    COLOR_DARK_200 = #3b4047;
    COLOR_DARK_300 = #2f333b;
    COLOR_DARK_400 = #282d35;
    COLOR_DARK_500 = #22272F;
    COLOR_DARK_600 = #1f242b;
    COLOR_DARK_700 = #1d2127;
    COLOR_DARK_800 = #1a1e24;
    COLOR_DARK_900 = #0f1115;
    // -------- color-primary --------------------------------------
    COLOR_PRIMARY_25 = #F5FEFF;
    COLOR_PRIMARY_50 = #ECFDFF;
    COLOR_PRIMARY_100 = #CFF9FE;
    COLOR_PRIMARY_200 = #A5F0FC;
    COLOR_PRIMARY_300 = #67E3F9;
    COLOR_PRIMARY_400 = #22CCEE;
    COLOR_PRIMARY_500 = #06AED4;
    COLOR_PRIMARY_600 = #088AB2;
    COLOR_PRIMARY_700 = #0E6F90;
    COLOR_PRIMARY_800 = #155B75;
    COLOR_PRIMARY_900 = #164C63;
    // -------- color-error ------------------------------------
    COLOR_ERROR_25 = #FFFBFA;
    COLOR_ERROR_50 = #FEF3F2;
    COLOR_ERROR_100 = #FEE4E2;
    COLOR_ERROR_200 = #FECDCA;
    COLOR_ERROR_300 = #FDA29B;
    COLOR_ERROR_400 = #F97066;
    COLOR_ERROR_500 = #F04438;
    COLOR_ERROR_600 = #D92D2D;
    COLOR_ERROR_700 = #B42318;
    COLOR_ERROR_800 = #912018;
    COLOR_ERROR_900 = #7A271A;
    // -------- color-warning ------------------------------------
    COLOR_WARNING_25 = #FFFCF5;
    COLOR_WARNING_50 = #FFFAEB;
    COLOR_WARNING_100 = #FEF0C7;
    COLOR_WARNING_200 = #FEDF89;
    COLOR_WARNING_300 = #FEC84B;
    COLOR_WARNING_400 = #FDB022;
    COLOR_WARNING_500 = #F79009;
    COLOR_WARNING_600 = #DC6803;
    COLOR_WARNING_700 = #B54708;
    COLOR_WARNING_800 = #93370D;
    COLOR_WARNING_900 = #7A2E0E;
    // -------- color-success ------------------------------------
    COLOR_SUCCESS_25 = #F6FEF9;
    COLOR_SUCCESS_50 = #ECFDF3;
    COLOR_SUCCESS_100 = #D1FADF;
    COLOR_SUCCESS_200 = #A6F4C5;
    COLOR_SUCCESS_300 = #6CE9A6;
    COLOR_SUCCESS_400 = #32D583;
    COLOR_SUCCESS_500 = #12B76A;
    COLOR_SUCCESS_600 = #039855;
    COLOR_SUCCESS_700 = #027A48;
    COLOR_SUCCESS_800 = #05603A;
    COLOR_SUCCESS_900 = #054F31;
    // -------- font-family ------------------------------------
    FONT_FAMILY = dep("crate://self/resources/font/OPPOSans-Regular.ttf");
    FONT_SIZE = 10.0;
    FONT_SIZE_SMALL = 9.0;
    // padding -----------------------------------------------------
    GLOBAL_PADDING = {top: 10.0, left: 16.0, bottom: 10.0, right: 16.0};
    GLOBAL_PADDING_SMALL = {top: 4.6, left: 9.0, bottom: 4.6, right: 9.0};
    // align -------------------------------------------------------
    ALIGN_CENTER_WALK = {x: 0.5, y: 0.5};
    ALIGN_LEFT_WALK = {x: 0.0, y: 0.5};
    GLOBAL_DURATION = 0.25;
}
