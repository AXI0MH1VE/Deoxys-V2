# Contributing to AxiomHive Sovereign Manifold

Thank you for your interest in contributing to AxiomHive! This document provides guidelines and instructions for contributing.

## Code of Conduct

By participating in this project, you agree to maintain a respectful and professional environment.

## Zero Entropy Law Compliance

All contributions must maintain the **Zero Entropy Law (C=0)**:

- ✅ All operations must be deterministic
- ✅ No random number generation without frozen seeds
- ✅ Temperature must default to 0.0
- ✅ All outputs must be verifiable
- ✅ Entropy count must equal 1

## ⚠️ CRITICAL: In-Process Execution Requirements

**MANDATORY READING:**

- **[AGENT_REQUIREMENTS.md](./AGENT_REQUIREMENTS.md)** - Zero OS command execution policy
- **[NETWORK_SAFETY.md](./NETWORK_SAFETY.md)** - Zero network operations policy

**ALL contributions must:**

- ✅ Use pure Rust implementations (no Python subprocess calls)
- ✅ Run all operations in-process (no `std::process::Command`)
- ✅ Have zero network dependencies (no HTTP, TCP, or socket operations)
- ✅ Pass verification commands before submitting PRs

**FORBIDDEN:**

- ❌ `std::process::Command` or any OS command execution
- ❌ External script execution (Python, shell, etc.)
- ❌ Network operations (HTTP requests, TCP sockets)
- ❌ Process spawning or system hooks

## Development Setup

1. **Fork the repository**
2. **Clone your fork:**

   ```bash
   git clone https://github.com/YOUR_USERNAME/Deoxys-V2.git
   cd Deoxys-V2
   ```

3. **Set up development environment:**

   ```bash
   ./setup.sh  # or setup.ps1 on Windows
   ```

4. **Create a branch:**

   ```bash
   git checkout -b feature/your-feature-name
   ```

## Contribution Areas

### High Priority

- Performance optimization
- Security enhancements
- Documentation improvements
- Test coverage expansion

### Medium Priority

- UI/UX improvements
- Additional features
- Bug fixes
- Code refactoring

## Coding Standards

### Rust

- Follow Rust style guidelines
- Use `cargo fmt` and `cargo clippy`
- Write comprehensive tests
- Document all public APIs

### Python

- Follow PEP 8 style guide
- Use type hints
- Write docstrings
- Include unit tests

### JavaScript

- Use modern ES6+ syntax
- Follow consistent naming conventions
- Comment complex logic
- Ensure accessibility

## Pull Request Process

1. **Update documentation** if needed
2. **Add tests** for new features
3. **Ensure all tests pass**
4. **Update CHANGELOG.md** with your changes
5. **Create a pull request** with a clear description

### PR Title Format

```text
[TYPE] Brief description
```

Types: `FEATURE`, `BUGFIX`, `DOCS`, `REFACTOR`, `PERF`

### PR Description Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Documentation update
- [ ] Performance improvement
- [ ] Code refactoring

## Zero Entropy Verification
- [ ] Maintains C=0 compliance
- [ ] All tests pass
- [ ] Documentation updated

## In-Process Execution Verification
- [ ] No `std::process::Command` usage
- [ ] No network operations
- [ ] All operations run in-process
- [ ] Passed verification commands (see AGENT_REQUIREMENTS.md)

## Testing
Describe the tests you ran

## Screenshots (if applicable)
```

## Testing

Before submitting a PR:

```bash
# Run Rust tests
cargo test --all

# Verify no OS commands (MANDATORY)
grep -r "std::process::Command" src-tauri/src/
grep -r "Command::new" src-tauri/src/
# Should return NO results

# Verify no network operations (MANDATORY)
grep -r "reqwest" src-tauri/src/
grep -r "TcpStream" src-tauri/src/
# Should return NO results (except in comments/docs)
```

## Documentation

- Update relevant documentation files
- Add code comments for complex logic
- Update README if adding features
- Include examples in docstrings

## Questions?

Feel free to open an issue for questions or discussions about contributions.

## Additional Resources

- **[AGENT_REQUIREMENTS.md](./AGENT_REQUIREMENTS.md)**: Detailed requirements for in-process execution
- **[NETWORK_SAFETY.md](./NETWORK_SAFETY.md)**: Network safety policies
- **[ARCHITECTURE.md](./ARCHITECTURE.md)**: System architecture and design
- **[README.md](./README.md)**: Project overview

---

**Thank you for contributing to AxiomHive Sovereign Manifold!**

*Remember: All operations must run in-process using pure Rust. See AGENT_REQUIREMENTS.md before making any changes.*
