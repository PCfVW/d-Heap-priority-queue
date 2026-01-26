//! Build configuration for Dijkstra example.
//!
//! Available build steps:
//! - `zig build`      : Build the dijkstra-example executable
//! - `zig build run`  : Build and run the example

const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Reference the d-heap module from the parent Zig directory
    const d_heap_module = b.addModule("d-heap", .{
        .root_source_file = b.path("../../../zig/src/d_heap.zig"),
        .target = target,
    });

    // Dijkstra example executable
    const exe = b.addExecutable(.{
        .name = "dijkstra-example",
        .root_module = b.createModule(.{
            .root_source_file = b.path("src/main.zig"),
            .target = target,
            .optimize = optimize,
            .imports = &.{
                .{ .name = "d-heap", .module = d_heap_module },
            },
        }),
    });
    b.installArtifact(exe);

    // Run step: `zig build run`
    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep());
    if (b.args) |args| {
        run_cmd.addArgs(args);
    }
    const run_step = b.step("run", "Run the Dijkstra example");
    run_step.dependOn(&run_cmd.step);
}
