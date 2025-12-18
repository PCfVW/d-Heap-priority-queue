//! Build configuration for d-ary heap priority queue.
//!
//! Available build steps:
//! - `zig build`      : Build the demo executable
//! - `zig build run`  : Build and run the demo
//! - `zig build test` : Run all unit tests
//!
//! For library users, import the "d-heap" module in your build.zig.

const std = @import("std");

pub fn build(b: *std.Build) void {
    // Standard build options
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // =========================================================================
    // Library Module (for external projects to use as dependency)
    // =========================================================================
    // d_heap module already re-exports Item from types.zig
    const d_heap_module = b.addModule("d-heap", .{
        .root_source_file = b.path("src/d_heap.zig"),
        .target = target,
    });

    // =========================================================================
    // Demo Executable
    // =========================================================================
    const exe = b.addExecutable(.{
        .name = "demo",
        .root_module = b.createModule(.{
            .root_source_file = b.path("src/main.zig"),
            .target = target,
            .optimize = optimize,
        }),
    });
    b.installArtifact(exe);

    // Run step: `zig build run`
    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep());
    if (b.args) |args| {
        run_cmd.addArgs(args);
    }
    const run_step = b.step("run", "Run the demo program");
    run_step.dependOn(&run_cmd.step);

    // =========================================================================
    // Unit Tests
    // =========================================================================
    const test_module = b.createModule(.{
        .root_source_file = b.path("src/tests/heap_tests.zig"),
        .target = target,
        .optimize = optimize,
        .imports = &.{
            .{ .name = "d_heap", .module = d_heap_module },
        },
    });

    const unit_tests = b.addTest(.{
        .root_module = test_module,
    });

    const run_unit_tests = b.addRunArtifact(unit_tests);
    const test_step = b.step("test", "Run unit tests");
    test_step.dependOn(&run_unit_tests.step);
}
