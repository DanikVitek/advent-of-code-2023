const std = @import("std");
const mem = std.mem;
const math = std.math;
const Allocator = mem.Allocator;

pub fn run(allocator: Allocator, input: []const u8) !u32 {
    var cards_iter = mem.tokenizeAny(u8, input, "\r\n");
    var points: u32 = 0;

    while (cards_iter.next()) |card| {
        points += try scorePoints(try processCard(allocator, card));
    }

    return points;
}

fn processCard(allocator: Allocator, card: []const u8) !u32 {
    const StringHashSet = std.StringHashMap(void);

    const numbers = blk: {
        var split_card_id = mem.splitSequence(u8, card, ": ");
        _ = split_card_id.first();
        break :blk split_card_id.rest();
    };
    var number_groups = mem.splitSequence(u8, numbers, " | ");

    const winning_numbers_str = number_groups.first();
    const our_numbers_str = number_groups.rest();

    var winning_numbers_iter = mem.tokenizeScalar(u8, winning_numbers_str, ' ');
    var our_numbers_iter = mem.tokenizeScalar(u8, our_numbers_str, ' ');

    var winning_numbers_set = StringHashSet.init(allocator);
    defer winning_numbers_set.deinit();
    while (winning_numbers_iter.next()) |winning_number| {
        try winning_numbers_set.put(winning_number, {});
    }

    var wins: u32 = 0;
    while (our_numbers_iter.next()) |our_number| {
        if (winning_numbers_set.contains(our_number)) {
            wins += 1;
        }
    }

    return wins;
}

fn scorePoints(wins: u32) error{Overflow}!u32 {
    return math.powi(u32, 2, math.sub(u32, wins, 1) catch return 0) catch |err| switch (err) {
        error.Overflow => return error.Overflow,
        else => unreachable,
    };
}

test scorePoints {
    const testing = std.testing;

    try testing.expectEqual(@as(u32, 8), try scorePoints(4));
    try testing.expectEqual(@as(u32, 4), try scorePoints(3));
    try testing.expectEqual(@as(u32, 2), try scorePoints(2));
    try testing.expectEqual(@as(u32, 1), try scorePoints(1));
    try testing.expectEqual(@as(u32, 0), try scorePoints(0));
}

test processCard {
    const testing = std.testing;
    const allocator = testing.allocator;

    try testing.expectEqual(@as(u32, 4), try processCard(allocator, "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"));
    try testing.expectEqual(@as(u32, 2), try processCard(allocator, "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"));
    try testing.expectEqual(@as(u32, 2), try processCard(allocator, "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"));
    try testing.expectEqual(@as(u32, 1), try processCard(allocator, "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"));
    try testing.expectEqual(@as(u32, 0), try processCard(allocator, "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"));
    try testing.expectEqual(@as(u32, 0), try processCard(allocator, "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"));
}

test "simple input" {
    const testing = std.testing;
    const input =
        \\Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        \\Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        \\Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        \\Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        \\Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        \\Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    ;
    try testing.expectEqual(@as(u32, 13), try run(testing.allocator, input));
}
