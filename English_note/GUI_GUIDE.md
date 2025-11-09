# 🖥️ High-Performance Prime Number Calculator - GUI Version

## 📋 Introduction

This is the GUI version of the high-performance prime number calculator, built using the egui framework. It provides an intuitive and easy-to-use graphical interface for calculating and visualizing prime numbers.

## 🚀 Quick Start

### Method 1: Run GUI Version
```bash
cargo run --bin prime-gui
```

### Method 2: Compile and Run
```bash
cargo build --bin prime-gui
./target/debug/prime-gui.exe  # Windows
```

### Method 3: Run Both Versions Simultaneously
- GUI: `cargo run --bin prime-gui`
- CLI: `cargo run --bin prime-cli`

## 🎛️ Interface Features

### 📊 Main Input Area
- **Maximum Value Input**：Enter the upper limit for prime number calculation
  - Supports 1 to 10,000,000,000 (10 billion)
  - Examples: 1000, 10000, 1000000
- **Calculate Button**：Click to start calculating prime numbers

### 📈 Real-time Statistics
After calculation completes, it displays:
- **Total Primes**: Number of prime numbers found
- **Calculation Time**: Algorithm execution time (seconds)
- **Processing Speed**: Numbers processed per second

### ⚙️ Options and Settings (Collapsible Panel)

#### 🎛️ Basic Options
- **Show Composites**: When checked, display composite numbers in results list (shown in gray)
- **Range Filtering**:
  - Can set specific number ranges for display
  - Enter start value and end value to filter results

#### 🧪 Special Testing Features
- **Auto Test Problem Numbers**: When checked, automatically test 21474836359 if calculation range includes it
- **🔍 Test Problem Numbers**: Manually test known composite 21474836359 = 17 × 126322727

### 📋 Results Display Area
- **Prime Display**: Green highlighted display (e.g.,      2,      3,      5...)
- **Composite Display**: Gray display (when "Show Composites" option is enabled)
- **Scrollable List**: Supports browsing large amounts of data
- **Formatted Display**: Shows 10 numbers per line with neat alignment

### 🧪 Test Results Display
Special test results appear below statistics:
- **✅ Success**: Algorithm correctly identified composite number
- **❌ Failed**: Algorithm incorrectly marked composite as prime

## 💡 Usage Examples

### Example 1: Calculate Small Range of Primes
1. Enter in "Maximum Value" input: 1000
2. Click "🚀 Calculate Primes"
3. View results: Should show 168 primes from 2 to 997

### Example 2: Test Specific Range
1. Enter in "Maximum Value" input: 100
2. Expand options panel, check "Range Filtering"
3. Set range: 50 to 80
4. Click calculate
5. View primes in specific range: 53, 59, 61, 67, 71, 73, 79

### Example 3: Verify Algorithm Fix
1. Enter in "Maximum Value" input: 21474836400
2. Check "Auto Test Problem Numbers"
3. Click calculate
4. Wait for calculation to complete
5. View test results: Should show "✅ Correct: 21474836359 correctly identified as composite"

## 🎨 Interface Features

### 🌈 Visual Design
- **Modern Interface**: Uses egui's modern design style
- **Responsive Layout**: Window size is adjustable
- **Color Coding**:
  - 🟢 Green: Prime numbers
  - 🔘 Gray: Composite numbers
  - 🔴 Red: Error messages
  - 🟣 Blue: Information messages

### 📱 User Experience
- **Real-time Feedback**: Progress indicator during calculation
- **Error Handling**: Friendly error messages
- **Status Bar**: Shows current status and tips at bottom
- **Scroll Support**: Large datasets can be viewed through scrolling

## 🔧 Technical Features

### ⚡ Performance Optimization
- **Segmented Sieve Algorithm**: Maintains high performance when processing large number ranges
- **Memory Management**: Optimized memory usage, prevents overflow
- **Multi-thread Safe**: Interface response separated from calculation

### 🛡️ Security Features
- **Input Validation**: Prevents program crashes from invalid input
- **Boundary Checking**: Prevents integer overflow
- **Error Recovery**: Provides clear error messages when calculation errors occur

## 📝 Important Notes

### ⚠️ Performance Recommendations
- Calculation time increases significantly for values over 1,000,000,000 (1 billion)
- Recommended to start testing with small ranges for first-time users

### 🔒 Memory Limitations
- Very large input values may require significant memory
- Program automatically limits maximum input value to 10 billion

### 🧪 Testing Recommendations
- Use "Test Problem Numbers" feature to verify algorithm correctness
- Check primes in 1-100 range as basic verification

## 🆚 Comparison with CLI Version

| Feature | GUI Version | CLI Version |
|---------|-------------|-------------|
| Ease of Use | ⭐⭐⭐⭐⭐ Simple and intuitive | ⭐⭐⭐ Requires command-line knowledge |
| Visualization | ⭐⭐⭐⭐⭐ Color coding and layout | ⭐⭐ Plain text output |
| Interactivity | ⭐⭐⭐⭐⭐ Real-time interaction | ⭐⭐ Batch processing mode |
| Performance | ⭐⭐⭐⭐ Slight GUI overhead | ⭐⭐⭐⭐⭐ Best performance |
| Automation | ⭐⭐ Suitable for manual operation | ⭐⭐⭐⭐⭐ Suitable for scripting |

## 🔗 Related Files

- **GUI Main Program**: `src/gui_main.rs`
- **Core Algorithm**: `src/prime_sieve.rs`
- **Test Suite**: `tests/prime_sieve_tests.rs`
- **Test Results**: `TEST_RESULTS.md`

## 🎉 Enjoy Using!

Now you can enjoy the convenience brought by the graphical interface and easily calculate and explore the wonderful world of prime numbers! 🎊