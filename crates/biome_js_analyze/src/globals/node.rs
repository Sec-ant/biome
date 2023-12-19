/// Sorted array of Node builtin
pub const BUILTIN: [&str; 29] = [
    "AbortController",
    "AbortSignal",
    "Buffer",
    "DOMException",
    "Event",
    "EventTarget",
    "Intl",
    "MessageChannel",
    "MessageEvent",
    "MessagePort",
    "TextDecoder",
    "TextEncoder",
    "URL",
    "URLSearchParams",
    "atob",
    "btoa",
    "clearImmediate",
    "clearInterval",
    "clearTimeout",
    "console",
    "fetch",
    "global",
    "performance",
    "process",
    "queueMicrotask",
    "setImmediate",
    "setInterval",
    "setTimeout",
    "structuredClone",
];

pub const NODE: [&str; 34] = [
    "AbortController",
    "AbortSignal",
    "Buffer",
    "DOMException",
    "Event",
    "EventTarget",
    "Intl",
    "MessageChannel",
    "MessageEvent",
    "MessagePort",
    "TextDecoder",
    "TextEncoder",
    "URL",
    "URLSearchParams",
    "__dirname",
    "__filename",
    "atob",
    "btoa",
    "clearImmediate",
    "clearInterval",
    "clearTimeout",
    "console",
    "exports",
    "fetch",
    "global",
    "module",
    "performance",
    "process",
    "queueMicrotask",
    "require",
    "setImmediate",
    "setInterval",
    "setTimeout",
    "structuredClone",
];

pub const NODE_BUILTINS: &[&str] = &[
    "assert",
    "assert/strict",
    "async_hooks",
    "buffer",
    "child_process",
    "cluster",
    "console",
    "constants",
    "crypto",
    "dgram",
    "diagnostics_channel",
    "dns",
    "dns/promises",
    "domain",
    "events",
    "fs",
    "fs/promises",
    "http",
    "http2",
    "https",
    "inspector",
    "inspector/promises",
    "module",
    "net",
    "node:buffer",
    "node:cares_wrap",
    "node:config",
    "node:constants",
    "node:contextify",
    "node:fs",
    "node:fs/promises",
    "node:fs_event_wrap",
    "node:icu",
    "node:inspector",
    "node:js_stream",
    "node:os",
    "node:pipe_wrap",
    "node:process_wrap",
    "node:spawn_sync",
    "node:stream_wrap",
    "node:tcp_wrap",
    "node:test",
    "node:test/reporters",
    "node:timers",
    "node:timers/promises",
    "node:tls_wrap",
    "node:tty_wrap",
    "node:udp_wrap",
    "node:uv",
    "node:zlib",
    "os",
    "path",
    "path/posix",
    "path/win32",
    "perf_hooks",
    "process",
    "punycode",
    "querystring",
    "readline",
    "readline/promises",
    "repl",
    "stream",
    "stream/consumers",
    "stream/promises",
    "stream/web",
    "string_decoder",
    "sys",
    "timers",
    "timers/promises",
    "tls",
    "trace_events",
    "tty",
    "url",
    "util",
    "util/types",
    "v8",
    "vm",
    "wasi",
    "worker_threads",
    "zlib",
];

/// Sorted array of CommonJs builtin
pub const COMMON_JS: [&str; 4] = ["exports", "global", "module", "require"];

#[test]
fn test_order() {
    for items in BUILTIN.windows(2) {
        assert!(items[0] < items[1], "{} < {}", items[0], items[1]);
    }
    for items in COMMON_JS.windows(2) {
        assert!(items[0] < items[1], "{} < {}", items[0], items[1]);
    }

    for items in NODE_BUILTINS.windows(2) {
        assert!(items[0] < items[1], "{} < {}", items[0], items[1]);
    }
}
