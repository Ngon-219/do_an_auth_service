use ethers::prelude::*;

// ABI for DataStorage contract
abigen!(
    DataStorage,
    r#"[
        function registerStudent(address _studentAddress, string _studentCode, string _fullName, string _email) external returns (uint256)
        function registerStudentsBatch(address[] _addresses, string[] _studentCodes, string[] _fullNames, string[] _emails) external
        function getStudent(uint256 _studentId) external view returns (uint256 id, address walletAddress, string studentCode, string fullName, string email, bool isActive, uint256 registeredAt)
        function assignRole(address _user, uint8 _role) external
        event StudentRegistered(uint256 indexed studentId, address indexed studentAddress, string studentCode)
        event RoleAssigned(address indexed user, uint8 role)
    ]"#
);

