# User Workflows

## Installation & Setup

1.  **Launch**: Open `GoxViet.app`.
2.  **Permissions**:
    -   On first launch, the app will detect missing Accessibility permissions.
    -   A dialog appears guiding the user to **System Settings > Privacy & Security > Accessibility**.
    -   The app polls for permission changes and automatically starts the input engine once granted.
3.  **Menu Bar**: The app runs in the menu bar. An icon (🇻🇳) indicates the IME is active.

## Usage

### Toggling Input
-   **Shortcut**: Press `Ctrl + Space` (default) to toggle between Vietnamese and English.
-   **Menu**: Click the menu bar icon and select "Vietnamese Input" (switch).
-   **Icon State**:
    -   `🇻🇳`: Vietnamese Mode (Engine Active).
    -   `EN`: English Mode (Passthrough).

### Switching Input Methods
1.  Open the Menu Bar menu.
2.  Select **Input Method: Telex** or **Input Method: VNI**.
3.  Alternatively, go to **Settings > General** and use the segment selector.

## Updates

GoxViet includes an integrated update checker (`UpdateManager`).
1.  **Check**: The app checks for updates on startup (background) or when opening the **About** tab in Settings.
2.  **Notification**: A banner appears in the About tab if an update is available.
3.  **Process**:
    -   Click "Update".
    -   The app downloads the new version.
    -   It extracts the `.zip` archive.
    -   It swaps the application bundle and relaunches.

## Troubleshooting

### "Input not working"
1.  Check the Menu Bar icon. Is it `🇻🇳`?
2.  Check Accessibility permissions. Remove and re-add GoxViet in System Settings if issues persist after an update.
3.  Check if you are in a password field (Secure Input mode disables all IMEs for security).

### "Keys are doubling"
-   This can happen if another Vietnamese keyboard (Mac system keyboard or other IME) is active simultaneously. Ensure only **ABC** or **US** is selected as the system keyboard source.
