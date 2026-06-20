# Contributing to proctui

Thank you for wanting to contribute to **proctui**! Whether you are fixing a bug, adding a feature, or improving documentation, your help is appreciated.

## How to Contribute

1. **Fork** the repository.
2. Create a new branch for your changes: `git checkout -b fix/my-fix-name`.
3. Make your changes and ensure they compile.
4. Submit a **Pull Request (PR)**.

## Code Standards

Since this is a Rust project, please follow these steps before submitting:

1. **Format code:**

    ```bash
    cargo fmt
    ```

2. **Lint code:**

    ```bash
    cargo clippy --all-targets --all-features -- -D warnings
    ```

3. **Run tests:**

    ```bash
    cargo test
    ```

## License

By contributing to proctui, you agree that your contributions will be licensed under the GNU General Public License v3.0 (GPL-3.0) found in the LICENSE file.

Your contributions become part of the project and must remain open source under GPL-3.0 if distributed.

Happy coding!
