const std = @import("std");
const builtin = @import("builtin");
const mem = std.mem;
const math = std.math;
const fmt = std.fmt;
const Allocator = mem.Allocator;
const ArrayList = std.ArrayList;
const SplitIterator = mem.SplitIterator;
const HashMap = std.HashMap;

const NL = switch (builtin.os.tag) {
    .windows => "\r\n",
    else => '\n',
};

pub fn run(allocator: Allocator, input: []const u8) !u64 {
    const lines = switch (builtin.os.tag) {
        .windows => mem.splitSequence(u8, input, NL),
        else => mem.splitScalar(u8, input, NL),
    };
    var entries = try (struct {
        lines: @TypeOf(lines),

        fn next(self: *@This()) !?Entry {
            var line: []const u8 = self.lines.next() orelse return null;
            const hand_and_bid = splitOnce(u8, .scalar, ' ', line) orelse return error.InvalidInput;
            const hand = try Hand.parse(hand_and_bid[0]);
            const bid = try fmt.parseInt(math.IntFittingRange(0, 999), hand_and_bid[1], 10);
            return .{ .hand = hand, .bid = bid };
        }

        fn collect(self: @This(), alloc: Allocator) ![]Entry {
            var self_mut = self;
            var entries = ArrayList(Entry).init(alloc);
            errdefer entries.deinit();
            while (try self_mut.next()) |entry| {
                try entries.append(entry);
            }
            return entries.toOwnedSlice();
        }
    }{ .lines = lines }).collect(allocator);
    defer allocator.free(entries);

    const Comparator = struct {
        allocator: Allocator,
        chached_types: HashMap(
            Hand,
            HandType,
            HandContext,
            std.hash_map.default_max_load_percentage,
        ),

        fn init(alloc: Allocator) @This() {
            return .{
                .allocator = alloc,
                .chached_types = HashMap(
                    Hand,
                    HandType,
                    HandContext,
                    std.hash_map.default_max_load_percentage,
                ).init(alloc),
            };
        }

        fn deinit(self: *@This()) void {
            self.chached_types.deinit();
        }

        const HandContext = struct {
            pub fn hash(self: @This(), s: Hand) u64 {
                _ = self;
                return hashHand(s);
            }
            pub fn eql(self: @This(), a: Hand, b: Hand) bool {
                _ = self;
                return eqlHand(a, b);
            }

            pub fn eqlHand(a: Hand, b: Hand) bool {
                return mem.eql(CardLabel, &a.cards, &b.cards);
            }

            pub fn hashHand(s: Hand) u64 {
                return std.hash.Wyhash.hash(0, mem.sliceAsBytes(&s.cards));
            }
        };

        fn lessThan(self: *@This(), lhs: Entry, rhs: Entry) bool {
            const lhs_type: HandType = blk: {
                var result = self.chached_types.getOrPut(lhs.hand) catch |err| std.debug.panic("{!}\n", .{err});
                if (!result.found_existing) {
                    result.value_ptr.* = lhs.hand.handType();
                    // std.debug.print("{s}: {any}\n", .{ &lhs.hand.asString(), result.value_ptr.* });
                }
                break :blk result.value_ptr.*;
            };
            const rhs_type: HandType = blk: {
                var result = self.chached_types.getOrPut(rhs.hand) catch |err| std.debug.panic("{!}\n", .{err});
                if (!result.found_existing) {
                    result.value_ptr.* = rhs.hand.handType();
                }
                break :blk result.value_ptr.*;
            };
            if (lhs_type != rhs_type) {
                return lhs_type.compare(rhs_type) == .lt;
            }

            for (lhs.hand.cards, rhs.hand.cards) |lhs_card, rhs_card| {
                if (lhs_card != rhs_card) {
                    return lhs_card.compare(rhs_card) == .lt;
                }
            }

            return false;
        }
    };

    var comparator = Comparator.init(allocator);
    mem.sortUnstable(Entry, entries, &comparator, Comparator.lessThan);
    comparator.deinit();

    var total_winnings: u64 = 0;
    for (entries, 1..) |entry, rank| {
        total_winnings += @as(u64, rank) * @as(u64, entry.bid);
    }

    return total_winnings;
}

fn splitOnce(
    comptime T: type,
    comptime delimiter_type: mem.DelimiterType,
    delimiter: switch (delimiter_type) {
        .scalar => T,
        .sequence, .any => []const T,
    },
    buffer: []const u8,
) ?struct { []const T, []const T } {
    var iter = SplitIterator(T, delimiter_type){
        .index = 0,
        .buffer = buffer,
        .delimiter = delimiter,
    };
    return .{ iter.first(), iter.next() orelse return null };
}

