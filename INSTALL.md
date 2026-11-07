# FAZORA Android APK installation

## Download from GitHub Actions

1. Open the repository Actions tab.
2. Select the latest successful Android APK workflow run.
3. Under Artifacts, download FAZORA-Android-arm64-v8a for a physical Android phone (HONOR X7d is ARM64), or FAZORA-Android-x86_64 for an x86_64 emulator.
4. Extract the artifact ZIP. Keep the APK and its .sha256 file together.
5. Verify the checksum on Linux/macOS:

```bash
sha256sum -c FAZORA-arm64-v8a.apk.sha256
```

Compare the computed hash with the matching .sha256 file. On Windows use Get-FileHash -Algorithm SHA256.

## Install on an Android phone

- Transfer the ARM64 APK to the phone or download it directly from the Actions artifact.
- Open the APK in Files/Downloads and allow installation from that source if Android asks.
- Install and launch FAZORA.

With Android Debug Bridge:

```bash
adb install FAZORA-arm64-v8a.apk
```

## Important: CI signing key

CI builds are signed with a temporary debug keystore generated for that workflow run. The key is deliberately not persisted in the repository or GitHub secrets. A later workflow run has a different signing identity, so Android rejects it as an in-place update. Uninstall the previous CI build before installing an APK from a different run; this removes that app's local data.

The build also verifies 16 KiB ZIP and ELF alignment for Android 15+ page-size compatibility. These APKs are for testing and sideloading, not Play Store distribution. A production update channel requires one owner-controlled private signing key stored as GitHub Actions secrets.

## Generate a private release keystore (do not commit it)

Run locally on a trusted computer. keytool prompts for passwords and certificate details:

```bash
keytool -genkeypair -v \
  -keystore fazora-release.p12 \
  -storetype PKCS12 \
  -alias fazora-release \
  -keyalg RSA -keysize 3072 -validity 10000
```

Inspect the certificate:

```bash
keytool -list -v -keystore fazora-release.p12 -storetype PKCS12
```

Back up the keystore and passwords offline. Never commit the keystore, print passwords in CI logs, or reuse the temporary CI debug key for production. Adding release-signing secrets and publishing a production release are separate owner-approval gates.
