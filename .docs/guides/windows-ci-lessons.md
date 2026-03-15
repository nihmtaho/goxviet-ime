# Windows CI – Lessons & Known Fixes

## 1. `dumpbin` không có trong PATH

**Triệu chứng:**
```
The term 'dumpbin' is not recognized as a name of a cmdlet, function, script file, or executable program.
```

**Fix:** Locate qua `vswhere` trước khi gọi:
```powershell
$vswhere = "C:\Program Files (x86)\Microsoft Visual Studio\Installer\vswhere.exe"
$vsPath  = & $vswhere -latest -property installationPath
$dumpbin = Get-ChildItem "$vsPath\VC\Tools\MSVC\*\bin\Hostx64\x64\dumpbin.exe" `
             -ErrorAction Stop | Select-Object -First 1 -ExpandProperty FullName
$exports = & $dumpbin /exports $dll 2>&1
```

---

## 2. PowerShell `-notmatch` trên array là filter, không phải boolean

**Triệu chứng:** Export check luôn báo "Missing" dù symbol có trong DLL.

**Root cause:** `$array -notmatch "x"` trả về *phần tử không chứa `"x"`* (array con), không phải `$false`. Array con luôn truthy.

**Fix:** Dùng `-not ($array -match $sym)`:
```powershell
# SAI
if ($exports -notmatch $sym) { $missing += $sym }

# ĐÚNG
if (-not ($exports -match $sym)) { $missing += $sym }
```

---

## 3. Ambiguous `Application` khi dùng cả WPF và WinForms

**Triệu chứng:**
```
error CS0104: 'Application' is an ambiguous reference between
'System.Windows.Forms.Application' and 'System.Windows.Application'
```

**Root cause:** `GoxViet.csproj` có cả `<UseWPF>true</UseWPF>` và `<UseWindowsForms>true</UseWindowsForms>`.

**Fix:** Thêm using alias trong file bị ảnh hưởng:
```csharp
// App.xaml.cs
using Application = System.Windows.Application;

// TrayIcon.cs (dùng cả Forms lẫn WPF)
using System.Windows.Forms;
using WpfApplication = System.Windows.Application;
// Sau đó dùng WpfApplication.Current.Shutdown()
```

---

## 4. Eval tests dùng legacy engine API — bỏ qua trong CI

`eval_vietnamese_22k.rs` và `eval_english_100k.rs` dùng `goxviet_core::engine::Engine` (API cũ trước v3.0.0) và chạy 22k/100k words — quá chậm cho CI.

**Fix:** Skip khi chạy integration tests:
```yaml
cargo test --tests --release --target x86_64-pc-windows-msvc -- \
  --skip eval_vietnamese_22k \
  --skip eval_english_100k
```

---

## 5. `serial_test` crate — cần single-threaded execution

Một số tests dùng `#[serial]` (ví dụ `shift_backspace_test.rs`). Nếu chạy parallel sẽ flaky.

**Fix:**
```yaml
env:
  RUST_TEST_THREADS: 1
```

---

## 6. cdylib cần thêm vào `crate-type` cho Windows DLL

`core/Cargo.toml` mặc định chỉ có `["staticlib", "rlib"]`. P/Invoke trên Windows cần `.dll` (`cdylib`).

**Fix:**
```toml
[lib]
crate-type = ["staticlib", "cdylib", "rlib"]
```
