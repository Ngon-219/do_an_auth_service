# Auth Service - User Management API

## Tổng quan

Service này cung cấp các API để quản lý người dùng với tích hợp blockchain:
- Thêm người dùng đơn lẻ
- Import hàng loạt từ file Excel
- Tự động lưu trữ thông tin trên blockchain thông qua smart contract

## Cài đặt

### 1. Cấu hình môi trường

Copy file `.env.example` thành `.env` và cập nhật các giá trị:

```bash
cp .env.example .env
```

Cập nhật các thông tin sau trong file `.env`:
- `DATABASE_URL`: Connection string đến PostgreSQL database
- `BLOCKCHAIN_RPC_URL`: URL của Ethereum node (Infura, Alchemy, hoặc local node)
- `DATA_STORAGE_CONTRACT_ADDRESS`: Địa chỉ của DataStorage smart contract đã deploy
- `ADMIN_PRIVATE_KEY`: Private key của admin wallet (để ký transactions)

### 2. Build và chạy

```bash
# Build project
cargo build --release

# Chạy migrations (nếu cần)
cd migration
cargo run -- up
cd ..

# Chạy server
cargo run --release
```

Server sẽ chạy tại `http://localhost:8080`

## API Endpoints

### 1. Thêm người dùng đơn lẻ

**Endpoint:** `POST /api/v1/users`

**Request Body:**
```json
{
  "first_name": "Nguyen",
  "last_name": "Van A",
  "address": "123 Main St, Hanoi",
  "email": "nguyenvana@example.com",
  "password": "password123",
  "cccd": "0123456789",
  "phone_number": "0912345678",
  "role": "student",
  "wallet_address": "0x742d35Cc6634C0532925a3b844Bc454e4438f44e",
  "student_code": "SV001"
}
```

**Lưu ý:**
- `student_code` bắt buộc nếu `role` là "student"
- `role` có thể là: `student`, `teacher`, `admin`, `manager`
- `wallet_address` phải là địa chỉ Ethereum hợp lệ

**Response (201 Created):**
```json
{
  "user_id": "550e8400-e29b-41d4-a716-446655440000",
  "first_name": "Nguyen",
  "last_name": "Van A",
  "email": "nguyenvana@example.com",
  "role": "student",
  "wallet_address": "0x742d35Cc6634C0532925a3b844Bc454e4438f44e",
  "is_first_login": true,
  "created_at": "2025-10-28T10:30:00"
}
```

**cURL Example:**
```bash
curl -X POST http://localhost:8080/api/v1/users \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "Nguyen",
    "last_name": "Van A",
    "address": "123 Main St, Hanoi",
    "email": "nguyenvana@example.com",
    "password": "password123",
    "cccd": "0123456789",
    "phone_number": "0912345678",
    "role": "student",
    "wallet_address": "0x742d35Cc6634C0532925a3b844Bc454e4438f44e",
    "student_code": "SV001"
  }'
```

### 2. Import hàng loạt từ Excel

**Endpoint:** `POST /api/v1/users/bulk`

**Request:** Multipart form-data với file Excel

**Định dạng file Excel:**

File Excel phải có các cột sau (theo thứ tự):

| Column | Tên cột | Mô tả | Bắt buộc |
|--------|---------|-------|----------|
| A | first_name | Họ | Có |
| B | last_name | Tên | Có |
| C | address | Địa chỉ | Có |
| D | email | Email (unique) | Có |
| E | password | Mật khẩu (min 6 ký tự) | Có |
| F | cccd | Số CMND/CCCD | Có |
| G | phone_number | Số điện thoại | Có |
| H | role | Vai trò (student/teacher/admin/manager) | Có |
| I | wallet_address | Địa chỉ ví Ethereum | Có |
| J | student_code | Mã sinh viên (chỉ cho student) | Tùy chọn |

**Ví dụ nội dung Excel:**

