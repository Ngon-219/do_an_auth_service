# JWT Authentication & Dynamic Blockchain Service Guide

## 🎯 Overview

System đã được update với:
1. ✅ **JWT Authentication** - Login và secure endpoints với JWT tokens
2. ✅ **Dynamic Blockchain Service** - Mỗi user call blockchain với private key riêng của họ

---

## 🔐 JWT Authentication Flow

### 1. Login
```bash
POST /api/v1/auth/login
Content-Type: application/json

{
  "email": "student@example.com",
  "password": "password123"
}
```

**Response:**
```json
{
  "access_token": "eyJhbGciOiJIUzI1NiIsInR...",
  "token_type": "Bearer",
  "expires_in": 86400,
  "user_id": "uuid-here",
  "email": "student@example.com",
  "role": "student"
}
```

### 2. Use Token for Protected Endpoints
```bash
GET /api/v1/profile
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR...
```

---

## 🔧 Implementation Details

### JWT Token Structure

**Claims:**
```rust
pub struct TokenClaims {
    pub user_id: String,      // User's UUID
    pub user_name: String,    // Full name
    pub iap: Option<usize>,   // Issued at provider (optional)
    pub iat: usize,           // Issued at timestamp
    pub exp: usize,           // Expiration timestamp
    pub role: UserRole,       // ADMIN | MANAGER | TEACHER | STUDENT
}
```

**Token Expiration:** 24 hours (86400 seconds)

**Secret Key:** Configured via `JWT_SECRET` env variable (fallback: "secret_key")

---

## 🌐 Dynamic Blockchain Service

### Old Way (Static - Admin Only):
```rust
// Tất cả calls dùng admin's private key
let blockchain = BLOCKCHAIN_SERVICES.get().unwrap();
blockchain.register_student(...).await?;
```

### New Way (Dynamic - User's Wallet):
```rust
// Mỗi user dùng private key riêng của họ
let user_id = auth.user_id()?;
let blockchain = get_user_blockchain_service(db, &user_id).await?;
blockchain.get_user_role(&wallet_address).await?;
```

---

## 🏗️ Architecture

### BlockchainService Factory

```rust
impl BlockchainService {
    /// Admin wallet - for system operations
    pub async fn new() -> Result<Self>
    
    /// Custom wallet - for user operations  
    pub async fn with_private_key(private_key: &str) -> Result<Self>
}
```

### Helper Functions

```rust
// Get user's private key from database
pub async fn get_user_private_key(
    db: &DatabaseConnection,
    user_id: &Uuid,
) -> Result<String>

// Create blockchain service for user
pub async fn get_user_blockchain_service(
    db: &DatabaseConnection,
    user_id: &Uuid,
) -> Result<BlockchainService>
```

---

## 📚 Usage Examples

### Example 1: Protected Route với JWT
```rust
use crate::extractor::AuthClaims;

pub async fn get_profile(
    auth: AuthClaims,  // ← Automatically validates JWT
) -> Result<...> {
    // Get user_id from token
    let user_id = auth.user_id()?;
    
    // Get user info from DB
    let user = user::Entity::find()
        .filter(user::Column::UserId.eq(user_id))
        .one(db)
        .await?;
    
    Ok(...)
}
```

### Example 2: Call Blockchain với User's Wallet
```rust
pub async fn get_profile(
    auth: AuthClaims,
) -> Result<...> {
    let db = DATABASE_CONNECTION.get().unwrap();
    let user_id = auth.user_id()?;
    
    // Create blockchain service with user's private key
    let user_blockchain = get_user_blockchain_service(db, &user_id).await?;
    
    // Call contract using user's wallet
    let role = user_blockchain
        .get_user_role(&wallet_address)
        .await?;
    
    let is_active = user_blockchain
        .is_active_student(&wallet_address)
        .await?;
    
    Ok(ProfileResponse { role, is_active, ... })
}
```

### Example 3: Admin Operations (vẫn dùng Admin Wallet)
```rust
pub async fn create_user(...) -> Result<...> {
    let blockchain = BLOCKCHAIN_SERVICES.get().unwrap();
    
    // Admin operations như register student, assign role
    // vẫn dùng admin wallet vì cần permissions cao
    blockchain.register_student(...).await?;
    blockchain.assign_role(...).await?;
    
    Ok(...)
}
```

---

## 🔒 Security Considerations

### 1. Private Key Storage
- Private keys stored in `wallet` table
- **⚠️ IMPORTANT**: In production, encrypt private keys before storing!
- Consider using AWS KMS, HashiCorp Vault, or similar

### 2. JWT Secret
```bash
# In production, use strong random secret
JWT_SECRET=$(openssl rand -base64 32)
```

### 3. Token Expiration
- Current: 24 hours
- Consider shorter for production (e.g., 1 hour)
- Implement refresh tokens for better UX

### 4. HTTPS Only
- Always use HTTPS in production
- Never send JWT over unencrypted connections

---

## 🚀 API Endpoints

### Public Endpoints (No JWT Required)
- `POST /api/v1/auth/login` - Login
- `POST /api/v1/users` - Register user (admin operation)
- `GET /api/v1/health` - Health check

### Protected Endpoints (JWT Required)
- `GET /api/v1/profile` - Get current user profile
- All blockchain operations when using user's wallet
- Future endpoints requiring authentication

---

## 🧪 Testing the Flow

### 1. Create a User
```bash
curl -X POST http://localhost:8081/api/v1/users \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "Test",
    "last_name": "User",
    "email": "test@example.com",
    "password": "password123",
    "role": "student",
    "student_code": "SV001",
    ...
  }'
```

