const global = struct {

const std = @import("std");
const assert = std.debug.assert;
const mem = std.mem;
const math = std.math;

const MAX_INPUT_SIZE = 1 << 24;
const safety = false;
const skip_delim = false;
const force_optimized = false;


pub fn solve() !void {
    const n = readInt(u32);
    const k = readInt(u32);
    const m = readInt(u32);
    var max: [2<<17]u32 = undefined;
    @memset(max[0..n], 0);
    var buf: [2<<17]u32 = undefined;
    var other: std.ArrayList(u32) = .initBuffer(&buf);
    for (0..n) |_| {
        const c = readInt(u32) - 1;
        var v = readInt(u32);
        assert(v != 0);
        if (max[c] < v) {
            mem.swap(u32, &max[c], &v);
        }
        if (v != 0) {
            other.appendAssumeCapacity(v);
        }
    }
    select.selectNth(u32, max[0..n], m - 1, {}, std.sort.desc(u32));
    var ans: u64 = 0;
    for (max[0..m]) |v| ans += v;
    for (max[m..n]) |v| {
        if (v != 0) other.appendAssumeCapacity(v);
    }
    const rem = k - m;
    if (rem > 0) {
        select.selectNth(u32, other.items, rem - 1, {}, std.sort.desc(u32));
        for (other.items[0..rem]) |v| ans += v;
    }
    print("{d}\n", .{ans});
}

const FixedQueue = lib.FixedQueue;
const FixedDeque = lib.FixedDeque;
const ModInt = lib.ModInt;
const PrimeModInt = lib.PrimeModInt;
const ModIntEx = lib.ModIntEx;
const Unionfind = lib.Unionfind;


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

    fn nextToken() !?[]u8 {
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

    fn nextInt(comptime Int: type) !?Int {
        const token = try nextToken() orelse return null;
        return try std.fmt.parseInt(Int, token, 10);
    }
};

const OptimizedScanner = struct {
    var input_buf: [MAX_INPUT_SIZE]u8 = undefined;
    var input: []const u8 = undefined;
    var pos: usize = 0;
    var buf_pos: usize = 1;

    fn init() !void {
        const ptr = try std.posix.mmap(null, MAX_INPUT_SIZE, std.posix.PROT.READ, .{
            .TYPE = .PRIVATE,
        }, std.posix.STDIN_FILENO, 0);
        input = ptr[0..MAX_INPUT_SIZE];
        input_buf[0] = 0;
    }

    fn getChar() ?u8 {
        defer pos += 1;
        return input[pos];
    }

    fn nextInt(comptime Int: type) !?Int {
        if (skip_delim) {
            while (input[pos] <= ' ') pos += 1;
        }
        var result: Int = 0;
        var ch = getChar() orelse return null;
        var neg = false;
        if (@typeInfo(Int).int.signedness == .signed) {
            if (ch == '-') {
                neg = true;
                ch = getChar().?;
            }
        }
        while (ch >= '0') {
            result = result * 10 + @as(Int, @intCast(ch - '0'));
            ch = getChar().?;
        }
        if (@typeInfo(Int).int.signedness == .signed) {
            if (neg) result = -result;
        }
        return result;
    }

    fn nextToken() !?[]u8 {
        if (skip_delim) {
            while (input[pos] <= ' ') pos += 1;
        }
        const old_buf_pos = buf_pos;
        while (input[pos] > ' ') {
            input_buf[buf_pos] = input[pos];
            buf_pos += 1;
            pos += 1;
        }
        defer pos += 1;
        return input_buf[old_buf_pos..buf_pos];
    }
};

const Scanner = if (force_optimized) OptimizedScanner else switch (builtin.mode) {
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
    // return value catch |err| {
    //     std.log.err("{any}", .{err});
    //     @panic("");
    // };
    return value catch unreachable;
}

fn readInt(comptime Int: type) Int {
    return readIntOptional(Int).?;
}

fn readIntOptional(comptime Int: type) ?Int {
    return ignoreError(Scanner.nextInt(Int));
}

fn readString() []u8 {
    return readStringOptional().?;
}

fn readStringOptional() ?[]u8 {
    return ignoreError(Scanner.nextToken());
}

fn readChar() u8 {
    if (Scanner == DebugScanner) {
        const token = readString();
        std.debug.assert(token.len == 1);
        return token[0];
    } else {
        assert(Scanner == OptimizedScanner);
        defer OptimizedScanner.pos += 1;
        return OptimizedScanner.getChar().?;
    }
}

fn print(comptime fmt: []const u8, args: anytype) void {
    ignoreError(stdout.print(fmt, args));
}

};

pub fn main() !void {
    if (global.safety) {
        try global.Scanner.init();
        try global.solve();
        try global.stdout.flush();
    } else {
        global.Scanner.init() catch unreachable;
        global.solve() catch unreachable;
        global.stdout.flush() catch unreachable;
    }
}

