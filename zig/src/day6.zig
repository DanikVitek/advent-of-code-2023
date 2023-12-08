pub const part1 = @import("day6/part1.zig");
pub const part2 = @import("day6/part2.zig");

const std = @import("std");
const Allocator = std.mem.Allocator;
const File = std.fs.File;
const printNanos = @import("main.zig").printNanos;

pub fn run(allocator: Allocator, stdout: File.Writer) !void {
    const input = @embedFile("day6/input.txt");

    {
        var timer = try std.time.Timer.start();
        const result = try part1.run(allocator, input);
        const time = timer.lap();
        try stdout.print("Day 6 Part 1: {d}\t(Time: ", .{result});
        try printNanos(stdout, time);
        try stdout.print(")\n", .{});
    }
    {
        var timer = try std.time.Timer.start();
        const result = try part2.run(allocator, input);
        const time = timer.lap();
        try stdout.print("Day 6 Part 2: {d}\t(Time: ", .{result});
        try printNanos(stdout, time);
        try stdout.print(")\n", .{});
    }
}

test {
    _ = part1;
    _ = part2;
}
