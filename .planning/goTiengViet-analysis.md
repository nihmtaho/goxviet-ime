# Phân tích goTiengViet Resources

**Source:** `.uvasx/goTiengViet-resource/Resources/`  
**Ngày phân tích:** 2026-02-23  
**Mục đích:** Học hỏi từ reference implementation để ứng dụng vào GoxViet

---

## 1. KieuGo.ini - Định nghĩa kiểu gõ

### Cấu trúc hiện tại
```ini
[Telex]
z = XoaDauThanh
s = DauSac
f = DauHuyen
r = DauHoi
x = DauNga
j = DauNang
a = A_Â
e = E_Ê
o = O_Ô
w = UOA_ƯƠĂ
d = D_Đ
```

### Các kiểu gõ được hỗ trợ
| Kiểu gõ | Mô tả |
|---------|-------|
| Telex | Kiểu gõ phổ biến nhất |
| Telex 2 | Telex với `[` `]` cho ư, ơ |
| VNI | Số 0-9 cho dấu và ký tự |
| VIQR | Dùng `'` `?` `~` `.` `^` `+` `*` `(` |
| Microsoft layout | Layout riêng của Microsoft |
| Telex + VNI + VIQR | Kết hợp tất cả |
| VNI bàn phím Pháp | Mapping cho bàn phím AZERTY |

### Action types được định nghĩa
- **Dấu thanh:** `XoaDauThanh`, `DauSac`, `DauHuyen`, `DauHoi`, `DauNga`, `DauNang`
- **Nguyên âm đơn:** `A_Â`, `E_Ê`, `O_Ô`, `A_Ă`
- **Nguyên âm kép:** `UOA_ƯƠĂ`, `AEO_ÂÊÔ`, `UO_ƯƠ`
- **Phụ âm:** `D_Đ`
- **Escape:** `Thoat` (cho VIQR)

### Ứng dụng vào GoxViet
- [ ] Thiết kế lại `InputMethod` trait với mapping table dạng config
- [ ] Tách logic processing ra khỏi definition
- [ ] Hỗ trợ load config từ file (user có thể custom)
- [ ] Thêm kiểu gõ Microsoft layout
- [ ] Thêm kiểu gõ VNI bàn phím Pháp (international support)

---

## 2. GhepVan.ini - Luật ghép vần (Syllable Structure)

### Cấu trúc âm tiết tiếng Việt
```
Âm tiết = PAD + NA + PAC
         (Phụ âm đầu) + (Nguyên âm) + (Phụ âm cuối)
```

### Phân nhóm chi tiết

#### PAD (Phụ âm đầu) - 3 nhóm
```
PAD.0 = b d đ g gh m n nh p ph r s t tr v    (phụ âm đứng trước mọi NA)
PAD.1 = c h k kh qu th                        (không đứng trước NA.3, NA.4)
PAD.2 = ch gi l ng ngh x                      (chỉ đứng trước NA.0, NA.1, NA.2)
```

#### NA (Nguyên âm) - 6 nhóm
```
NA.0 = ê i ua uê uy y                         (nguyên âm trước/không có PAC)
NA.1 = a iê oa uyê yê                         (nguyên âm có thể có PAC)
NA.2 = â ă e o oo ô ơ oe u ư uâ uô ươ        (nguyên âm phổ biến)
NA.3 = oă                                     (chỉ có PAC.1, PAC.2)
NA.4 = uơ                                     (chỉ có PAC.1, PAC.2)
NA.5 = ai ao au âu ay ây eo êu ia iêu iu...  (nguyên âm kép, không có PAC)
```

#### PAC (Phụ âm cuối) - 3 nhóm
```
PAC.0 = ch nh                                 (chỉ sau NA.0, NA.2)
PAC.1 = c ng                                  (sau NA.0, NA.1, NA.2, NA.3, NA.4)
PAC.2 = m n p t                               (sau NA.0, NA.1, NA.2, NA.3, NA.4)
```

### Bảng kết hợp PAD_NA
```
PAD_NA.0 = 0 1 2 5    (PAD.0 kết hợp với NA.0, NA.1, NA.2, NA.5)
PAD_NA.1 = 0 1 2 3 4 5
PAD_NA.2 = 0 1 2 3 5
```

### Bảng kết hợp NA_PAC
```
NA_PAC.0 = 0 2       (NA.0 kết hợp với PAC.0, PAC.2)
NA_PAC.1 = 0 1 2
NA_PAC.2 = 1 2
NA_PAC.3 = 1 2
NA_PAC.4 =           (NA.4 không có PAC)
NA_PAC.5 =           (NA.5 không có PAC)
```

### Ứng dụng vào GoxViet
- [ ] Cải thiện FSM validator trong `engine_v2/fsm/`
- [ ] Thêm validation rules theo nhóm PAD/NA/PAC
- [ ] Tối ưu performance với lookup table thay vì regex
- [ ] Hỗ trợ syllable decomposition (tách âm tiết thành PAD + NA + PAC)

---

## 3. BangMa.ini - Bảng mã Encoding

### Các bảng mã được hỗ trợ (15+)
| Bảng mã | Mô tả | Dùng cho |
|---------|-------|----------|
| TCVN3 (ABC) | Bộ mã tiếng Việt phổ biến tại VN | Legacy systems |
| VNI Windows | Encoding của VNI | Legacy Windows |
| Unicode tổ hợp | Unicode với combining marks | Max compatibility |
| Windows 1258 | Codepage của Windows | Windows legacy |
| VIQR | Quoted-readable format | Email, text-only |
| VISCII | Vietnamese Standard Code | Unix legacy |
| VPS | Vietnamese Professional Society | Legacy |
| BKHCM 1/2 | Bộ mã BK HCM | Legacy |
| Vietware X/F | Encoding của Vietware | Legacy |
| UTF-8 | Modern standard | **Default** |
| NCR Decimal/Hex | Numeric Character Reference | Web, HTML |
| Unicode C String Hex | Escape sequences | Programming |

