# Hướng dẫn Onboarding & Quy tắc Phát triển cho AI Agents

Tài liệu này lưu trữ thông tin cốt lõi của dự án và các quy tắc ứng xử bắt buộc dành cho các AI Agent khi tham gia phát triển codebase này.

---

## 1. Tổng quan Dự án & Cấu trúc Thư mục

Dự án này là bản tái hiện lại logic game **Plants vs. Zombies** sử dụng Rust và game engine **Bevy 0.14**, đi kèm hệ thống ráp hoạt ảnh gốc `.reanim` từ PopCap Games.

### Cấu trúc thư mục cốt lõi:
- **`pvz_logic/`**: Thư mục chứa project Rust / Bevy.
  - **`src/main.rs`**: Logic game hoàn chỉnh (Vòng lặp game 6 bước + Peashooter bắn Zombie + Đạn bay + Ăn cây + Trạng thái Game Over).
  - **`src/bin/peashooter_demo.rs`**: Scene trống để debug hoạt ảnh của Peashooter (phóng lớn 1.5 lần, nền tối, có crosshair debug).
  - **`assets/PvZ_Assets/`**: Liên kết ảo (Symlink) trỏ tới thư mục chứa toàn bộ tài nguyên hình ảnh/âm thanh/hoạt ảnh đã được giải mã từ `main.pak`.
- **`PvZ_Assets/`**: Thư mục chứa các tài nguyên thô (đã decrypt) dùng chung.
- **`docs/`**: Chứa tài liệu kỹ thuật, tiêu biểu là [reanim_assembly_guide.md](docs/reanim_assembly_guide.md).
- **`Makefile`**: Interface tự động hóa các lệnh chạy/build/test.

---

## 2. Thiết lập Môi trường & Lệnh chạy Nhanh

Các lệnh tự động hóa đã được thiết lập sẵn trong `Makefile` ở thư mục gốc:

*   **Chạy game chính (Gameplay Loop)**:
    ```bash
    make run
    ```
*   **Chạy scene debug Peashooter**:
    ```bash
    make demo
    ```
*   **Kiểm tra lỗi cú pháp (Compiler Check)**:
    ```bash
    make check
    ```
*   **Build file thực thi**:
    ```bash
    make build
    ```
*   **Dọn dẹp build target**:
    ```bash
    make clean
    ```

---

## 3. Quy tắc Git & Commit Messages

Mọi commit của agent phải tuân thủ chuẩn **Conventional Commits**:

```pattern
<type>(<scope>): <description>
```

*   **Types được chấp nhận:**
    *   `feat`: Tính năng mới cho người dùng.
    *   `fix`: Sửa lỗi (bug).
    *   `docs`: Thay đổi tài liệu hướng dẫn hoặc comment code.
    *   `style`: Thay đổi định dạng code (formatting, semicolon), không ảnh hưởng logic.
    *   `refactor`: Cơ cấu lại mã nguồn, không sửa lỗi cũng không thêm tính năng.
    *   `test`: Viết thêm unit test hoặc sửa test.
    *   `chore`: Thay đổi quy trình build, package dependencies, v.v.
*   **Ví dụ đúng:**
    *   `feat(classroom): add split-screen audio player with speed controls`
    *   `fix(quiz): resolve layout overflow on mobile screens`
    *   `docs(agents): update commit conventions in AGENTS.md`

---

## 4. Quản lý Tiến độ (Progress Tracking)

*   **TODO.md** là nguồn sự thật duy nhất về tiến độ công việc hiện tại.
*   Khi hoàn thành bất kỳ task nào, agent phải cập nhật trạng thái trong `TODO.md` thành `[x]` và ghi rõ ngày hoàn thành nếu cần.
*   Tuyệt đối không tự ý xóa bỏ các task chưa hoàn thành mà không có sự đồng ý của User.

---

## 5. Hướng dẫn Hành vi & Lập trình cho AI Agent (Behavioral Guidelines)

Bộ hướng dẫn giúp hạn chế tối đa các lỗi lập trình phổ biến của AI. Tác vụ triển khai cần ưu tiên sự cẩn trọng hơn là tốc độ.

