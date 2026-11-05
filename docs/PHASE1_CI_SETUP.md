# Phase 1 CI/CD setup

## Android signing

Generate a long-lived release keystore locally and keep the file and passwords private:

```bash
keytool -genkeypair -v \
  -keystore fazora-release.jks \
  -alias fazora \
  -keyalg RSA \
  -keysize 4096 \
  -validity 10000
```

Encode the keystore as a single-line Base64 value:

```bash
base64 -w 0 fazora-release.jks
```

Add these repository Actions secrets under Settings → Secrets and variables → Actions:

- `ANDROID_KEYSTORE_BASE64`: Base64 output above
- `ANDROID_KEYSTORE_PASSWORD`: keystore password
- `ANDROID_KEY_ALIAS`: `fazora`
- `ANDROID_KEY_PASSWORD`: key password

Never commit the keystore, its Base64 data, or passwords. Fork pull requests do not receive repository secrets; those builds use the ephemeral debug-key fallback.

## Branch protection

Repository owner must configure Settings → Rules → Rulesets (or Branches → Branch protection rules) for `main`:

- Require a pull request before merging.
- Require status checks to pass before merging.
- Select the required Phase 1 checks after they appear in the PR checks list.
- Require branches to be up to date before merging.
- Block force pushes and deletion.
- Do not allow bypass unless explicitly intended.

The GitHub App connection used for this work cannot be assumed to have repository administration rights, so branch protection is documented rather than changed automatically.

## Current validation scope

The workflows validate formatting, Clippy, workspace tests, dependency policy, advisory audit, docs, MSRV, Linux/Windows compilation, Android APK packaging/signing, and x86_64 emulator launch/logcat/screenshots. A green CI result is not a substitute for testing the signed release APK on a physical Android device.