### Format mapping
```
[UTF-8]
đ=Ä'
â=Ã¢
ă=Äƒ
...
```

### Ứng dụng vào GoxViet
- [ ] Feature output encoding (optional, low priority)
- [ ] Hỗ trợ copy với encoding khác cho legacy apps
- [ ] Unicode tổ hợp vs Unicode dựng sẵn (precomposed)

---

## 4. TuDien.txt - Từ điển đơn âm

### Thông tin
- **Số lượng:** ~7,000+ âm tiết
- **Format:** 1 từ/line, có dấu đầy đủ
- **Mục đích:** Validate âm tiết hợp lệ

### Ví dụ
```
a
á
à
ã
ả
ạ
ác
ắc
ậc
ặc
...
```

### Ứng dụng vào GoxViet
- [ ] Nạp vào HashSet/PhfMap cho O(1) lookup
- [ ] Build-time embed vào binary (phf - perfect hash function)
- [ ] Fallback validation khi FSM không match
- [ ] Combine với FSM để tối ưu

### Code snippet gợi ý
```rust
use phf::Set;

static VALID_SYLLABLES: Set<&'static str> = phf::phf_set! {
    "a", "á", "à", "ã", "ả", "ạ", "ác", "ắc", ...
};

pub fn is_valid_syllable(s: &str) -> bool {
    VALID_SYLLABLES.contains(s) || FSM::validate(s)
}
```

---

## 5. TuDienTuGhep.txt - Từ điển từ ghép

### Thông tin
- **Số lượng:** 68,769 từ ghép
- **Format:** Dòng đầu là số lượng, sau đó mỗi dòng 1 từ
- **Mục đích:** Nhận diện từ ghép tiếng Việt

### Ví dụ
```
68769
á châu
a còng
ả đào
a di đà phật
á đông
á hậu
a hoàn
á khẩu
...
```

### Ứng dụng vào GoxViet

#### 5.1 English Auto-Restore
- [ ] Check từ ghép trước khi restore
- [ ] Nếu match từ ghép tiếng Việt → không restore
- [ ] Tránh restore nhầm từ như "ánh sáng", "ai đó"

#### 5.2 Smart Suggestions
- [ ] Gợi ý từ ghép khi user gõ
- [ ] Autocomplete cho từ ghép
- [ ] Ranking theo tần suất sử dụng

#### 5.3 Implementation
```rust
static COMPOUND_WORDS: Set<&'static str> = phf::phf_set! {
    "á châu", "a còng", "ả đào", "a di đà phật", ...
};

pub fn is_vietnamese_compound(text: &str) -> bool {
    let normalized = text.to_lowercase();
    COMPOUND_WORDS.contains(&normalized.as_str())
}
```

---

## 6. KieuGoTatNA.ini - Gõ tắt nguyên âm

### Cấu trúc
```ini
[Dùng phím số]
1 = ươ
! = ƯƠ
2 = ươi
@ = ƯƠI
3 = iê
# = IÊ
...

[Dùng ký hiệu]
[ = ươ
{ = ƯƠ
] = iê
} = IÊ
...
```

### Mapping đầy đủ
| Key | Output | Shift + Key |
|-----|--------|-------------|
| 1 | ươ | ƯƠ |
| 2 | ươi | ƯƠI |
| 3 | iê | IÊ |
| 4 | iêu | IÊU |
| 5 | uô | UÔ |
| 6 | uôi | UÔI |
| 7 | uyê | UYÊ |
| 8 | uâ | UÂ |
| 9 | oă | OĂ |
| 0 | ơi | ƠI |
| [ | ươ | ƯƠ |
| ] | iê | IÊ |
| \ | uyê | UYÊ |

### Ứng dụng vào GoxViet
- [ ] Thêm vào feature **Text Expansion** trong Settings
- [ ] Configurable (user có thể thêm/sửa/xóa)
- [ ] Persist trong UserDefaults/JSON config
- [ ] Toggle on/off trong settings

---

## 7. Summary - Action Items

### Priority High
1. [ ] Nạp `TuDien.txt` vào dictionary cho O(1) validation
2. [ ] Cải thiện FSM với rules từ `GhepVan.ini`
3. [ ] Thiết kế lại `InputMethod` trait theo pattern `KieuGo.ini`

### Priority Medium
4. [ ] Thêm `TuDienTuGhep.txt` cho English Auto-Restore
5. [ ] Implement Text Expansion từ `KieuGoTatNA.ini`
6. [ ] Thêm Microsoft layout, VNI bàn phím Pháp

### Priority Low
7. [ ] Multi-encoding output (BangMa.ini)
8. [ ] Config file cho user custom kiểu gõ
9. [ ] Autocomplete suggestions với từ ghép

---

## 8. Technical Notes

### Performance Targets
- Dictionary lookup: O(1) với phf
- FSM validation: < 1μs per syllable
- Compound word check: < 100μs for 5-word sequence

### Memory Considerations
- TuDien.txt: ~100KB raw → ~50KB embedded
- TuDienTuGhep.txt: ~2MB raw → ~1MB embedded
- Consider lazy loading cho từ điển từ ghép

### File Format
- INI format: dễ parse, human-readable
- Consider migrating to TOML/JSON cho GoxViet
- Build-time code generation với phf

---

## 9. References

- Original files: `.uvasx/goTiengViet-resource/Resources/`
- Current implementation: `core/src/engine_v2/`
- FSM docs: `.docs/features/core-engine/`