### A. Suy nghĩ trước khi lập trình (Think Before Coding)
**Không tự giả định. Không che giấu sự mơ hồ. Luôn nêu rõ các đánh đổi (tradeoffs).**
*   **Xác định giả định:** Luôn viết rõ các giả định của bạn trước khi bắt đầu. Nếu chưa chắc chắn, hãy hỏi lại.
*   **Trình bày các giải pháp:** Nếu có nhiều cách hiểu hoặc phương án thiết kế khác nhau, hãy nêu rõ các lựa chọn thay vì âm thầm tự quyết định.
*   **Ưu tiên sự đơn giản:** Nếu có cách tiếp cận đơn giản hơn, hãy đề xuất cho User. Hãy phản biện khi thấy cần thiết.
*   **Dừng lại khi không rõ ràng:** Nếu có bất cứ điểm nào mơ hồ, hãy tạm dừng, nêu rõ phần gây bối rối và hỏi ý kiến User.

### B. Ưu tiên sự tối giản (Simplicity First)
**Chỉ viết lượng code tối thiểu để giải quyết vấn đề. Tuyệt đối không suy đoán tính năng tương lai.**
*   Không thêm các tính năng nằm ngoài phạm vi yêu cầu của User.
*   Không thiết lập các lớp trừu tượng (abstractions) cho phần mã nguồn chỉ dùng một lần.
*   Không tự ý thêm tính năng "linh hoạt" (flexibility) hay "khả năng cấu hình" (configurability) nếu không được yêu cầu.
*   Không viết code xử lý lỗi cho các kịch bản bất khả thi.
*   Nếu viết 200 dòng code nhưng có thể giải quyết gọn gàng trong 50 dòng, hãy viết lại.
*   **Câu hỏi tự kiểm tra:** *"Một senior engineer có cho rằng đoạn code này đang bị làm quá phức tạp không?"* Nếu có, hãy tối giản nó.

### C. Thay đổi có mục tiêu (Surgical Changes)
**Chỉ can thiệp vào những gì bắt buộc. Chỉ dọn dẹp phần do chính mình tạo ra.**
*   **Không lan man:** Khi sửa code, không tự ý "tiện tay cải tiến" phần code, comment hoặc định dạng xung quanh.
*   **Không sửa thứ đang chạy tốt:** Không tái cấu trúc (refactor) những phần code không bị lỗi.
*   **Tôn trọng Style cũ:** Tuân thủ phong cách viết code hiện tại của dự án, ngay cả khi bạn có sở thích hoặc cách làm khác.
*   **Xử lý code thừa:**
    *   If phát hiện code thừa không liên quan, hãy nhắc đến trong câu trả lời chứ không tự ý xóa bỏ.
    *   If các thay đổi của chính bạn làm phát sinh code thừa (imports/biến/hàm không dùng nữa), hãy dọn dẹp sạch sẽ.
*   **Câu hỏi tự kiểm tra:** *"Mỗi dòng code thay đổi có thể truy xuất nguồn gốc trực tiếp từ yêu cầu của User không?"*

### D. Thực thi theo mục tiêu rõ ràng (Goal-Driven Execution)
**Định nghĩa rõ tiêu chí thành công. Lặp lại chu kỳ kiểm thử cho đến khi được xác minh.**
*   Biến các yêu cầu chung chung thành các mục tiêu cụ thể, kiểm chứng được:
    *   *"Thêm validation"* $\rightarrow$ *"Viết test cho đầu vào sai, sau đó code cho test pass"*.
    *   *"Sửa bug X"* $\rightarrow$ *"Viết test tái hiện bug X, sau đó sửa code cho test pass"*.
*   Đối với tác vụ phức tạp gồm nhiều bước, hãy viết ra một kế hoạch ngắn gọn trước khi code:
    ```markdown
    1. [Bước thực hiện 1] → xác minh bằng: [cách kiểm tra]
    2. [Bước thực hiện 2] → xác minh bằng: [cách kiểm tra]
    ```
*   **Lợi ích:** Tiêu chí thành công rõ ràng giúp AI tự lập trình và kiểm thử độc lập, hạn chế tối đa việc liên tục hỏi các câu làm rõ lặt vặt.
