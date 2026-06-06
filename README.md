rMonitor Windows System Monitor TUI

Doctor Diagnostic System
Runs a quick health check of the monitoring environment, checking execution privilege levels, registry access permissions, GPU hardware paths, and connection interfaces by running:
rmon.exe --doctor

Native Windows Installation
Registers rMonitor under the user's App Paths registry entry (enabling execution via Win + R as rmon) and creates a shortcut in the Windows Start Menu programs folder (making it instantly searchable/launchable via the Start Menu) by running:
rmon.exe --install

Error Logging
All runtime activities, errors, and system crashes (including panics) are automatically captured and logged silently to:
%APPDATA%\rmonitor\rmonitor.log (typically C:\Users\User\AppData\Roaming\rmonitor\rmonitor.log)

Building From Source
Ensure you have the Rust compiler toolchain installed on Windows.

To build, first clone the repository and navigate to the folder:
cd rMonitor

Then build the release binary:
.\build.bat

This will generate an optimized executable with embedded application resource icons directly at the root (rmon.exe).
