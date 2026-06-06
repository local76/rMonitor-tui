Contributing to rMonitor

We are thrilled that you want to help improve rMonitor! Contributions from the community are what make open-source projects so special. Please follow these guidelines to make sure your contribution matches the style and quality standards of the project.

Developer Environment Setup
To build and test rMonitor locally:
  Make sure you have the standard Rust toolchain installed.
  Clone this repository.
  Check code formatting:
    cargo fmt --check
  Run standard compiler lints:
    cargo clippy
  Test the debug build:
    cargo run
  Build and package the final release with the custom resource compiler script:
    .\build.bat

Pull Request Process
  Fork the repository and create a new feature branch:
    git checkout -b feature/my-new-feature
  Write clean code and keep your changes focused.
  Make sure all compile checks and lints pass.
  Document any new features in the README.md or corresponding help manuals.
  Open a Pull Request detailing the purpose of your change and any design decisions you made.

TUI Design Principles
If you are modifying the user interface, please keep in mind:
  Aesthetics: We use high-contrast HSL/RGB tailored color themes. Do not use plain primaries (such as pure blue, pure red).
  Balance: Maintain four line layouts in the top statistics panels to keep the dashboard balanced.
  Compact Core Grid: Keep core layouts wrapped so they support very high core counts (up to 64+ logical cores) without breaking borders.