### 2. Login
```bash
curl -X POST http://localhost:8081/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "password": "password123"
  }'
```

**Save the `access_token` from response**

### 3. Access Protected Endpoint
```bash
curl -X GET http://localhost:8081/api/v1/profile \
  -H "Authorization: Bearer YOUR_ACCESS_TOKEN_HERE"
```

**Expected Response:**
```json
{
  "user_id": "uuid",
  "first_name": "Test",
  "last_name": "User",
  "email": "test@example.com",
  "role": "student",
  "wallet_address": "0x...",
  "blockchain_role": 1,
  "is_active": true
}
```

---

## 🔍 AuthClaims Extractor

### How It Works
```rust
impl FromRequestParts for AuthClaims {
    async fn from_request_parts(parts: &mut Parts, _state: &S) 
        -> Result<Self, Self::Rejection> 
    {
        // 1. Extract Authorization header
        let bearer_token = extract_bearer_token(parts)?;
        
        // 2. Decode and validate JWT
        let claims = jwt_manager.decode_jwt(token)?;
        
        // 3. Verify user exists in database
        let user = UserModel::find()
            .filter(user::Column::UserId.eq(claims.user_id))
            .one(db).await?;
        
        // 4. Return claims
        Ok(AuthClaims(claims))
    }
}
```

### Helper Methods
```rust
impl AuthClaims {
    /// Get user_id as UUID
    pub fn user_id(&self) -> Result<Uuid, String>
    
    /// Get claims reference
    pub fn claims(&self) -> &TokenClaims
}
```

---

## 🛠️ Configuration

### Environment Variables
```env
# Database
DATABASE_URL=postgresql://user:pass@localhost/dbname

# Blockchain
BLOCKCHAIN_RPC_URL=https://sepolia.infura.io/v3/YOUR_KEY
DATA_STORAGE_CONTRACT_ADDRESS=0x1fB07a31906c8CE60b372079355E7077769Eb147
ADMIN_PRIVATE_KEY=0xYOUR_ADMIN_PRIVATE_KEY

# JWT
JWT_SECRET=your-super-secret-key-here

# Server
PORT=8081
SWAGGER_ENABLED=true
LOG_LEVEL=info
```

---

## 📊 When to Use Admin vs User Wallet

| Operation | Wallet | Reason |
|-----------|--------|--------|
| Register Student | Admin | Requires `onlyAdmin` permission |
| Add Manager | Admin | Requires `onlyAdmin` permission |
| Assign Role | Admin/Owner | Requires `onlyOwner` permission |
| Get User Role | User | Read-only, user can check own role |
| Check Active Status | User | Read-only, user can check own status |
| Get Profile | User | User-specific data |
| Sign Documents | Manager | Manager uses own wallet |
| Submit Votes | Student | Student uses own wallet |

---

## 🔄 Migration from Static to Dynamic

### Before
```rust
// All handlers used global admin wallet
pub async fn some_handler() -> Result<...> {
    let blockchain = BLOCKCHAIN_SERVICES.get().unwrap();
    blockchain.some_operation(...).await?;
}
```

### After
```rust
// Handlers can use user's wallet when needed
pub async fn some_handler(auth: AuthClaims) -> Result<...> {
    let db = DATABASE_CONNECTION.get().unwrap();
    let user_id = auth.user_id()?;
    
    // Use user's wallet for user-specific operations
    let blockchain = get_user_blockchain_service(db, &user_id).await?;
    blockchain.some_operation(...).await?;
}
```

---

## ⚡ Performance Considerations

### Caching
Consider caching blockchain service instances:
```rust
// Per-request caching (simple)
thread_local! {
    static BLOCKCHAIN_CACHE: RefCell<HashMap<Uuid, BlockchainService>> 
        = RefCell::new(HashMap::new());
}

// Or use connection pooling pattern
```

### Database Queries
- Private key fetch is optimized with indexed `user_id`
- Consider caching frequently accessed wallets

---

## 🚨 Error Handling

### JWT Errors
- `401 Unauthorized` - Missing or invalid token
- `403 Forbidden` - Valid token but insufficient permissions

### Blockchain Errors
- `500 Internal Server Error` - Blockchain connection issues
- `404 Not Found` - Wallet not found for user

### Example Error Response
```json
{
  "error": "Invalid or expired token"
}
```

---

## 📝 TODO / Future Improvements

- [ ] Implement refresh tokens
- [ ] Add rate limiting per user
- [ ] Encrypt private keys in database
- [ ] Add token blacklist for logout
- [ ] Implement role-based access control middleware
- [ ] Add audit logging for blockchain operations
- [ ] Connection pooling for blockchain services
- [ ] Add WebSocket support with JWT authentication

---

## 🎓 Summary

### What We Achieved:

1. **JWT Authentication**
   - Login endpoint returns JWT token
   - Protected endpoints require valid JWT
   - Token contains user_id and role

2. **Dynamic Blockchain Service**
   - Each user calls blockchain with their own wallet
   - Admin wallet still used for admin operations
   - Helper functions to get user's blockchain service

3. **Security**
   - JWT validation on every protected request
   - User verification against database
   - Role-based permissions ready for implementation

4. **Example Implementation**
   - `/api/v1/profile` demonstrates full flow
   - Login → Get Token → Call Blockchain with User's Wallet

**Status:** ✅ Fully Implemented and Tested

---

**Last Updated:** 2025-10-28  
**Version:** 1.0  
**Author:** Ho1m3s & AI Assistant

