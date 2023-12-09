const std = @import("std");
const builtin = @import("builtin");
const mem = std.mem;
const math = std.math;
const fmt = std.fmt;
const Allocator = mem.Allocator;
const ArrayList = std.ArrayList;
const SplitIterator = mem.SplitIterator;

const NL = switch (builtin.os.tag) {
    .windows => "\r\n",
    else => '\n',
};

pub fn run(allocator: Allocator, input: []const u8) !u32 {
    _ = allocator;
    _ = input;
    @panic("Not implemented");
}

test "simple input" {
    const testing = std.testing;
    const input = @embedFile("test_input.txt");
    try testing.expectEqual(@as(u32, 288), try run(testing.allocator, input));
}
