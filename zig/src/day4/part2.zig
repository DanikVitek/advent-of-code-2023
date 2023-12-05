const std = @import("std");
const mem = std.mem;
const fmt = std.fmt;
const Allocator = mem.Allocator;
const ArrayList = std.ArrayList;

pub fn run(allocator: Allocator, input: []const u8) !usize {
    var cards_iter = std.mem.tokenizeAny(u8, input, "\r\n");

    var cards = ArrayList(Card).init(allocator);
    defer cards.deinit();

    while (cards_iter.next()) |card| {
        try cards.append(try parseCard(allocator, card));
    }

    return processCards(cards.items, cards.items);
}

const Card = struct {
    id: u32,
    wins: u32,
};

fn processCards(orig_cards: []const Card, queued_cards: []const Card) usize {
    var total: usize = queued_cards.len;
    for (queued_cards) |card| {
        total += processCards(orig_cards, orig_cards[card.id .. card.id + card.wins]);
    }
    return total;
}

fn parseCard(allocator: Allocator, card: []const u8) !Card {
    const StringHashSet = std.StringHashMap(void);

    var split_card_id = mem.splitSequence(u8, card, ": ");
    const id = blk: {
        var card_id = mem.splitBackwardsScalar(u8, split_card_id.first(), ' ');
        break :blk try std.fmt.parseInt(u32, card_id.first(), 10);
    };
    const numbers = split_card_id.rest();
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

    return .{ .id = id, .wins = wins };
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
    try testing.expectEqual(@as(usize, 30), try run(testing.allocator, input));
}
