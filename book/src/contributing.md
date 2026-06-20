# Contributing

## Getting Started

1. Fork the repository
2. Clone your fork
3. Create a feature branch
4. Make your changes
5. Run tests
6. Submit a pull request

## Development Setup

```bash
git clone https://github.com/<your-fork>/seekr.git
cd seekr
cargo build
cargo test
```

## Code Style

- Use `cargo fmt` before committing
- Use `cargo clippy` and fix all warnings
- No `unwrap()` or `expect()` in production code
- All public items must have rustdoc documentation
- Every public API must have tests

## Testing

```bash
cargo test --all-targets
cargo fmt --check
cargo clippy --all-targets -- -D warnings
```

## Pull Request Process

1. Update documentation if needed
2. Add tests for new functionality
3. Ensure all CI checks pass
4. Request a review

## Reporting Issues

Use the GitHub issue templates for:
- Bug reports
- Feature requests

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
