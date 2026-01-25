# Ke hoach trien khai cho Core engine performance optimizations

## Mo ta

Tang toc duong di nhap lieu Viet/Anh trong core engine (buffer, transform, restore, dictionary) giu dung hanh vi hien tai.

## Problem Issue (neu co)

### Current Issues

- Allocation va clone trong buffer/history/restore lam tang latency keystroke.
- Transform Vietnamese quet Vec va realloc khong can thiet.
- English auto-restore dung iterator chain tao Vec trung gian.
- Dictionary check tao Vec tam va khong cache.

### Root Causes

- Su dung Vec collect/filter_map trong duong nong.
- Clone Buffer/RawInput khi pop history.
- Thieu preallocation/small buffer cho tu ngan.
- Tone positioning va transform quet nhieu lan.

## Cac buoc trien khai

1. Thu perf baseline (benches) cho transform, restore, buffer, dictionary.
2. Toi uu buffer/raw_input/restore (preallocate, vong lap tay, mem::take history) giu API.
3. Toi uu transform/tone_positioning/validation bang bang tra truoc va inline fast-paths.
4. Giam overhead dictionary (tranh Vec tam, co the cache tu gan day) va language_decision.
5. Chay test + bench xac nhan khong thay doi hanh vi, so sanh so lieu truoc/sau.

## Proposed Changes

- Buffer: pre-reserve/smallvec noi bo, vong lap tay thay filter_map, inline getter.
- History: pop dung mem::take hoac swap de tranh clone, van giu LIFO/capacity.
- Restore: build_raw_output dung vong lap don, prealloc theo raw_input.len, truc tiep dien vao Result buffer.
- Transform/Tone positioning: bang tra phan loai vowel/diacritic, giam Vec tam va scan; inline ham hot.
- Dictionary: tranh Vec u16 tam, kiem tra bang slice, co the them cache nho.

## Thoi gian du kien

- Buoc 1: 0.5 ngay
- Buoc 2: 1 ngay
- Buoc 3: 1.5 ngay
- Buoc 4: 0.5 ngay
- Buoc 5: 0.5 ngay

## Tai nguyen can thiet

- cargo bench/test, flamegraph neu can, du lieu dictionary san co.

## Implementation Order

1. Baseline (benches/tests).
2. Buffer/restore/history vi rui ro thap.
3. Transform/tone_positioning/validation.
4. Dictionary/language decision.
5. Regression tests + doc summary.
