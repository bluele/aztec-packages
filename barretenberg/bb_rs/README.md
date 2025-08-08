# bb.rs

Rust bindings for Barretenberg C++ codebase.

## Build

```
# Build on your own machine
cargo build -vvvv

# Cross-compile for iOS
cargo build -vvvv --target aarch64-apple-ios

# Cross-compile for Android
cargo build -vvvv --target aarch64-linux-android
```

### Controlling Build Parallelism

If you encounter out-of-memory (OOM) errors during the CMake build process, you can control the number of parallel build jobs using environment variables:

```bash
# Option 1: Use CMAKE_BUILD_PARALLEL_LEVEL (standard CMake variable)
CMAKE_BUILD_PARALLEL_LEVEL=2 cargo build

# Option 2: Use BB_BUILD_JOBS (custom variable for this project)
BB_BUILD_JOBS=2 cargo build

# The build will default to using all available CPU cores if neither variable is set
```

The build script will display the number of parallel jobs being used:
```
warning: Using 2 parallel jobs for CMake build
```

## Known issues

### Missing `sys/random.h`

random.h is not available in the iOS SDK includes but it is available in the MacOS SDK includes. So you can copy it from `/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys` and paste it in `/Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/SDKs/iPhoneOS.sdk/usr/include/sys`. This will work, no compability issues, it's just not there for some reason.

You can also run `scripts/patcher.sh` to do this (you may need to run it as `sudo`).



