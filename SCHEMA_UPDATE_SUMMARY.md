# Schema Update Summary - Department & Major Restructure

## 📋 Overview

Đã update database schema từ 3-way junction table sang cấu trúc rõ ràng hơn theo diagram mới.

## 🔄 Thay đổi Schema

### Before (Cũ):
```
User ←→ major_department_user ←→ Major
     ←→                       ←→ Department
```
- Junction table `major_department_user` với 3 foreign keys (user, major, department)
- Phức tạp và khó maintain

### After (Mới):
```
User ←→ user_major ←→ Major → Department
(many)   (junction)   (many)  (one)
```
- Junction table `user_major` chỉ 2 foreign keys (user ↔ major)
- `Major` có `department_id` trực tiếp
- Đơn giản và logic hơn

## 📝 Migration Details

### File Created:
`migration/src/m20251028_000002_restructure_major_department.rs`

### Operations:
1. ✅ **Dropped** `major_department_user` table
2. ✅ **Added** `department_id` column to `major` table (nullable, FK to `department`)
3. ✅ **Created** `user_major` table (User ↔ Major many-to-many)

### Rollback Support:
Migration includes `down()` method để revert về schema cũ nếu cần.

## 🗂️ Entities Updated

### New Entities:
- ✅ `src/entities/user_major.rs` - Junction table mới
- ✅ `src/entities/major.rs` - Updated với `department_id: Option<Uuid>`

### Removed Entities:
- ❌ `src/entities/major_department_user.rs` - Đã xóa

## 🔧 Code Changes

### 1. User DTO (`routes/users/dto.rs`)
**Before:**
```rust
pub department_majors: Vec<DepartmentMajorPair>,

pub struct DepartmentMajorPair {
    pub department_id: Uuid,
    pub major_id: Uuid,
}
```

**After:**
```rust
pub major_ids: Vec<Uuid>,  // Chỉ cần major IDs
```

### 2. User Creation (`routes/users/route.rs`)
**Before:**
```rust
for dm_pair in payload.department_majors.iter() {
    major_department_user::ActiveModel {
        major_id: Set(dm_pair.major_id),
        department_id: Set(dm_pair.department_id),
        user_id: Set(user_id),
        ...
    }
}
```

**After:**
```rust
for major_id in payload.major_ids.iter() {
    user_major::ActiveModel {
        user_id: Set(user_id),
        major_id: Set(*major_id),
        ...
    }
}
```

### 3. Major DTO (`routes/majors/dto.rs`)
**Added:**
```rust
pub department_id: Option<Uuid>  // In CreateMajorRequest, UpdateMajorRequest, MajorResponse
```

### 4. Major CRUD (`routes/majors/route.rs`)
- ✅ Create major with `department_id`
- ✅ Update major `department_id`
- ✅ Return `department_id` in responses

### 5. RoleEnum (`entities/sea_orm_active_enums.rs`)
**Added derives:**
```rust
#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "lowercase")]
```

## 📊 API Changes

### User Creation:
```json
POST /api/v1/users
{
  "first_name": "Nguyen",
  "last_name": "Van A",
  "email": "student@example.com",
  "password": "password123",
  "role": "student",
  "student_code": "SV001",
  "major_ids": ["uuid1", "uuid2"]  // ← Changed! Chỉ major IDs
}
```

### Major Creation:
```json
POST /api/v1/majors
{
  "name": "Computer Science",
  "founding_date": "2020-01-01T00:00:00",
  "department_id": "dept-uuid"  // ← New! Optional
}
```

### Major Response:
```json
{
  "major_id": "uuid",
  "name": "Computer Science",
  "founding_date": "2020-01-01T00:00:00",
  "department_id": "dept-uuid",  // ← New!
  "create_at": "2025-10-28T...",
  "update_at": "2025-10-28T..."
}
```

## 🚀 Running Migration

```bash
# 1. Build migration
cd migration
cargo build

# 2. Run migration
cargo run -- up

# 3. Generate entities
cd ..
sea-orm-cli generate entity -o src/entities

# 4. Build application
cargo build --release
```

## ✅ Validation

### Build Status:
```
✅ Migration: Success
✅ Entity Generation: Success
✅ Code Compilation: Success
✅ Release Build: Success
```

### Schema Verification:
```sql
-- Check major table has department_id
SELECT column_name, data_type, is_nullable 
FROM information_schema.columns 
WHERE table_name = 'major';

-- Check user_major table exists
SELECT * FROM information_schema.tables 
WHERE table_name = 'user_major';

-- Check major_department_user is dropped
SELECT * FROM information_schema.tables 
WHERE table_name = 'major_department_user';  -- Should return empty
```

## 🎯 Benefits

1. **Simpler Data Model**
   - User chỉ liên kết với Major
   - Major tự biết thuộc Department nào

2. **Better Performance**
   - Ít joins hơn khi query
   - Indexes hiệu quả hơn

3. **Easier to Maintain**
   - Logic rõ ràng hơn
   - Ít foreign keys phải manage

4. **More Flexible**
   - Dễ thêm Major mới
   - Dễ change Department của Major

## 📚 Next Steps

Bước tiếp theo cần làm:

### 1. JWT Authentication ⏳
- [ ] Implement JWT middleware
- [ ] Extract user from token
- [ ] Use user's wallet for blockchain calls (thay vì admin)

### 2. Testing
- [ ] Test user creation với multiple majors
- [ ] Test major creation với department
- [ ] Test major update department
- [ ] Test cascade deletes

### 3. Documentation
- [ ] Update API documentation
- [ ] Add Postman/Swagger examples
- [ ] Document relationship queries

## 🐛 Known Issues

None currently. Schema migration successful!

## 📝 Notes

- Department ID trong Major là **nullable** → Major có thể không thuộc department nào
- User có thể có **multiple majors** qua `user_major` table
- Khi delete User, all `user_major` records cascade delete
- Khi delete Major, all `user_major` records cascade delete
- Khi delete Department, Major's `department_id` được set to NULL (SetNull policy)

---

**Migration Date:** 2025-10-28  
**Schema Version:** v2.0  
**Status:** ✅ Deployed Successfully

