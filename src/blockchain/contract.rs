use ethers::prelude::*;

// ABI for DataStorage contract - Full version matching deployed contract
abigen!(
    DataStorage,
    r#"[
        function owner() external view returns (address)
        function factoryContract() external view returns (address)
        function changeOwner(address _newOwner) external
        function authorizeContract(address _contract) external
        function unauthorizeContract(address _contract) external
        function addManager(address _manager) external
        function removeManager(address _manager) external
        function getAllManagers() external view returns (address[])
        function getManagerCount() external view returns (uint256)
        function registerStudent(address _studentAddress, string _studentCode, string _fullName, string _email) external returns (uint256)
        function registerStudentsBatch(address[] _addresses, string[] _studentCodes, string[] _fullNames, string[] _emails) external
        function deactivateStudent(uint256 _studentId) external
        function activateStudent(uint256 _studentId) external
        function getStudent(uint256 _studentId) external view returns (uint256 id, address walletAddress, string studentCode, string fullName, string email, bool isActive, uint256 registeredAt)
        function getStudentIdByAddress(address _address) external view returns (uint256)
        function getStudentIdByCode(string _studentCode) external view returns (uint256)
        function isActiveStudent(address _address) external view returns (bool)
        function assignRole(address _user, uint8 _role) external
        function getUserRole(address _user) external view returns (uint8)
        function hasRole(address _user, uint8 _role) external view returns (bool)
        function isTeacherOrAdmin(address _user) external view returns (bool)
        function getTotalStudents() external view returns (uint256)
        function getContractInfo() external view returns (address currentOwner, uint256 studentCount, uint256 managerCount)
        function userRoles(address) external view returns (uint8)
        function isRegistered(address) external view returns (bool)
        function isManager(address) external view returns (bool)
        function authorizedContracts(address) external view returns (bool)
        event OwnerChanged(address indexed oldOwner, address indexed newOwner)
        event ManagerAdded(address indexed manager)
        event ManagerRemoved(address indexed manager)
        event StudentRegistered(uint256 indexed studentId, address indexed studentAddress, string studentCode)
        event StudentDeactivated(uint256 indexed studentId)
        event RoleAssigned(address indexed user, uint8 role)
        event ContractAuthorized(address indexed contractAddress)
        event ContractUnauthorized(address indexed contractAddress)
    ]"#
);