const lib = struct {

const std = @import("std");
const mem = std.mem;
const math = std.math;

fn FixedQueue(comptime T: type, comptime max_size: u32) type {
    return struct {
        const Self = @This();
        buf: [max_size]T = undefined,
        rp: u32 = 0,
        wp: u32 = 0,

        pub fn clear(self: *Self) void {
            self.rp = 0;
            self.wp = 0;
        }
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

fn FixedDeque(comptime T: type, comptime front_cap: comptime_int, comptime back_cap: comptime_int) type {
    return struct {
        const Self = @This();
        buf: [front_cap+back_cap]T = undefined,
        head: u32 = front_cap,
        tail: u32 = front_cap,

        pub fn clear(self: *Self) void {
            self.head = front_cap;
            self.tail = front_cap;
        }
        pub fn len(self: *const Self) u32 {
            return self.tail - self.head;
        }
        pub fn isEmpty(self: *const Self) bool {
            return self.tail == self.head;
        }
        pub fn pushBack(self: *Self, item: T) void {
            self.buf[self.tail] = item;
            self.tail += 1;
        }
        pub fn pushFront(self: *Self, item: T) void {
            self.head -= 1;
            self.buf[self.head] = item;
        }
        pub fn popBack(self: *Self) ?T {
            if (self.isEmpty()) return null;
            self.tail -= 1;
            return self.buf[self.tail];
        }
        pub fn popFront(self: *Self) ?T {
            if (self.isEmpty()) return null;
            defer self.head += 1;
            return self.buf[self.head];
        }
        pub fn front(self: *const Self) ?T {
            if (self.isEmpty()) return null;
            return self.buf[self.head];
        }
        pub fn back(self: *const Self) ?T {
            if (self.isEmpty()) return null;
            return self.buf[self.tail - 1];
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
};
const select = struct {
    const std = @import("std");
    const assert = std.debug.assert;

    const INSERTION_THRESHOLD = 16;

    pub fn selectNth(
        comptime T: type,
        items: []T,
        index: usize,
        context: anytype,
        comptime lessThanFn: fn (ctx: @TypeOf(context), lhs: T, rhs: T) bool,
    ) void {
        if (index == items.len) return;
        var array = items;
        var idx = index;
        var limit: u8 = 16;
        var ancestor_pivot: ?T = null;

        while (true) {
            assert(idx < array.len);
            if (idx == 0) {
                const minIdx = argMin(T, array, context, lessThanFn);
                std.mem.swap(T, &array[minIdx], &array[0]);
                return;
            }
            if (idx == array.len - 1) {
                const maxIdx = argMax(T, array, context, lessThanFn);
                std.mem.swap(T, &array[maxIdx], &array[array.len - 1]);
                return;
            }
            if (array.len <= INSERTION_THRESHOLD) {
                std.sort.insertionContext(0, array.len, struct {
                    inner: @TypeOf(context),
                    items: []T,
                    pub fn swap(ctx: @This(), ind1: usize, ind2: usize) void {
                        std.mem.swap(T, &ctx.items[ind1], &ctx.items[ind2]);
                    }
                    pub fn lessThan(ctx: @This(), ind1: usize, ind2: usize) bool {
                        return lessThanFn(ctx.inner, ctx.items[ind1], ctx.items[ind2]);
                    }
                } {
                    .inner = context,
                    .items = array,
                });
                return;
            }

            var piv: usize = undefined;
            if (limit == 0) {
                piv = medianOfNinthers(T, array, context, lessThanFn);
            } else {
                limit -= 1;
                piv = median3rec(T, array, context, lessThanFn);
            }

            if (ancestor_pivot) |p| {
                if (!lessThanFn(context, p, array[piv])) {
                    // p == array[piv]
                    const eq_cnt = partition(
                        T,
                        array,
                        piv,
                        context,
                        struct {
                            fn lessThanEq(ctx: @TypeOf(context), lhs: T, rhs: T) bool {
                                return !lessThanFn(ctx, rhs, lhs);
                            }
                        }.lessThanEq
                    ) + 1;
                    if (idx < eq_cnt) {
                        return;
                    }
                    array = array[eq_cnt..];
                    idx -= eq_cnt;
                    ancestor_pivot = null;
                    continue;
                }
            }

            const piv_idx = partition(T, array, piv, context, lessThanFn);

            assert(piv_idx < array.len);
            switch (std.math.order(piv_idx, idx)) {
                .lt => {
                    ancestor_pivot = array[piv_idx];
                    array = array[piv_idx + 1..];
                    idx -= piv_idx + 1;
                },
                .eq => return,
                .gt => {
                    array = array[0..piv_idx];
                }
            }
        }
    }

    fn medianOfNinthers(
        comptime T: type,
        array: []T,
        context: anytype,
        comptime lessThanFn: fn (ctx: @TypeOf(context), lhs: T, rhs: T) bool,
    ) usize {
        const frac = if (array.len <= 1024)
            array.len / 12
        else if (array.len <= 128*1024)
            array.len / 64
        else
            array.len / 1024;

        const piv = frac / 2;
        const lo = array.len / 2 - piv;
        const hi = frac + lo;
        const gap = (array.len - 9 * frac) / 4;
        var a = lo - 4 * frac - gap;
        var b = hi + gap;
        for (lo..hi) |i| {
            ninther(T, array, a, i - frac, b, a + 1, i, b + 1, a + 2, i + frac, b + 2, context, lessThanFn);
            a += 3;
            b += 3;
        }
        selectNth(T, array[lo..lo + frac], piv, context, lessThanFn);
        return lo + piv;
    }

    fn ninther(
        comptime T: type,
        array: []T,
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        e: usize,
        f: usize,
        g: usize,
        h: usize,
        i: usize,
        context: anytype,
        comptime lessThanFn: fn (ctx: @TypeOf(context), lhs: T, rhs: T) bool,
    ) void {
        var nb = medianIndex(T, array, a, b, c, context, lessThanFn);
        var nh = medianIndex(T, array, g, h, i, context, lessThanFn);
        if (lessThanFn(context, array[nh], array[nb])) {
            std.mem.swap(usize, &nb, &nh);
        }
        var nf = f;
        var nd = d;
        if (lessThanFn(context, array[nf], array[nd])) {
            std.mem.swap(usize, &nd, &nf);
        }
        if (lessThanFn(context, array[e], array[nd])) {

        } else if (lessThanFn(context, array[nf], array[e])) {
            nd = nf;
        } else {
            if (lessThanFn(context, array[e], array[nb])) {
                std.mem.swap(T, &array[e], &array[nd]);
            } else if (lessThanFn(context, array[nh], array[e])) {
                std.mem.swap(T, &array[nh], &array[e]);
            }
            return;
        }
        if (lessThanFn(context, array[nd], array[nb])) {
            nd = nb;
        } else if (lessThanFn(context, array[nh], array[nd])) {
            nd = nh;
        }
        std.mem.swap(T, &array[nd], &array[e]);
    }

    inline fn medianIndex(
        comptime T: type,
        array: []const T,
        a: usize,
        b: usize,
        c: usize,
        context: anytype,
        comptime lessThanFn: fn (ctx: @TypeOf(context), lhs: T, rhs: T) bool,
    ) usize {
        var na = a;
        var nc = c;
        if (lessThanFn(context, array[c], array[a])) {
            std.mem.swap(usize, &na, &nc);
        }
        if (lessThanFn(context, array[nc], array[b])) return nc;
        if (lessThanFn(context, array[b], array[na])) return na;
        return b;
    }

    fn argMin(
        comptime T: type,
        items: []T,
        context: anytype,
        comptime lessThanFn: fn (ctx: @TypeOf(context), lhs: T, rhs: T) bool,
    ) usize {
        assert(items.len > 0);
        var min = items[0];
        var idx: usize = 0;
        for (1.., items[1..]) |i, v| {
            if (lessThanFn(context, v, min)) {
                min = v;
                idx = i;
            }
        }
        return idx;
    }

    fn argMax(
        comptime T: type,
        items: []T,
        context: anytype,
        comptime lessThanFn: fn(ctx: @TypeOf(context), lhs: T, rhs: T) bool,
    ) usize {
        assert(items.len > 0);
        var max = items[0];
        var idx: usize = 0;
        for (1.., items[1..]) |i, v| {
            if (lessThanFn(context, max, v)) {
                max = v;
                idx = i;
            }
        }
        return idx;
    }

    pub fn partition(
        comptime T: type,
        items: []T,
        piv_idx: usize,
        context: anytype,
        comptime lessThanFn: fn (ctx: @TypeOf(context), lhs: T, rhs: T) bool,
    ) usize {
        const piv = items[piv_idx];
        items[piv_idx] = items[0];
        var j: usize = 1;
        for (1..items.len) |i| {
            std.mem.swap(T, &items[i], &items[j]);
            j += @intFromBool(lessThanFn(context, items[j], piv));
        }
        j -= 1;
        assert(j < items.len);
        items[0] = items[j];
        items[j] = piv;
        return j;
    }

    fn median3rec(
        comptime T: type,
        items: []const T,
        context: anytype,
        comptime lessThanFn: fn (ctx: @TypeOf(context), lhs: T, rhs: T) bool,
    ) usize {
        if (items.len < 8) return 0;
        const div8 = items.len / 8;
        const items_a = items[0..div8];
        const a = median3rec(T, items_a, context, lessThanFn);
        const items_b = items[div8*4..][0..div8];
        const b = median3rec(T, items_b, context, lessThanFn) + div8 * 4;
        const items_c = items[div8*7..][0..div8];
        const c = median3rec(T, items_c, context, lessThanFn) + div8 * 7;
        return medianIndex(T, items, a, b, c, context, lessThanFn);
    }
};
