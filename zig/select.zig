const select = struct {
const std = @import("std");
const assert = std.debug.assert;
const INSERTION_THRESHOLD = 16;

pub fn select_nth(
    comptime T: type,
    items: []T,
    index: usize,
    context: anytype,
    comptime lessThanFn: fn (ctx: @TypeOf(context), lhs: T, rhs: T) bool,
) void {
    assert(index < items.len);
    var array = items;
    var idx = index;
    while (true) {
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
        const piv = pivot(T, array, context, lessThanFn);
        const piv_idx = partition(T, array[piv+1..], array[piv], context, lessThanFn) + piv;
        assert(piv_idx < array.len);
        switch (std.math.order(piv_idx, idx)) {
            .lt => {
                array = array[piv_idx + 1..];
                idx -= piv_idx - 1;
            },
            .eq => return,
            .gt => {
                std.mem.swap(T, &array[piv], &array[piv_idx]);
                array = array[0..piv_idx];
            }
        }
    }
}

pub fn sortSmall(
    comptime T: type,
    comptime len: usize,
    items: *[len]T,
    context: anytype,
    comptime lessThanFn: fn (ctx: @TypeOf(context), lhs: T, rhs: T) bool,
) void {
    if (len == 0) @compileError("zero size array is not allowed");
    inline for (1..len) |i| {
        comptime var j = i;
        inline while (j > 0 and lessThanFn(context, items[j], items[j - 1])) j -= 1;
    }
}

pub fn pivot(
    comptime T: type,
    items: []T,
    context: anytype,
    comptime lessThanFn: fn (ctx: @TypeOf(context), lhs: T, rhs: T) bool,
) usize {
    const div = items.len / 5;
    const rem = items.len % 5;
    for (0..div) |i| {
        sortSmall(T, 5, items[i * 5..(i + 1) * 5], context, lessThanFn);
        std.mem.swap(T, &items[i], &items[i * 5 + 2]);
    }
    var rec_len = div;
    switch (rem) {
        0 => {},
        inline 1...4 => |len| {
            sortSmall(T, len, items[div * 5..][0..len], context, lessThanFn);
            std.mem.swap(T, &items[rec_len], &items[div * 5 + len / 2]);
            rec_len += 1;
        }
    }
    select_nth(T, items[0..rec_len], rec_len / 2, context, lessThanFn);
    return rec_len / 2;
}

pub fn partition(
    comptime T: type,
    items: []T,
    piv: T,
    context: anytype,
    comptime lessThanFn: fn (ctx: @TypeOf(context), lhs: T, rhs: T) bool,
) usize {
    var i: usize = 0;
    while (i < items.len and lessThanFn(context, items[i], piv)) i += 1;
    var j: usize = i;
    i += 1;
    while (i < items.len) : (i += 1) {
        if (lessThanFn(context, items[i], piv)) {
            std.mem.swap(T, &items[j], &items[i]);
            j += 1;
        }
    }
    return j;
}
};