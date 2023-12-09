pub const part1 = @import("day7/part1.zig");
pub const part2 = @import("day7/part2.zig");

const std = @import("std");
const Allocator = std.mem.Allocator;
const File = std.fs.File;
const printNanos = @import("main.zig").printNanos;

pub fn run(allocator: Allocator, stdout: File.Writer) !void {
    const input = @embedFile("day7/input.txt");

    {
        var timer = try std.time.Timer.start();
        const result = try part1.run(allocator, input);
        const time = timer.lap();
        try stdout.print("Day 7 Part 1: {d}\t(Time: ", .{result});
        try printNanos(stdout, time);
        try stdout.print(")\n", .{});
    }
    {
        var timer = try std.time.Timer.start();
        const result = try part2.run(allocator, input);
        const time = timer.lap();
        try stdout.print("Day 7 Part 2: {d}\t(Time: ", .{result});
        try printNanos(stdout, time);
        try stdout.print(")\n", .{});
    }
}

test {
    _ = @import("day7/part1.zig");
    _ = @import("day7/part2.zig");
}
