const std = @import("std");
const builtin = @import("builtin");
const mem = std.mem;
const math = std.math;
const Allocator = mem.Allocator;
const ArrayList = std.ArrayList;
const SplitIterator = std.mem.SplitIterator;

const NL = switch (builtin.os.tag) {
    .windows => "\r\n",
    else => '\n',
};

pub fn run(allocator: Allocator, input: []const u8) !u32 {
    var blocks = std.mem.splitSequence(u8, input, NL ** 2);

    var seeds = Seeds{ .iter = std.mem.splitScalar(u8, blocks.first()[7..], ' ') };

    var seed_to_soil = try Map.parse(allocator, blocks.next().?);
    defer seed_to_soil.deinit();
    var soil_to_fertilizer = try Map.parse(allocator, blocks.next().?);
    defer soil_to_fertilizer.deinit();
    var fertilizer_to_water = try Map.parse(allocator, blocks.next().?);
    defer fertilizer_to_water.deinit();
    var water_to_light = try Map.parse(allocator, blocks.next().?);
    defer water_to_light.deinit();
    var light_to_temperature = try Map.parse(allocator, blocks.next().?);
    defer light_to_temperature.deinit();
    var temperature_to_humidity = try Map.parse(allocator, blocks.next().?);
    defer temperature_to_humidity.deinit();
    var humidity_to_location = try Map.parse(allocator, blocks.next().?);
    defer humidity_to_location.deinit();

    var closest_location: u32 = std.math.maxInt(u32);
    while (try seeds.next()) |seed| {
        closest_location = @min(closest_location, seedToLocation(
            seed,
            &seed_to_soil,
            &soil_to_fertilizer,
            &fertilizer_to_water,
            &water_to_light,
            &light_to_temperature,
            &temperature_to_humidity,
            &humidity_to_location,
        ));
    }

    return closest_location;
}

const Seeds = struct {
    iter: SplitIterator(u8, .scalar),
    last_range: ?struct {
        start: u32,
        len: u32,
        index: u32 = 0,
    } = null,

    fn next(self: *Seeds) !?u32 {
        if (self.last_range) |*range| {
            range.index += 1;
            if (range.index < range.len) {
                return range.start + range.index;
            }
        }
        const start = try std.fmt.parseInt(u32, self.iter.next() orelse return null, 10);
        const len = try std.fmt.parseInt(u32, self.iter.next().?, 10);
        self.last_range = .{ .start = start, .len = len };
        return start;
    }
};

const RangeMapIterator = struct {
    lines: SplitIterator(u8, switch (builtin.os.tag) {
        .windows => .sequence,
        else => .scalar,
    }),

    pub inline fn init(block: []const u8) RangeMapIterator {
        return .{ .lines = linesPastFirst(block) };
    }

    fn linesPastFirst(block: []const u8) SplitIterator(u8, switch (builtin.os.tag) {
        .windows => .sequence,
        else => .scalar,
    }) {
        var iter = switch (builtin.os.tag) {
            .windows => std.mem.splitSequence(u8, block, NL),
            else => std.mem.splitScalar(u8, block, NL),
        };
        _ = iter.first();
        return iter;
    }

    pub fn next(self: *RangeMapIterator) !?RangeMap {
        if (self.lines.next()) |line| {
            return try RangeMap.parse(line);
        } else {
            return null;
        }
    }

    pub fn collect(self: RangeMapIterator, allocator: Allocator) !Map {
        var self_mut = self;
        var result = ArrayList(RangeMap).init(allocator);
        errdefer result.deinit();
        while (try self_mut.next()) |range| {
            try result.append(range);
        }
        var result_owned = try result.toOwnedSlice();

        const Comparator = struct {
            fn lessThan(_: @This(), lhs: RangeMap, rhs: RangeMap) bool {
                return lhs.source_start < rhs.source_start;
            }
        };

        std.mem.sortUnstable(RangeMap, result_owned, Comparator{}, Comparator.lessThan);
        return .{
            .ranges = result_owned,
            .allocator = allocator,
        };
    }
};

const RangeMap = struct {
    destination_start: u32,
    source_start: u32,
    len: u32,

    fn apply(self: RangeMap, source: u32) ?u32 {
        if (self.source_start <= source and source - self.source_start < self.len) {
            return self.destination_start + (source - self.source_start);
        }
        return null;
    }

    fn parse(row: []const u8) !RangeMap {
        var iter = std.mem.splitScalar(u8, row, ' ');
        const destination_start = try std.fmt.parseInt(u32, iter.first(), 10);
        const source_start = try std.fmt.parseInt(u32, iter.next().?, 10);
        const len = try std.fmt.parseInt(u32, iter.rest(), 10);
        return .{
            .destination_start = destination_start,
            .source_start = source_start,
            .len = len,
        };
    }
};

const Map = struct {
    ranges: []const RangeMap,
    allocator: Allocator,

    fn parse(allocator: Allocator, block: []const u8) !Map {
        return RangeMapIterator.init(block).collect(allocator);
    }

    fn deinit(self: Map) void {
        self.allocator.free(self.ranges);
    }

    fn apply(self: *const Map, source: u32) u32 {
        const Searcher = struct {
            fn compare(_: @This(), key: u32, mid_item: RangeMap) math.Order {
                if (mid_item.source_start > key) {
                    return .lt;
                } else if (key - mid_item.source_start >= mid_item.len) {
                    return .gt;
                } else {
                    return .eq;
                }
            }
        };

        const index = std.sort.binarySearch(RangeMap, source, self.ranges, Searcher{}, Searcher.compare);
        if (index) |i| {
            return self.ranges[i].apply(source) orelse unreachable;
        } else {
            return source;
        }
    }
};

fn seedToLocation(
    seed: u32,
    seed_to_soil: *const Map,
    soil_to_fertilizer: *const Map,
    fertilizer_to_water: *const Map,
    water_to_light: *const Map,
    light_to_temperature: *const Map,
    temperature_to_humidity: *const Map,
    humidity_to_location: *const Map,
) u32 {
    const soil = seed_to_soil.apply(seed);
    const fertilizer = soil_to_fertilizer.apply(soil);
    const water = fertilizer_to_water.apply(fertilizer);
    const light = water_to_light.apply(water);
    const temperature = light_to_temperature.apply(light);
    const humidity = temperature_to_humidity.apply(temperature);
    return humidity_to_location.apply(humidity);
}
