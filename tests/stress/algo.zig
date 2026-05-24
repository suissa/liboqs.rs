const std = @import("std");

const Metric = struct {
    t_s: u32,
    ops_target: u64,
};

pub fn main() !void {
    // stress profile: 30s total, update every 2s, doubling throughput target,
    // starting at 1_000_000 ops.
    const total_s: u32 = 30;
    const step_s: u32 = 2;
    const initial_ops: u64 = 1_000_000;

    var metrics: [16]Metric = undefined;
    var idx: usize = 0;
    var t_s: u32 = 0;
    var ops_target: u64 = initial_ops;

    while (t_s <= total_s) : (t_s += step_s) {
        metrics[idx] = .{ .t_s = t_s, .ops_target = ops_target };
        idx += 1;
        if (t_s < total_s) ops_target *%= 2;
    }

    const stdout = std.io.getStdOut().writer();
    try stdout.print("make stress tests/stress/algo.zig 30s 2s/2x 1000000ops\\n", .{});
    try stdout.print("{{\n  \"profile\": {{\n", .{});
    try stdout.print("    \"duration_s\": {d},\n    \"step_s\": {d},\n    \"scale\": \"2x\",\n    \"initial_ops\": {d}\n  }},\n  \"metrics\": [\n", .{ total_s, step_s, initial_ops });

    for (metrics, 0..) |m, i| {
        const comma = if (i + 1 < metrics.len) "," else "";
        try stdout.print("    {{ \"t_s\": {d}, \"ops_target\": {d} }}{s}\n", .{ m.t_s, m.ops_target, comma });
    }

    try stdout.print("  ]\n}}\n", .{});
}
