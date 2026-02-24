const std = @import("std");
const assert = std.debug.assert;
const mem = std.mem;
const math = std.math;

const MAX_INPUT_SIZE = 1 << 24;

pub fn solve() !void {
    
}

const builtin = @import("builtin");

var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
const allocator = arena.allocator();
const delimiters = " \n";

fn is_delimiter(char: u8) bool {
    inline for (delimiters) |delimiter| {
        if (char == delimiter) {
            return true;
        }
    }
    return false;
}

const DebugScanner = struct {
    var stdin_buf: [MAX_INPUT_SIZE]u8 = undefined;
    var stdin_reader = std.fs.File.stdin().reader(&stdin_buf);
    const stdin = &stdin_reader.interface;
    var line: []u8 = undefined;
    var pos: usize = 0;

    fn init() !void {
        if (!try getLine()) {
            line = &.{};
        }
    }

    fn getLine() !bool {
        line = stdin.takeDelimiterExclusive('\n') catch |err| switch (err) {
            error.EndOfStream => return false,
            else => return err,
        };
        pos = 0;
        return true;
    }

    fn next() !?[]u8 {
        while (true) {
            while (pos < line.len and is_delimiter(line[pos])) : (pos += 1) {}
            if (pos >= line.len) {
                if (!try getLine()) {
                    return null;
                }
                continue;
            }
            var end = pos;
            while (end < line.len and !is_delimiter(line[end])) : (end += 1) {}
            const ret = try allocator.dupe(u8, line[pos..end]);
            pos = end;
            return ret;
        }
    }
};

const OptimizedScanner = struct {
    var input_buf: [MAX_INPUT_SIZE]u8 = undefined;
    var input: []u8 = undefined;
    var pos: usize = 0;
    var stdin_buf: [1024]u8 = undefined;
    var stdin_reader = std.fs.File.stdin().reader(&stdin_buf);
    const stdin = &stdin_reader.interface;

    fn init() !void {
        const size: usize = @intCast(try stdin_reader.getSize());
        if (MAX_INPUT_SIZE < size) return anyerror.FileSizeExceeded;
        try stdin.readSliceAll(input_buf[0..size]);
        input = input_buf[0..size];
    }

    fn next() !?[]u8 {
        while (pos < input.len and is_delimiter(input[pos])) : (pos += 1) {}
        if (pos >= input.len) {
            return null;
        }
        var end = pos;
        while (end < input.len and !is_delimiter(input[end])) : (end += 1) {}
        const ret = input[pos..end];
        pos = end;
        return ret;
    }
};

const Scanner = switch (builtin.mode) {
    .Debug, .ReleaseSafe => DebugScanner,
    else => OptimizedScanner,
};

var stdout_buf: [1 << 20]u8 = undefined;
var stdout_writer = std.fs.File.stdout().writer(&stdout_buf);
const stdout = &stdout_writer.interface;

fn ErrorUnionPayload(comptime T: type) type {
    return @typeInfo(T).error_union.payload;
}

fn ignoreError(value: anytype) ErrorUnionPayload(@TypeOf(value)) {
    return value catch |err| {
        std.log.err("{any}", .{err});
        @panic("");
    };
}

fn tryReadInt(comptime T: type) !T {
    const token = try Scanner.next() orelse return error.UnexpectedEof;
    return std.fmt.parseInt(T, token, 10);
}

fn tryReadIntOptional(comptime T: type) !?T {
    const token = try Scanner.next() orelse return null;
    return try std.fmt.parseInt(T, token, 10);
}

fn readInt(comptime T: type) T {
    return ignoreError(tryReadInt(T));
}

fn readIntOptional(comptime T: type) ?T {
    return ignoreError(tryReadIntOptional(T));
}

fn tryReadString() ![]u8 {
    const token = try Scanner.next() orelse return error.UnexpectedEof;
    return token;
}

fn tryReadStringOptional() !?[]u8 {
    return try Scanner.next();
}

