# Contributing to SpringOS

Terima kasih telah tertarik berkontribusi pada SpringOS!

## Code of Conduct

- Be respectful dan constructive
- Welcome diversity of opinion
- Focus on code quality, not personal matters
- Help others learn

## Getting Started

1. **Fork & Clone**
   ```bash
   git clone https://github.com/yourusername/springos.git
   cd springos
   ```

2. **Read Documentation**
   - Start with [ROADMAP.md](./ROADMAP.md)
   - Check current phase requirements
   - Understand architecture in [Architecture Overview](./architecture/OVERVIEW.md)

3. **Setup Development Environment**
   - Follow [Development Guide](./development/DEVELOPMENT.md)
   - Build and test locally

## Contribution Types

### Bug Fixes
1. Check [Issues](https://github.com/springos/springos/issues)
2. Comment if you'll work on it
3. Create feature branch
4. Submit pull request with test

### New Features
1. Check [ROADMAP.md](./ROADMAP.md) for current phase
2. Open issue to discuss
3. Wait for approval
4. Implement following guidelines
5. Submit PR with documentation

### Documentation
1. Edit `.md` files in `docs/`
2. Keep docs in sync with code
3. Add examples where helpful
4. Submit PR

## Code Guidelines

### Rust Style
- Follow Rust naming conventions
  - `snake_case` for functions, variables
  - `PascalCase` for types
  - `SCREAMING_SNAKE_CASE` for constants

- Document public APIs
  ```rust
  /// Initializes the GDT and loads it into the CPU.
  ///
  /// # Safety
  /// Must be called only once during kernel initialization.
  pub unsafe fn init_gdt() { }
  ```

- Mark unsafe blocks with SAFETY comments
  ```rust
  // SAFETY: The pointer is valid because...
  unsafe { }
  ```

### Assembly
- Use inline assembly sparingly
- Add comments explaining CPU instructions
- Keep architecture-specific code in `arch/` folder

### Comments
- Explain "why", not "what"
- Keep comments up-to-date with code
- Use `// TODO` for incomplete work

## Workflow

### 1. Create Feature Branch
```bash
git checkout -b feature/exception-handlers
```

### 2. Make Changes
- Follow code guidelines
- Test locally
- Keep commits atomic and logical

### 3. Commit Messages
```
Short summary (50 chars max)

Detailed explanation if needed. Wrap at 72 chars.
Reference issues: Fixes #123
```

### 4. Push & Create PR
```bash
git push origin feature/exception-handlers
```

In PR:
- Link related issues
- Describe changes clearly
- Mention any new dependencies
- Add testing instructions

### 5. Respond to Review
- Address feedback promptly
- Discuss disagreements respectfully
- Request re-review when ready

### 6. Merge
- Squash if requested
- Delete branch after merge
- Close related issues

## Testing

### Before Submitting PR
```bash
# Test compilation
cd kernel && cargo build --release

# Build ISO
make iso

# Run in QEMU
make run
```

### Adding Tests
- Add test code for critical functions
- Document test expectations
- Include edge cases

## Documentation

### Requirements
- Update relevant `.md` files
- Add architecture decisions to [design/](./design/)
- Update [ROADMAP.md](./ROADMAP.md) if changing timeline
- Add comments to complex code

### Structure
```
docs/
├── ROADMAP.md           # Overall roadmap
├── README.md            # Main docs index
├── architecture/        # Architecture docs
├── design/              # Detailed specs
├── development/         # Dev guides
├── api/                 # API documentation
└── CONTRIBUTING.md      # This file
```

## Areas Needing Help

### High Priority
1. Exception handlers (Phase 2)
2. Timer interrupt setup (Phase 2)
3. Context switching (Phase 2)
4. VGA driver (Phase 2)

### Medium Priority
1. Process management (Phase 3)
2. System calls (Phase 3)
3. Virtual memory (Phase 3)

### Lower Priority
1. Additional drivers
2. Filesystem support
3. Networking

## Getting Help

- **Questions:** Open GitHub discussion
- **Bug Reports:** Create issue with reproduction steps
- **Architecture:** Check [docs/design/](./design/)
- **API Questions:** See [docs/api/](./api/)

## Pull Request Checklist

- [ ] Follows code style guidelines
- [ ] All tests pass locally
- [ ] Documentation updated
- [ ] Commit messages are clear
- [ ] No breaking changes (without discussion)
- [ ] Related issues linked

## Licensing

By contributing, you agree that your code will be licensed under the same license as SpringOS (see [LICENSE](../LICENSE)).

## Questions?

- Check [docs/](./docs/) for answers
- Open issue for discussion
- Ask in PR comments

---

**Thank you for contributing to SpringOS!** 🚀
