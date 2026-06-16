use cocoa::base::id;
use gpui::HapticFeedbackStyle;
use objc::{class, msg_send, sel, sel_impl};

/// macOS haptic feedback using [`NSHapticFeedbackManager`].
///
/// Delivers transient taps through the Force Touch trackpad (macOS 10.11+).
/// No engine lifecycle — fire and forget. On machines without haptic hardware,
/// calls are silently ignored by AppKit.
pub(crate) struct MacHaptics {
    supported: bool,
}

impl MacHaptics {
    pub fn new(headless: bool) -> Self {
        Self {
            supported: !headless,
        }
    }

    pub fn supported(&self) -> bool {
        self.supported
    }

    pub fn play(&self, style: HapticFeedbackStyle) {
        if !self.supported {
            return;
        }

        /// <https://developer.apple.com/documentation/appkit/nshapticfeedbackmanager/feedbackpattern>
        #[allow(dead_code)]
        mod feedback_pattern {
            pub const GENERIC: isize = 0;
            pub const ALIGNMENT: isize = 1;
            pub const LEVEL_CHANGE: isize = 2;
        }

        let pattern: isize = match style {
            HapticFeedbackStyle::Generic => feedback_pattern::GENERIC,
            HapticFeedbackStyle::Alignment => feedback_pattern::ALIGNMENT,
            HapticFeedbackStyle::LevelChange => feedback_pattern::LEVEL_CHANGE,
        };

        /// <https://developer.apple.com/documentation/appkit/nshapticfeedbackmanager/performancetime>
        const PERFORMANCE_TIME_NOW: usize = 1;

        // Safety: NSHapticFeedbackManager is always available on macOS 10.11+.
        // All Platform trait methods run on the main thread.
        unsafe {
            let manager: id = msg_send![class!(NSHapticFeedbackManager), defaultPerformer];
            let _: () = msg_send![
                manager,
                performFeedbackPattern: pattern
                performanceTime: PERFORMANCE_TIME_NOW
            ];
        }
    }
}
