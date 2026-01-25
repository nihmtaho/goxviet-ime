# Nhiem vu cho Core engine performance optimizations

- [x] Do perf baseline (benches/tests) cho transform, restore, buffer, dictionary
- [x] Toi uu buffer/raw_input/restore (prealloc, vong lap tay, mem::take history) khong doi API
- [x] Toi uu transform/tone_positioning/validation bang lookup bang va inline fast paths
- [x] Giam overhead dictionary/language decision, tranh Vec tam, xem cache nho (tối ưu hot path dictionary)
- [ ] Chay test + bench xac nhan khong regress
- [ ] Cap nhat review/workflow khi hoan tat
