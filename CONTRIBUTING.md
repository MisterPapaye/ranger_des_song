# Contributing to Ranger de Song

Thank you for your interest in contributing! This project welcomes contributions from developers of all skill levels.

## How to Contribute

### Reporting Bugs

1. Check if the bug has been reported in [Issues](https://github.com/yourname/ranger_de_song/issues)
2. Provide detailed reproduction steps
3. Include system information:
   - OS and version
   - Rust version (`rustc --version`)
   - Node version (`node --version`)
   - App version

### Suggesting Features

- Open an issue with the `[FEATURE]` prefix
- Describe the use case and expected behavior
- Include mockups or examples if applicable

### Code Contributions

1. **Fork** the repository
2. **Create** a feature branch: `git checkout -b feature/amazing-feature`
3. **Make** your changes following code style (see below)
4. **Test** your changes:
   - `cd src-tauri && cargo test`
   - `npm run type-check`
   - Manual testing in `npm run dev`
5. **Commit** with clear messages: `git commit -m "Add feature: description"`
6. **Push** your branch: `git push origin feature/amazing-feature`
7. **Create** a Pull Request with description and rationale

## Code Style

### Rust

```bash
# Format code
cd src-tauri && cargo fmt

# Lint code
cd src-tauri && cargo clippy -- -D warnings
```

### TypeScript/React

- Use 2-space indentation
- Use descriptive variable names
- Add JSDoc comments for complex functions
- Follow React hooks conventions (use* prefix)

## Development Setup

```bash
# Install git hooks (auto-format before commit)
npm install husky --save-dev
npx husky install
```

## PR Guidelines

1. Link related issues: "Fixes #123"
2. Keep PRs focused (one feature per PR)
3. Provide context and rationale
4. Update documentation if needed
5. Allow edits from maintainers

## Areas Needing Help

- 📝 Documentation improvements
- 🐛 Bug fixes
- 🎨 UI/UX improvements
- 🚀 Performance optimizations
- 🌍 Additional language support
- 🧪 Test coverage

## License

By contributing, you agree that your contributions will be licensed under the project's MIT License.

---

Questions? Open a discussion or reach out to the maintainers.
