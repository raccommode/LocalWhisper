/// macOS permission checks for microphone and accessibility.
/// Uses native APIs (AVCaptureDevice, AXIsProcessTrusted) called from
/// the app's own process so the checks reflect LocalWhisper's permissions.
/// On non-macOS platforms, permissions are always reported as granted.

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct PermissionStatus {
    pub microphone: bool,
    pub accessibility: bool,
}

#[cfg(target_os = "macos")]
mod macos {

    // ── Accessibility (ApplicationServices framework) ──

    #[link(name = "ApplicationServices", kind = "framework")]
    extern "C" {
        fn AXIsProcessTrusted() -> bool;
        fn AXIsProcessTrustedWithOptions(options: *const std::ffi::c_void) -> bool;
    }

    #[link(name = "CoreFoundation", kind = "framework")]
    extern "C" {
        fn CFStringCreateWithCString(
            alloc: *const std::ffi::c_void,
            c_str: *const i8,
            encoding: u32,
        ) -> *const std::ffi::c_void;
        fn CFDictionaryCreate(
            allocator: *const std::ffi::c_void,
            keys: *const *const std::ffi::c_void,
            values: *const *const std::ffi::c_void,
            num_values: isize,
            key_callbacks: *const std::ffi::c_void,
            value_callbacks: *const std::ffi::c_void,
        ) -> *const std::ffi::c_void;
        fn CFRelease(cf: *const std::ffi::c_void);

        static kCFTypeDictionaryKeyCallBacks: std::ffi::c_void;
        static kCFTypeDictionaryValueCallBacks: std::ffi::c_void;
        static kCFBooleanTrue: *const std::ffi::c_void;
    }

    const KCF_STRING_ENCODING_UTF8: u32 = 0x08000100;

    /// Check microphone permission by trying to open a cpal input stream.
    /// This is more reliable than AVCaptureDevice on ARM64e/macOS 26+
    /// because it tests the same CoreAudio path we actually use for recording.
    pub fn check_microphone() -> bool {
        use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

        let host = cpal::default_host();
        let device = match host.default_input_device() {
            Some(d) => d,
            None => return false,
        };
        let config = match device.default_input_config() {
            Ok(c) => c,
            Err(_) => return false,
        };
        let stream_config = cpal::StreamConfig {
            channels: config.channels(),
            sample_rate: config.sample_rate(),
            buffer_size: cpal::BufferSize::Default,
        };
        match device.build_input_stream(
            &stream_config,
            |_data: &[f32], _: &cpal::InputCallbackInfo| {},
            |_err| {},
            None,
        ) {
            Ok(stream) => {
                // If we can play, permission is granted
                let ok = stream.play().is_ok();
                drop(stream);
                ok
            }
            Err(_) => false,
        }
    }

    /// Request microphone access. Triggers the macOS TCC permission dialog by
    /// briefly opening a CoreAudio input stream via cpal. This avoids the
    /// ARM64e pointer-authentication issues with hand-rolled ObjC blocks
    /// that `AVCaptureDevice requestAccessForMediaType:completionHandler:` needs.
    pub fn request_microphone() {
        use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

        std::thread::spawn(|| {
            let host = cpal::default_host();
            if let Some(device) = host.default_input_device() {
                if let Ok(config) = device.default_input_config() {
                    let stream_config = cpal::StreamConfig {
                        channels: config.channels(),
                        sample_rate: config.sample_rate(),
                        buffer_size: cpal::BufferSize::Default,
                    };
                    if let Ok(stream) = device.build_input_stream(
                        &stream_config,
                        |_data: &[f32], _: &cpal::InputCallbackInfo| {},
                        |_err| {},
                        None,
                    ) {
                        let _ = stream.play();
                        // Keep alive briefly so TCC has time to show the dialog
                        std::thread::sleep(std::time::Duration::from_millis(200));
                        drop(stream);
                    }
                }
            }
        });
    }

    /// Check accessibility permission using the native AXIsProcessTrusted() API.
    pub fn check_accessibility() -> bool {
        unsafe { AXIsProcessTrusted() }
    }

    /// Prompt for accessibility permission.
    /// AXIsProcessTrustedWithOptions with kAXTrustedCheckOptionPrompt = true
    /// shows the system alert AND opens System Settings > Accessibility.
    pub fn prompt_accessibility() {
        unsafe {
            let key_str = b"AXTrustedCheckOptionPrompt\0";
            let key = CFStringCreateWithCString(
                std::ptr::null(),
                key_str.as_ptr() as *const i8,
                KCF_STRING_ENCODING_UTF8,
            );

            let keys = [key];
            let values = [kCFBooleanTrue];

            let options = CFDictionaryCreate(
                std::ptr::null(),
                keys.as_ptr(),
                values.as_ptr(),
                1,
                &kCFTypeDictionaryKeyCallBacks as *const _ as *const std::ffi::c_void,
                &kCFTypeDictionaryValueCallBacks as *const _ as *const std::ffi::c_void,
            );

            AXIsProcessTrustedWithOptions(options);

            CFRelease(options);
            CFRelease(key);
        }
    }

    pub fn open_accessibility_settings() {
        prompt_accessibility();
    }
}

#[cfg(target_os = "macos")]
pub fn check_permissions() -> PermissionStatus {
    PermissionStatus {
        microphone: macos::check_microphone(),
        accessibility: macos::check_accessibility(),
    }
}

#[cfg(target_os = "macos")]
pub fn request_microphone_permission() {
    macos::request_microphone();
}

#[cfg(target_os = "macos")]
pub fn open_accessibility_preferences() {
    macos::open_accessibility_settings();
}

// ── Non-macOS fallbacks ──

#[cfg(not(target_os = "macos"))]
pub fn check_permissions() -> PermissionStatus {
    PermissionStatus {
        microphone: true,
        accessibility: true,
    }
}

#[cfg(not(target_os = "macos"))]
pub fn request_microphone_permission() {}

#[cfg(not(target_os = "macos"))]
pub fn open_accessibility_preferences() {}
