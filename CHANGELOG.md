# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Bottom navigation bar layout matching ClassTop MainWindow design
- Vue Router integration for frontend navigation
- Route guards for authentication protection
- Keep-alive caching for better performance

### Fixed
- Rust code formatting issues to pass CI checks
- User model timestamp type mismatch in integration tests
- All model timestamp type mismatches (created_at, last_sync fields)
- start.sh project compilation check logic
- User authentication database type mismatch issues
- Nginx configuration and permission issues

## [1.2.0] - 2025-11-01

### Added
- Frontend authentication integration with login/register UI
- Complete deployment configuration with Nginx
- Vue 3 frontend with MDUI components
- Login and registration views with form validation
- Dashboard, clients, and data management views
- Automatic database migrations on startup

### Changed
- Improved error handling and user feedback
- Enhanced security with JWT token validation
- Better frontend-backend integration

### Fixed
- Nginx configuration structure and permissions
- Log and PID file permission issues on macOS/Linux
- Font file caching support in Nginx

## [1.1.0] - 2025-11-01

### Added
- User authentication system with JWT tokens
- Password hashing with bcrypt
- Pagination support for API endpoints
- Structured logging with tracing and tracing-subscriber
- Rate limiting with actix-governor
- API documentation with Swagger UI and ReDoc
- CORS configuration for cross-origin requests
- Environment-based configuration
- Health check endpoint
- User registration and login endpoints

### Changed
- Improved error handling across all endpoints
- Better database connection management
- Enhanced API response structure with consistent format

### Security
- Implemented JWT-based authentication
- Added password hashing for user credentials
- Rate limiting to prevent abuse

## [1.0.0] - 2025-10-26

### Added
- SQL Server native driver implementation (mssql-driver)
- TDS protocol support with Pre-Login, Login7, and Token Stream parsing
- Row data value parsing and transaction support
- SQL query execution functionality
- Complete SQL Server authentication flow

### Changed
- Updated README with SQL Server Developer Build availability

### Fixed
- Code formatting and Clippy warnings

## [0.9.0] - 2025-10-18

### Added
- WebSocket real-time control system
- CCTV (Camera) management functionality
- LMS (Light Management Service) instance management
- WebSocket connection manager
- Client registry and command logging

### Changed
- Enhanced client management capabilities
- Improved real-time communication

## [0.8.0] - 2025-10-09

### Added
- Client adaptation documentation
- SQL Server support preparation

### Changed
- Updated README and CLAUDE.md documentation

## [0.7.0] - 2025-10-08

### Added
- Complete GitHub Actions CI/CD workflow
- CODEOWNERS file for code review
- Frontend and backend testing
- Security audit workflow
- Integration checks

### Changed
- Improved .gitignore configuration
- Better dependency management

### Fixed
- CI issues and optimized dependency configuration
- Clippy warnings

## [0.1.0] - 2025-10-07

### Added
- Initial ClassTop Management Server implementation
- PostgreSQL database integration
- RESTful API endpoints for client management
- Course and schedule data synchronization
- Client registration and data upload
- Statistics and analytics endpoints
- Rust backend with Actix-Web framework
- Vue 3 frontend with MDUI components
- Basic CRUD operations for courses and schedules
- CLAUDE.md development guide

### Changed
- Migrated frontend from vanilla HTML to Vue 3
- Updated platform support documentation

[Unreleased]: https://github.com/Zixiao-System/Classtop-Management-Server/compare/v1.2.0...HEAD
[1.2.0]: https://github.com/Zixiao-System/Classtop-Management-Server/compare/v1.1.0...v1.2.0
[1.1.0]: https://github.com/Zixiao-System/Classtop-Management-Server/compare/v1.0.0...v1.1.0
[1.0.0]: https://github.com/Zixiao-System/Classtop-Management-Server/compare/v0.9.0...v1.0.0
[0.9.0]: https://github.com/Zixiao-System/Classtop-Management-Server/compare/v0.8.0...v0.9.0
[0.8.0]: https://github.com/Zixiao-System/Classtop-Management-Server/compare/v0.7.0...v0.8.0
[0.7.0]: https://github.com/Zixiao-System/Classtop-Management-Server/compare/v0.1.0...v0.7.0
[0.1.0]: https://github.com/Zixiao-System/Classtop-Management-Server/releases/tag/v0.1.0