#!/bin/bash

# Performance profiling script for Pidgin compiler
# This script runs various benchmarks and performance tests

echo "=== Pidgin Compiler Performance Profiling ==="

# Build release version for profiling
echo "Building release version..."
cargo build --release

# Test 1: Basic performance benchmark
echo "Running basic performance benchmark..."
time cargo run --release examples/performance_test.pg

# Test 2: Memory usage analysis
echo "Analyzing memory usage..."
/usr/bin/time -v cargo run --release examples/performance_test.pg 2>&1 | grep -E "(Maximum resident set size|User time|System time)" || true

# Test 3: Large array operations
echo "Testing large array operations..."
cat > /tmp/large_array_test.pg << 'EOF'
let large_array = {};
let i = 0;
while (i < 1000) {
    large_array = large_array.push(i);
    i = i + 1;
}
print "Created array with {} elements", large_array.length();
EOF

time cargo run --release /tmp/large_array_test.pg

# Test 4: String operations performance
echo "Testing string operations performance..."
cat > /tmp/string_test.pg << 'EOF'
let long_string = "This is a very long string that we will manipulate many times";
let i = 0;
while (i < 100) {
    let upper = long_string.toUpper();
    let lower = long_string.toLower();
    let trimmed = long_string.trim();
    let replaced = long_string.replaceChar`very -> extremely`;
    i = i + 1;
}
print "String operations completed";
EOF

time cargo run --release /tmp/string_test.pg

# Test 5: Object operations performance
echo "Testing object operations performance..."
cat > /tmp/object_test.pg << 'EOF'
let obj = Object();
let i = 0;
while (i < 100) {
    let key = "key" + i;
    let value = "value" + i;
    obj = obj.set(key, value);
    let retrieved = obj.get(key);
    let has_key = obj.has(key);
    i = i + 1;
}
print "Object operations completed";
EOF

time cargo run --release /tmp/object_test.pg

# Test 6: Function call performance
echo "Testing function call performance..."
cat > /tmp/function_test.pg << 'EOF'
function fibonacci(n) {
    if (n <= 1) {
        return n;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}

let result = fibonacci(20);
print "Fibonacci(20) = {}", result;
EOF

time cargo run --release /tmp/function_test.pg

# Test 7: Array method performance
echo "Testing array method performance..."
cat > /tmp/array_methods_test.pg << 'EOF'
let arr = {1, 2, 3, 4, 5};
let i = 0;
while (i < 100) {
    arr = arr.push(i);
    let len = arr.length();
    arr = arr.reverse();
    arr = arr.reverse(); // Reverse back
    i = i + 1;
}
print "Array method operations completed";
EOF

time cargo run --release /tmp/array_methods_test.pg

# Test 8: Date operations performance
echo "Testing date operations performance..."
cat > /tmp/date_test.pg << 'EOF'
let i = 0;
while (i < 100) {
    let now = Date();
    let year = now.getYear();
    let month = now.getMonth();
    let day = now.getDay();
    let formatted = now.format("%Y-%m-%d");
    i = i + 1;
}
print "Date operations completed";
EOF

time cargo run --release /tmp/date_test.pg

# Cleanup
rm -f /tmp/large_array_test.pg /tmp/string_test.pg /tmp/object_test.pg /tmp/function_test.pg /tmp/array_methods_test.pg /tmp/date_test.pg

echo "=== Performance Profiling Completed ==="
echo "Check the output above for timing information." 