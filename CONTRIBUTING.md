# Contributing to DeckGym

Thank you for considering contributing to DeckGym! We welcome contributions from everyone, regardless of your programming experience. Whether you're new to coding, learning Rust, or wanting to practice modern AI-assisted development, this is a great place to start.

## New to Programming?

Don't let inexperience hold you back! This project is an excellent opportunity to learn:
- **Rust programming** through hands-on contributions
- **AI-assisted coding** with modern tools that help you write better code faster
- **Open source collaboration** and professional development workflows

We encourage using AI tools like **Claude Code** (recommended) to help you understand the codebase, write code, and learn best practices along the way.

## How to Contribute

### Getting Started with Claude Code (Recommended)

If you're using [Claude Code](https://claude.com/claude-code), start your session by asking it to familiarize itself with the project:

```
Read all *.md files in the repo
```

Then you can ask it to implement features for you. For example:

```
Implement Mantyke's attack
```

Claude Code will guide you through the implementation, explain the code, and help you understand the patterns used in the project.

### Standard Git Workflow

1. **Fork the repository**: Click the "Fork" button at the top right of the repository page.
2. **Clone your fork**:
   ```sh
   git clone https://github.com/your-username/deckgym-core.git
   ```
3. **Create a branch**:
   ```sh
   git checkout -b my-feature-branch
   ```
4. **Make your changes**: Implement your feature or bug fix (see main [README](./README.md) for more information).
   - If using Claude Code or another AI assistant, describe what you want to implement and let it help guide you
   - Don't hesitate to ask questions about the codebase or Rust syntax
5. **Commit your changes**:
   ```sh
   git commit -am 'Add new feature'
   ```
6. **Push to your branch**:
   ```sh
   git push origin my-feature-branch
   ```
7. **Create a Pull Request**: Go to the repository on GitHub and click "New Pull Request".

## Reporting Issues

If you find a bug or have a feature request, please create an issue on GitHub. Provide as much detail as possible to help us understand and address the issue.

## Code Quality Checks

Before submitting your pull request, make sure your code passes these checks (CI will run them automatically):

**Format your code:**
```sh
cargo fmt
```

**Run the linter:**
```sh
cargo clippy --features tui -- -D warnings
```

**Run the test suite:**
```sh
cargo test --features tui
```

Thank you for your contributions!
