pub const part1 = @import("day5/part1.zig");
pub const part2 = @import("day5/part2.zig");

const std = @import("std");
const Allocator = std.mem.Allocator;
const File = std.fs.File;
const printNanos = @import("main.zig").printNanos;

pub fn run(allocator: Allocator, stdout: File.Writer) !void {
    const input = @embedFile("day5/input.txt");

    {
        var timer = try std.time.Timer.start();
        const result = try part1.run(allocator, input);
        const time = timer.lap();
        try stdout.print("Day 5 Part 1: {d}\t(Time: ", .{result});
        try printNanos(stdout, time);
        try stdout.print(")\n", .{});
    }
    {
        // var timer = try std.time.Timer.start();
        // const result = try part2.run(allocator, input);
        // const time = timer.lap();
        // try stdout.print("Day 5 Part 2: {d}\t(Time: ", .{result});
        // try printNanos(stdout, time);
        // try stdout.print(")\n", .{});
        try stdout.print("Day 5 Part 2: 23738616\t(Skip. Time: >1m)\n", .{}); // Skip
        // look at this: https://zigbin.io/0688c6
    }
}

test {
    _ = @import("day5/part1.zig");
    _ = @import("day5/part2.zig");
}