| first_name | last_name | address | email | password | cccd | phone_number | role | wallet_address | student_code |
|------------|-----------|---------|-------|----------|------|--------------|------|----------------|--------------|
| Nguyen | Van A | 123 Main St | nva@email.com | pass123 | 001234567 | 0912345678 | student | 0x742d35Cc... | SV001 |
| Tran | Thi B | 456 Oak Ave | ttb@email.com | pass456 | 001234568 | 0912345679 | teacher | 0x8f3a2b1c... | |

**Response (201 Created):**
```json
{
  "total_records": 100,
  "successful": 95,
  "failed": 5,
  "errors": [
    {
      "row": 10,
      "email": "duplicate@email.com",
      "error": "Failed to create user: Email already exists"
    },
    {
      "row": 25,
      "email": "invalid@email.com",
      "error": "Invalid wallet address"
    }
  ]
}
```

**cURL Example:**
```bash
curl -X POST http://localhost:8080/api/v1/users/bulk \
  -F "file=@users.xlsx"
```

**JavaScript/Fetch Example:**
```javascript
const formData = new FormData();
formData.append('file', fileInput.files[0]);

fetch('http://localhost:8080/api/v1/users/bulk', {
  method: 'POST',
  body: formData
})
.then(response => response.json())
.then(data => console.log(data));
```

## Tương tác với Blockchain

Khi tạo user, service tự động thực hiện các thao tác sau trên blockchain:

### Đối với Student:
1. Gọi `registerStudent()` trên DataStorage contract
2. Lưu thông tin: wallet_address, student_code, full_name, email
3. Tự động assign role STUDENT (role = 1)

### Đối với Teacher/Admin/Manager:
1. Gọi `assignRole()` trên DataStorage contract
2. Assign role tương ứng:
   - Teacher: role = 2
   - Admin: role = 3
   - Manager: role = 4

### Batch Registration:
- Import Excel sử dụng `registerStudentsBatch()` để tối ưu gas fee
- Tự động chia batch (mỗi batch tối đa 50 users)

## Swagger UI

API documentation có sẵn tại:
```
http://localhost:8080/swagger-ui
```

## Error Codes

| Status Code | Mô tả |
|-------------|-------|
| 201 | Created - User(s) tạo thành công |
| 400 | Bad Request - Dữ liệu đầu vào không hợp lệ |
| 500 | Internal Server Error - Lỗi server hoặc blockchain |

## Lưu ý bảo mật

1. **Private Key:** Không bao giờ commit file `.env` chứa private key
2. **Password:** Password được hash bằng bcrypt trước khi lưu vào database
3. **Wallet Private Key:** Service không lưu private key của user wallet
4. **Email:** Phải unique trong hệ thống
5. **Blockchain:** Mọi transaction đều được ký bởi admin wallet

## Monitoring

Service sử dụng `tracing` để log. Xem logs:
```bash
# Set log level
export LOG_LEVEL=debug

# Run service
cargo run
```

## Troubleshooting

### Lỗi database connection
- Kiểm tra `DATABASE_URL` trong `.env`
- Đảm bảo PostgreSQL đang chạy
- Chạy migrations: `cd migration && cargo run -- up`

### Lỗi blockchain transaction
- Kiểm tra `BLOCKCHAIN_RPC_URL` có hoạt động
- Đảm bảo admin wallet có đủ ETH/token để trả gas
- Verify contract address đúng
- Kiểm tra admin wallet có quyền ADMIN trên contract

### Excel import errors
- Đảm bảo file đúng định dạng (.xlsx, .xls)
- Header row phải đúng tên cột
- Email không được trùng lặp
- Wallet address phải hợp lệ (format 0x...)

## Development

### Thêm dependencies
```bash
cargo add <package_name>
```

### Format code
```bash
cargo fmt
```

### Run tests
```bash
cargo test
```

### Build for production
```bash
cargo build --release
```

## License

GPL-3.0

