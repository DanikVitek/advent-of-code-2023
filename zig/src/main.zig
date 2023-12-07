const std = @import("std");
const builtin = @import("builtin");

const days = [_]type{ @import("day4.zig"), @import("day5.zig") };

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    if (builtin.os.tag == .windows) {
        _ = std.os.windows.kernel32.SetConsoleOutputCP(65001); // set the console to UTF-8 codepage
    }
    const stdout = std.io.getStdOut().writer();

    inline for (days) |day| {
        try day.run(allocator, stdout);
        _ = arena.reset(.free_all);
    }
}

pub fn printNanos(stdout: std.fs.File.Writer, nanos: u64) !void {
    if (nanos < 1000) {
        try stdout.print("{d}ns", .{nanos});
    } else {
        const nanos_f: f64 = @floatFromInt(nanos);
        if (nanos < 1000000) {
            try stdout.print("{d}μs", .{nanos_f / 1000});
        } else if (nanos < 1000000000) {
            try stdout.print("{d}ms", .{nanos_f / 1000000});
        } else {
            try stdout.print("{d}s", .{nanos_f / 1000000000});
        }
    }
}

test {
    _ = @import("day4.zig");
}