const Entry = struct {
    hand: Hand,
    bid: math.IntFittingRange(0, 999),
};

const CardLabel = enum(math.IntFittingRange(1, 13)) {
    joker = 1,
    two = 2,
    three = 3,
    four = 4,
    give = 5,
    six = 6,
    seven = 7,
    eight = 8,
    nine = 9,
    ten = 10,
    queen = 11,
    king = 12,
    ace = 13,

    fn fromChar(c: u8) !CardLabel {
        return switch (c) {
            'J' => .joker,
            '2' => .two,
            '3' => .three,
            '4' => .four,
            '5' => .give,
            '6' => .six,
            '7' => .seven,
            '8' => .eight,
            '9' => .nine,
            'T' => .ten,
            'Q' => .queen,
            'K' => .king,
            'A' => .ace,
            else => error.InvalidChar,
        };
    }

    fn compare(self: CardLabel, other: CardLabel) math.Order {
        return math.order(@intFromEnum(self), @intFromEnum(other));
    }
};

const HandType = enum(math.IntFittingRange(0, 6)) {
    high_card,
    one_pair,
    two_pair,
    three_of_a_kind,
    full_house,
    four_of_a_kind,
    five_of_a_kind,

    fn compare(self: HandType, other: HandType) math.Order {
        return math.order(@intFromEnum(self), @intFromEnum(other));
    }
};

const Hand = struct {
    cards: [5]CardLabel,

    fn parse(line: []const u8) !Hand {
        var cards: [5]CardLabel = undefined;
        for (&cards, line) |*card, char| {
            card.* = try CardLabel.fromChar(char);
        }
        return .{ .cards = cards };
    }

    fn handType(self: Hand) HandType {
        var counts = [_]math.IntFittingRange(0, 5){0} ** 13;
        for (self.cards) |card| {
            counts[@intFromEnum(card) - 1] += 1;
        }
        const jokers = counts[0];

        var pair_count: u2 = 0;
        var three_count: u1 = 0;
        var four_count: u1 = 0;
        var five_count: u1 = 0;
        for (counts) |count| {
            switch (count) {
                0, 1 => continue,
                2 => pair_count += 1,
                3 => three_count += 1,
                4 => four_count += 1,
                5 => five_count += 1,
                else => unreachable,
            }
        }

        // joker would make the hand to the strongest type possible
        if (five_count == 1 or (four_count == 1 and jokers == 1) or (three_count == 1 and jokers == 2) or (pair_count == 1 and jokers == 3) or (jokers == 4)) {
            // AAAAA
            // AAAAJ
            // AAAJJ
            // AAJJJ
            // AJJJJ
            return .five_of_a_kind;
        } else if (four_count == 1 or (three_count == 1 and (jokers == 1 or jokers == 3)) or (pair_count == 2 and jokers == 2)) {
            // AAAAB
            // AAAJB
            // AAJJB
            // AJJJB
            return .four_of_a_kind;
        } else if ((three_count == 1 and (pair_count == 1 or jokers == 1)) or (pair_count == 2 and jokers == 1)) {
            // AAABB
            // AAABJ
            // AABBJ
            return .full_house;
        } else if (three_count == 1 or (pair_count == 1 and jokers == 1) or (jokers == 2)) {
            // AAABC
            // AAJBC
            // AJJBC
            return .three_of_a_kind;
        } else if (pair_count == 2 or (pair_count == 1 and jokers == 1)) {
            // AABBC
            // AABJC
            return .two_pair;
        } else if (pair_count == 1 or (jokers == 1)) {
            // AABCD
            // AJBCD
            return .one_pair;
        } else {
            return .high_card;
        }
    }

    fn asString(self: Hand) [5]u8 {
        var result: [5]u8 = undefined;
        for (self.cards, &result) |card, *result_char| {
            result_char.* = switch (card) {
                .joker => 'J',
                .two => '2',
                .three => '3',
                .four => '4',
                .give => '5',
                .six => '6',
                .seven => '7',
                .eight => '8',
                .nine => '9',
                .ten => 'T',
                .queen => 'Q',
                .king => 'K',
                .ace => 'A',
            };
        }
        return result;
    }
};

test "simple input" {
    const testing = std.testing;
    const input = @embedFile("test_input.txt");
    try testing.expectEqual(@as(u64, 5905), try run(testing.allocator, input));
}
