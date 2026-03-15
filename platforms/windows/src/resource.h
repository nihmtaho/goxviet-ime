#pragma once

// Icons — IDI_APP_ICON must be 1 for Explorer / Task Manager
#define IDI_APP_ICON        1
#define IDI_TRAY_ON         101
#define IDI_TRAY_OFF        102

// Tray context menu
#define IDM_ENABLE          201
#define IDM_TELEX           202
#define IDM_VNI             203
#define IDM_SETTINGS        204
#define IDM_ABOUT           205
#define IDM_EXIT            206

// Settings window controls
#define IDC_TOGGLE_ENABLED      301
#define IDC_CMB_METHOD          302
#define IDC_CMB_TONE            303
#define IDC_TOGGLE_SMART        304
#define IDC_TOGGLE_INSTANT      305
#define IDC_TOGGLE_ESC          306
#define IDC_TOGGLE_SHORTCUTS    307
#define IDC_TOGGLE_AUTOSTART    308
#define IDC_TOGGLE_SOUND        309
#define IDC_TOGGLE_PERAPP       310
#define IDC_TOGGLE_CAPITALIZE   311
#define IDC_CMB_ENCODING        312
#define IDC_BTN_SHORTCUTS           313
#define IDC_LBL_VERSION             314
#define IDC_BTN_ABOUT               315
#define IDC_NAV_LIST                316
#define IDC_TOGGLE_FREE_TONE        317
#define IDC_TOGGLE_SHIFT_BS         318
#define IDC_TOGGLE_AUTO_DISABLE_NL  319

// Shortcuts dialog controls
#define IDC_SHORTCUTS_LIST      401
#define IDC_EDIT_TRIGGER        402
#define IDC_EDIT_REPLACEMENT    403
#define IDC_BTN_ADD             404
#define IDC_BTN_DELETE          405
#define IDC_BTN_IMPORT          406
#define IDC_BTN_EXPORT          407

// Dialog IDs (templates in resources.rc)
#define IDD_SETTINGS            501
#define IDD_SHORTCUTS           502
#define IDD_ABOUT               503

// Settings window — new IDs
#define IDC_BTN_RESET               320
#define IDC_CMB_RESTORE_MOD         321
#define IDC_PA_BTN_TOGGLE           322
#define IDC_PA_BTN_REMOVE           323
#define IDC_ABOUT_GITHUB            330
#define IDC_ABOUT_ISSUES            331
#define IDC_ABOUT_UPDATE            332

// Custom window messages
#define WM_TRAYICON             (WM_USER + 1)
#define WM_TOGGLE_CHANGED       (WM_USER + 2)