fn readString() []u8 {
    return ignoreError(tryReadString());
}

fn readStringOptional() ?[]u8 {
    return ignoreError(tryReadStringOptional());
}

fn readChar() u8 {
    const token = readString();
    std.debug.assert(token.len == 1);
    return token[0];
}

fn print(comptime fmt: []const u8, args: anytype) void {
    ignoreError(stdout.print(fmt, args));
}

pub fn main() !void {
    try Scanner.init();
    try solve();
    try stdout.flush();
}

fn FixedQueue(comptime T: type, comptime max_size: u32) type {
    return struct {
        const Self = @This();
        buf: [max_size]T = undefined,
        rp: u32 = 0,
        wp: u32 = 0,

        pub fn push(self: *Self, item: T) void {
            self.buf[self.wp] = item;
            self.wp += 1;
        }
        pub fn pop(self: *Self) ?T {
            if (self.wp == self.rp) {
                return null;
            }
            const rp = self.rp;
            self.rp = rp + 1;
            return self.buf[rp];
        }
    };
}

const Unionfind = struct {
    const Self = @This();
    size: []u32,
    parent: []u32,

    pub fn init(alloc: mem.Allocator, num: u32) !Self {
        const buf = try alloc.alloc(u32, num);
        const buf2 = try alloc.alloc(u32, num);
        @memset(buf, 1);
        @memset(buf2, std.math.maxInt(u32));
        return .{
            .size = buf,
            .parent = buf2,
        };
    }

    pub fn find(self: Self, u: u32) u32 {
        var prev: u32 = math.maxInt(u32);
        var cur = u;
        while (self.parent[cur] != std.math.maxInt(u32)) {
            self.size[cur] = prev; // rootじゃないので変更してよい
            prev = cur;
            cur = self.parent[cur];
        }
        // 経路圧縮
        while (prev != std.math.maxInt(u32)) : (prev = self.size[prev]) {
            self.parent[prev] = cur;
        }
        return cur;
    }

    pub fn unite(self: Self, u: u32, v: u32) bool {
        const ru = self.find(u);
        const rv = self.find(v);
        if (ru == rv) {
            return false;
        } else {
            if (self.size[ru] > self.size[rv]) {
                self.parent[rv] = ru;
                self.size[ru] += self.size[rv];
            } else {
                self.parent[ru] = rv;
                self.size[rv] += self.size[ru];
            }
            return true;
        }
    }

    pub fn deinit(self: Self, alloc: mem.Allocator) void {
        alloc.free(self.parent);
        alloc.free(self.size);
    }

    pub fn clear(self: Self) void {
        @memset(self.size, 1);
        @memset(self.parent, std.math.maxInt(usize));
    }
};

fn requiringBits(val: comptime_int) comptime_int {
    var cur = val;
    var count = 0;
    while (cur > 0) : (cur >>= 1) count += 1;
    return count;
}

pub fn ModInt(modulo: comptime_int) type {
    if (modulo <= 0) @compileError("Modulo must be positive");
    if (modulo > 100_000_000_000) @compileError("Cannot check primary of modulo");
    @setEvalBranchQuota(1_000_000);
    comptime var is_prime = false;
    if (modulo >= 2) {
        var i = 2;
        is_prime = true;
        while (i * i <= modulo) : (i += 1) {
            if (modulo % i == 0) {
                is_prime = false;
                break;
            }
        }
    }
    return ModIntEx(modulo, is_prime);
}

pub fn PrimeModInt(modulo: comptime_int) type {
    return ModIntEx(modulo, true);
}

