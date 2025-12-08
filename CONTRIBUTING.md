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
```
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

## Testing
Describe the tests you ran

## Screenshots (if applicable)
```

## Testing

Before submitting a PR:

```bash
# Run Rust tests
cargo test --all

# Run Python tests
python -m pytest

# Run system verification
python test_system.py
```

## Documentation

- Update relevant documentation files
- Add code comments for complex logic
- Update README if adding features
- Include examples in docstrings

## Questions?

Feel free to open an issue for questions or discussions about contributions.

---

**Thank you for contributing to AxiomHive Sovereign Manifold!**

