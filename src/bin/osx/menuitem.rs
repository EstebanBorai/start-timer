use cocoa::appkit::NSApplicationActivationPolicy::NSApplicationActivationPolicyAccessory;
use cocoa::foundation::NSProcessInfo;
use cocoa::{
    appkit::{
        NSApp, NSApplication, NSMenu, NSMenuItem, NSSquareStatusItemLength, NSStatusBar,
        NSStatusItem, NSWindow,
    },
    base::{nil, selector},
    foundation::{NSAutoreleasePool, NSString},
};

pub struct MenuItem;

impl MenuItem {
    pub fn run() {
        unsafe {
            let _pool = NSAutoreleasePool::new(nil);
            let app = NSApp();

            app.setActivationPolicy_(NSApplicationActivationPolicyAccessory);
            app.activateIgnoringOtherApps_(cocoa::base::YES);

            let status_bar = NSStatusBar::systemStatusBar(app);
            let status_item = status_bar.statusItemWithLength_(NSSquareStatusItemLength);
            let title = NSString::alloc(nil).init_str("Start Timer");

            status_item.button().setTitle_(title);

            let quit_prefix = NSString::alloc(nil).init_str("Quit ");
            let quit_title =
                quit_prefix.stringByAppendingString_(NSProcessInfo::processInfo(nil).processName());
            let quit_action = selector("terminate:");
            let quit_key = NSString::alloc(nil).init_str("q");
            let quit_item = NSMenuItem::alloc(nil)
                .initWithTitle_action_keyEquivalent_(quit_title, quit_action, quit_key)
                .autorelease();

            let status_bar_menu = NSMenu::new(nil).autorelease();

            status_bar_menu.addItem_(quit_item);

            status_item.setMenu_(status_bar_menu);
            app.setMenu_(status_bar_menu);
            app.run();
        }
    }
}