pub fn ModIntEx(modulo: comptime_int, comptime modulo_is_prime: bool) type {
    if (modulo <= 0) @compileError("Modulo must be positive");
    return struct {
        const Self = @This();
        const Int = std.meta.Int(.unsigned, requiringBits(modulo - 1));
        const Extended = std.meta.Int(.unsigned, requiringBits(modulo - 1) + 1);
        value: Int,
        pub const zero: Self = .{ .value = 0 };
        pub const one: Self = .{ .value = 1 };

        pub fn add(self: Self, other: Self) Self {
            const a: Extended = self.value;
            const b: Extended = other.value;
            return .{
                .value = @intCast((a + b) % modulo),
            };
        }

        pub fn sub(self: Self, other: Self) Self {
            const a: Extended = self.value;
            const b: Extended = other.value;
            return .{
                .value = @intCast((a + modulo - b) % modulo),
            };
        }

        pub fn mul(self: Self, other: Self) Self {
            const prod = std.math.mulWide(Int, self.value, other.value);
            return .{
                .value = @intCast(prod % modulo),
            };
        }

        pub fn pow(self: Self, exp: anytype) Self {
            var base = self;
            var result = Self.one;
            switch (@typeInfo(@TypeOf(exp))) {
                .int => |int| {
                    if (int.signedness == .signed) {
                        @compileError("Signed int is not allowed");
                    }
                    var exp_cur = exp;
                    while (exp_cur > 0) : (exp_cur >>= 1) {
                        if (exp_cur % 2 != 0) result = result.mul(base);
                        base = base.mul(base);
                    }
                },
                .comptime_int => {
                    comptime var exp_cur = exp;
                    inline while (exp_cur > 0) : (exp_cur >>= 1) {
                        if (exp_cur % 2 != 0) result = result.mul(base);
                        base = base.mul(base);
                    }
                },
                else => {
                    @compileError("Only int or comptime_int allowed for exponent");
                },
            }
            return result;
        }

        pub fn powSignedExp(self: Self, exp: anytype) Self {
            switch (@typeInfo(@TypeOf(exp))) {
                .int, .comptime_int => {},
                else => @compileError("Only int or comptime_int allowed for exponent"),
            }
            const base = if (exp < 0) self.inv() else self;
            return base.pow(@abs(exp));
        }

        pub fn inv(self: Self) Self {
            if (!modulo_is_prime) @compileError("Non-prime modulo integer is not support inverse operation");
            return self.pow(modulo - 2);
        }

        pub fn fromRaw(value: Int) Self {
            return .{ .value = value };
        }

        pub fn init(value: anytype) Self {
            const info = @typeInfo(@TypeOf(value));
            switch (info) {
                .int, .comptime_int => {},
                else => {
                    @compileError("Only int or comptime_int allowed to initialize ModInt");
                },
            }
            return .{ .value = @intCast(@mod(value, modulo)) };
        }

        pub fn format(self: Self, w: *std.Io.Writer) std.Io.Writer.Error!void {
            return w.print("{d}", .{self.value});
        }

        pub const Combination = struct {
            factorial: []Self,
            factorial_inv: []Self,

            pub fn init(n: usize, gpa: mem.Allocator) mem.Allocator.Error!Combination {
                const fact = try gpa.alloc(Self, n + 1);
                errdefer gpa.free(fact);
                const fact_i = try gpa.alloc(Self, n + 1);
                fact[0] = .one;
                for (1..n + 1) |i| {
                    fact[i] = Self.init(i).mul(fact[i - 1]);
                }
                fact_i[n] = fact[n].inv();
                var i = n;
                while (i > 0) : (i -= 1) {
                    fact_i[i - 1] = Self.init(i).mul(fact_i[i]);
                }
                return .{
                    .factorial = fact,
                    .factorial_inv = fact_i,
                };
            }

            pub fn deinit(self: Combination, gpa: mem.Allocator) void {
                gpa.free(self.factorial);
                gpa.free(self.factorial_inv);
            }

            /// `n`個の中から`k`個選ぶときの組み合わせ数を返す
            pub fn combi(self: Combination, n: usize, k: usize) Self {
                return if (k > n) .zero else self.factorial[n].mul(self.factorial_inv[k]).mul(self.factorial_inv[n - k]);
            }
        };
    };
}