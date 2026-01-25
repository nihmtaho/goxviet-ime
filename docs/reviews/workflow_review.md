# Đánh giá Quy trình làm việc cho Issue #36 - Backspace xóa nhầm autocomplete (xóa suggestion + ký tự)

## Mô tả

Hoàn thiện lần 2 cho bug #36: bảo đảm Backspace trong browser address bars/search fields xóa đồng thời ký tự trước cursor và phần autocomplete đang highlight bằng cách dùng slicing UTF-16 qua Accessibility API.

## Những gì đã làm tốt

- Cập nhật implementation plan và task list trước khi code, bám sát yêu cầu workflow.
- Refactor `injectViaAX` dùng NSString/UTF-16 để khớp với AX ranges, loại bỏ sai lệch index.
- Dọn suffix logic: bỏ hoàn toàn suggestion đang highlight, giữ phần suffix thật, đặt cursor bằng `utf16.count`.
- Giữ logging chi tiết để hỗ trợ debug các trường hợp autocomplete.

## Những gì cần cải thiện

- Chưa chạy manual test trên Arc/Chrome/Safari/Firefox sau thay đổi này; cần xác nhận thực tế.
- Chưa có test tự động cho nhánh AX; cân nhắc thêm harness hoặc mô phỏng AX để regression test.
- Logging đang luôn bật; xem xét flag hóa trong build Release để giảm noise.

## Bài học rút ra

- Làm việc với AX text phải dùng đơn vị UTF-16, tránh thao tác bằng `String.count` gây lệch cursor/suffix.
- Khi selection > 0 trong address bar, xem đó là autocomplete và loại bỏ toàn bộ selection trước khi chèn text mới.
- Truncate file/plan trước khi ghi lại giúp tránh lỗi heredoc khi viết lại nội dung dài.

## Notes/Important

- Pending: manual validation trên browsers; nếu fail cần thu log `AX autocomplete` để tra cứu.
- Tests chưa chạy trong vòng này (build/test thủ công cần thực hiện trước khi release).
