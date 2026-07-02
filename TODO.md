# Danh sách công việc cần làm (TODO.md)

Tài liệu này lưu trữ tiến độ phát triển dự án game Plants vs. Zombies (Bevy / Rust) dựa trên [PLAN.md](file:///Users/fregd/Documents/code/playground/PvZ/PLAN.md).

---

## 📅 Tiến độ Tổng quan

- [x] **Giai đoạn 0 — Xương sống: Asset & Scene** (Hoàn thành: 03/07/2026)
- [x] **Giai đoạn 1 — Vòng lặp gameplay lõi (1 lane)** (Hoàn thành: 03/07/2026)
- [x] **Giai đoạn 2 — Mở rộng ra 5 lane** (Hoàn thành: 03/07/2026)
- [ ] **Giai đoạn 3 — Kinh tế Sun**
- [ ] **Giai đoạn 4 — UI/HUD (`bevy_ui`)**
- [ ] **Giai đoạn 5 — Wave System & AppState**
- [ ] **Giai đoạn 6 — Data-driven mở rộng nội dung**
- [ ] **Giai đoạn 7 — Save/Progress**
- [ ] **Giai đoạn 8 — Polish**

---

## 📝 Chi tiết Từng Giai đoạn

### Giai đoạn 0 — Xương sống: Asset & Scene
- [x] Thiết lập Camera 2D cố định.
- [x] Giải mã tài nguyên `main.pak` và tích hợp vào dự án tại thư mục cục bộ `pvz_logic/assets/`.
- [x] Xây dựng hệ tọa độ logic Grid 5 hàng $\times$ 9 cột cùng hệ thống chuyển đổi tọa độ chuột (Snapping & Hover Highlight).
- [x] Tích hợp ảnh nền sân vườn ban ngày gốc `background1.jpg` khớp 100% từng pixel với hệ tọa độ Grid toán học.
- [x] Ráp thành công hoạt ảnh và cơ chế Blend/Overlay Track cho Peashooter Single, Repeater và Cattail.
- [x] Tạo công cụ Custom Skill hỗ trợ tìm kiếm Asset tự động cho AI Agents.
- [x] Tạo demo cuộn cỏ lăn trải thảm cỏ (`sod_roll_demo.rs`) có âm thanh đi kèm.

### Giai đoạn 1 & 2 — Vòng lặp gameplay lõi & 5 lane
- [x] Spawn Zombie ngẫu nhiên theo các làn cỏ.
- [x] Cơ chế di chuyển của Zombie trên các hàng độc lập.
- [x] Peashooter, Repeater, Cattail tự động scan Zombie trên cùng làn và tiến hành bắn (Repeater bắn đúp, Cattail bắn gai nhọn).
- [x] Sát thương và va chạm đạn-zombie theo hàng (`GridPos`).
- [x] Trạng thái Zombie dừng lại ăn cây và cây chết biến mất khi hết máu.
- [x] Trạng thái Game Over khi Zombie chạm vào rìa trái bản đồ (đột nhập vào nhà).

### Giai đoạn 3 — Kinh tế Sun
- [ ] Thêm Resource `SunCount` lưu trữ số lượng mặt trời.
- [ ] Cơ chế rơi Sun tự nhiên từ trên trời xuống (X ngẫu nhiên, dừng lại ở Y ngẫu nhiên).
- [ ] Tương tác click chuột vào Sun để thu thập mặt trời (cập nhật Resource `SunCount`).
- [ ] Triển khai cây Hoa hướng dương (`Sunflower` / `Sunflower.reanim`) với cơ chế sản sinh mặt trời theo chu kỳ.
- [ ] Ràng buộc lượng Sun và cooldown khi click đặt cây trên Grid.

### Giai đoạn 4 — UI/HUD (`bevy_ui`)
- [ ] Thiết kế thanh chọn cây (Seed Packet Bar) ở phía trên: hiển thị icon cây, giá mặt trời, hiệu ứng mờ khi chưa đủ sun hoặc đang trong cooldown.
- [ ] Ô hiển thị số lượng mặt trời (Sun Counter) ở góc trên bên trái.
- [ ] Thanh hiển thị tiến trình làn sóng Zombie (Wave Progress Bar).
- [ ] Nút xẻng (Shovel Tool) cho phép người chơi click vào để đào bỏ cây đã trồng.

### Giai đoạn 5 — Wave System & AppState
- [ ] Thiết lập hệ thống cấu hình đợt tấn công từ file dữ liệu màn chơi (`WaveEvent` bao gồm: thời gian, loại zombie, làn cỏ xuất hiện).
- [ ] Máy trạng thái GameState đầy đủ: `MainMenu → LevelSelect → ChoosePlants → InGame → Paused → GameWin / GameOver`.
- [ ] Điều kiện thắng cuộc: tiêu diệt toàn bộ Zombie ở Wave cuối cùng.

### Giai đoạn 6 — Data-driven mở rộng nội dung
- [ ] Chuyển các thông số của Plant và Zombie từ hardcode trong Rust sang cấu hình định dạng RON (`serde`).
- [ ] Thêm cây mới: Wall-nut (Kháng cự cao), Cherry Bomb (Nổ diện rộng cùng hàng/hàng lân cận).
- [ ] Thêm biến thể Zombie: Conehead Zombie, Buckethead Zombie (Tăng giáp máu).

### Giai đoạn 7 — Save/Progress
- [ ] Cơ chế lưu trữ tiến độ chơi (màn đã mở khóa, loại cây đã mở khóa) xuống file lưu trữ cục bộ.
- [ ] Màn hình chọn tối đa 6 loại cây trước khi bắt đầu màn chơi.

### Giai đoạn 8 — Polish
- [ ] Tích hợp toàn bộ âm thanh trò chơi (nhạc nền, tiếng đạn trúng, tiếng ăn cây, tiếng nổ...).
- [ ] Tích hợp máy cắt cỏ (Lawn Mower) ở rìa trái mỗi làn cỏ làm phòng tuyến cuối cùng.
- [ ] Hiệu ứng rung màn hình (Screen Shake) khi Cherry Bomb nổ.
- [ ] Tích hợp công cụ gỡ lỗi trực quan `bevy_inspector_egui` để theo dõi các thực thể lúc runtime.
