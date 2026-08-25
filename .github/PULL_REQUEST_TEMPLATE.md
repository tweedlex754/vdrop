# What this changes

<!-- What behaviour is different after this, and why it needed to change. -->

# How it was verified

<!--
Tests that pass are necessary but rarely sufficient in this codebase - the
history in docs/DURUM.md is a list of defects that were green in tests and
still wrong in the running app. If the change is visible, say what you saw.
-->

- [ ] `cargo test --workspace` and `npm run test:front` pass
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` is clean
- [ ] If the change is visible in the interface, I looked at it
- [ ] If it touches downloading, I ran a real download through it

# Anything left undone

<!--
Scope you deliberately did not cover, or a limitation the change carries.
Writing it here is better than letting the next person discover it.
-->
