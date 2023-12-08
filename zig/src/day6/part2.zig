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

// T = const
// V0 = 0
// a = 1
// V = V0 + t * a
// S* < S = (T - t) * V = (T - t) * (0 + t * 1) = T * t - t^2
// S = -t^2 + T*t > S*
// -t^2 + T*t - S* > 0
// t^2 - T*t + S* < 0

// x1 = (b + sqrt(b^2 - 4ac)) / 2a
// x2 = (b - sqrt(b^2 - 4ac)) / 2a
// t1 = (T - sqrt(T^2 - 4 * S*)) / 2 = (T - sqrt(T^2 - 4 * S*)) / 2
// t2 = (T + sqrt(T^2 - 4 * 1 * S*)) / 2 = (T + sqrt(T^2 - 4 * S*)) / 2
// S in (t1, t2)

pub fn run(allocator: Allocator, input: []const u8) !u64 {
    _ = allocator;
    var lines = switch (builtin.os.tag) {
        .windows => mem.splitSequence(u8, input, NL),
        else => mem.splitSequence(u8, input, NL),
    };
    const times = mem.tokenizeScalar(u8, lines.first()[11..], ' ');
    const distances = mem.tokenizeScalar(u8, lines.rest()[11..], ' ');
    var distances_per_times = zip(distances, times);
    var actual_distance_and_time = @Vector(2, f64){ 0, 0 };
    while (distances_per_times.next()) |distance_pet_time| {
        const distance_and_time = @Vector(2, f64){
            @floatFromInt(try fmt.parseInt(u32, distance_pet_time.first, 10)),
            @floatFromInt(try fmt.parseInt(u32, distance_pet_time.second, 10)),
        };
        const power = @Vector(2, f64){ @floatFromInt(distance_pet_time.first.len), @floatFromInt(distance_pet_time.second.len) };
        actual_distance_and_time = actual_distance_and_time * @Vector(2, f64){ math.pow(f64, 10, power[0]), math.pow(f64, 10, power[1]) } + distance_and_time;
    }
    const actual_time_category = actual_distance_and_time[1];
    const actual_best_distance = actual_distance_and_time[0];
    const discriminant_sqrt: f64 = @sqrt(actual_time_category * actual_time_category - 4 * actual_best_distance);
    const min_time = (actual_time_category - discriminant_sqrt) / 2;
    const max_time = (actual_time_category + discriminant_sqrt) / 2;
    const min_time_int: u64 = switch (min_time - @floor(min_time) > 0) {
        true => @intFromFloat(@ceil(min_time)),
        false => @intFromFloat(min_time + 1),
    };
    const max_time_int: u64 = switch (max_time - @floor(max_time) > 0) {
        true => @intFromFloat(max_time),
        false => @intFromFloat(max_time - 1),
    };

    return max_time_int - min_time_int + 1;
}

fn zip(iter1: anytype, iter2: anytype) struct {
    iter1: @TypeOf(iter1),
    iter2: @TypeOf(iter2),

    const T = @typeInfo(@typeInfo(@TypeOf(@TypeOf(iter1).next)).Fn.return_type.?).Optional.child;
    const U = @typeInfo(@typeInfo(@TypeOf(@TypeOf(iter2).next)).Fn.return_type.?).Optional.child;

    const Pair = struct {
        first: T,
        second: U,
    };

    fn next(self: *@This()) ?Pair {
        var first: T = self.iter1.next() orelse return null;
        var second: U = self.iter2.next() orelse return null;
        return .{ .first = first, .second = second };
    }
} {
    return .{ .iter1 = iter1, .iter2 = iter2 };
}

test "simple input" {
    const testing = std.testing;
    const input = @embedFile("test_input.txt");
    try testing.expectEqual(@as(u64, 71503), try run(testing.allocator, input));
}
