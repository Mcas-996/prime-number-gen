use eframe::egui;
use prime::prime_sieve;
use std::fs;
use std::path::Path;
use std::time::Instant;

#[derive(Default)]
struct PrimeCalculatorApp {
    // Input state
    max_value_input: String,
    calculating: bool,

    // Computation results
    primes: Vec<u64>,
    calculation_time: f64,
    processing_speed: u64,
    error_message: Option<String>,

    // Options and filtering
    show_composites: bool,
    filter_range: bool,
    range_start: String,
    range_end: String,

    // Special test controls
    testing_problematic: bool,
    test_result: Option<TestResult>,

    // Summary statistics
    total_primes: usize,
}

#[derive(Clone, Debug)]
struct TestResult {
    is_composite: bool,
    message: String,
}

impl PrimeCalculatorApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }

    fn calculate_primes(&mut self) {
        // Clear previous results
        self.error_message = None;
        self.test_result = None;

        // Parse user input
        match self.max_value_input.trim().parse::<u64>() {
            Ok(max_value) => {
                if max_value < 2 {
                    self.error_message = Some("Value must be at least 2".to_string());
                    return;
                }

                if max_value > 10_000_000_000 {
                    self.error_message = Some(
                        "Value is too large and may take a very long time to compute".to_string(),
                    );
                    return;
                }

                // Start the calculation
                self.calculating = true;

                // Execute the computation
                let start_time = Instant::now();
                match std::panic::catch_unwind(|| {
                    prime_sieve::segmented_sieve(max_value, 1_000_000)
                }) {
                    Ok(primes) => {
                        let duration = start_time.elapsed();

                        self.primes = primes;
                        self.total_primes = self.primes.len();
                        self.calculation_time = duration.as_secs_f64();
                        self.processing_speed = if duration.as_secs_f64() > 0.0 {
                            (max_value as f64 / duration.as_secs_f64()) as u64
                        } else {
                            0
                        };

                        // Automatically test the problematic number
                        if max_value >= 21474836359 && self.testing_problematic {
                            self.test_problematic_number();
                        }
                    }
                    Err(_) => {
                        self.error_message =
                            Some("An error occurred during the calculation".to_string());
                    }
                }

                self.calculating = false;
            }
            Err(_) => {
                self.error_message = Some("Invalid numeric input".to_string());
                self.calculating = false;
            }
        }
    }

    fn test_problematic_number(&mut self) {
        let problem_number = 21474836359;

        let is_marked_as_prime = self.primes.contains(&problem_number);

        // Manual verification
        let manual_result = self.verify_composite(problem_number);

        self.test_result = Some(TestResult {
            is_composite: !is_marked_as_prime && manual_result.is_composite,
            message: if is_marked_as_prime && manual_result.is_composite {
                format!(
                    "❌ Error: {} was marked as prime but is composite = {} x {}",
                    21474836359u64, manual_result.factor1, manual_result.factor2
                )
            } else if !is_marked_as_prime && manual_result.is_composite {
                format!(
                    "✅ Correct: {} was correctly marked as composite = {} x {}",
                    21474836359u64, manual_result.factor1, manual_result.factor2
                )
            } else {
                format!("Unexpected test result for 21474836359")
            },
        });
    }

    fn verify_composite(&self, num: u64) -> FactorResult {
        if num < 2 {
            return FactorResult {
                is_composite: false,
                factor1: 0,
                factor2: 0,
            };
        }

        let sqrt_num = (num as f64).sqrt() as u64 + 1;
        for i in 2..=sqrt_num {
            if num % i == 0 {
                return FactorResult {
                    is_composite: true,
                    factor1: i,
                    factor2: num / i,
                };
            }
        }

        FactorResult {
            is_composite: false,
            factor1: 0,
            factor2: 0,
        }
    }

    fn get_filtered_numbers(&self) -> Vec<NumberInfo> {
        let mut numbers = Vec::new();

        let max_val = match self.primes.last() {
            Some(&val) => val,
            None => return numbers,
        };
        let mut range_start = if self.filter_range {
            self.range_start.parse().unwrap_or(1)
        } else {
            1
        };
        if range_start == 0 {
            range_start = 1;
        }

        let mut range_end = if self.filter_range {
            self.range_end.parse().unwrap_or(max_val)
        } else {
            max_val
        };
        if range_end > max_val {
            range_end = max_val;
        }

        if range_start > max_val {
            return numbers;
        }

        if range_start > range_end {
            std::mem::swap(&mut range_start, &mut range_end);
        }

        let mut prime_iter = self.primes.iter().peekable();

        while let Some(&p) = prime_iter.peek() {
            if *p < range_start {
                prime_iter.next();
            } else {
                break;
            }
        }

        for num in range_start..=range_end {
            while let Some(&p) = prime_iter.peek() {
                if *p < num {
                    prime_iter.next();
                } else {
                    break;
                }
            }

            let is_prime = prime_iter.peek().map(|&&p| p == num).unwrap_or(false);
            if is_prime {
                prime_iter.next(); // Consume this prime entry
            }

            if is_prime || self.show_composites {
                numbers.push(NumberInfo {
                    number: num,
                    is_prime,
                });
            }
        }

        numbers
    }
}

#[derive(Clone, Debug)]
struct NumberInfo {
    number: u64,
    is_prime: bool,
}

#[derive(Clone, Debug)]
struct FactorResult {
    is_composite: bool,
    factor1: u64,
    factor2: u64,
}

impl eframe::App for PrimeCalculatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🔢 High Performance Prime Calculator - GUI Edition");
            ui.separator();

            // Input area
            ui.horizontal(|ui| {
                ui.label("Max value:");
                ui.add(
                    egui::TextEdit::singleline(&mut self.max_value_input)
                        .desired_width(150.0)
                        .hint_text("e.g. 1000000"),
                );

                if ui
                    .add_enabled(!self.calculating, egui::Button::new("🚀 Calculate primes"))
                    .clicked()
                {
                    self.calculate_primes();
                }

                if self.calculating {
                    ui.spinner();
                    ui.label("Calculating...");
                }
            });

            // Error message
            if let Some(ref error) = self.error_message {
                ui.colored_label(egui::Color32::RED, format!("❌ {}", error));
            }

            ui.separator();

            // Result statistics
            if !self.primes.is_empty() && !self.calculating {
                ui.horizontal(|ui| {
                    ui.label("📊 Statistics:");
                    ui.separator();
                    ui.label(format!("Total primes: {}", self.total_primes));
                    ui.label(format!("Elapsed time: {:.2}s", self.calculation_time));
                    ui.label(format!(
                        "Processing speed: {} numbers/second",
                        self.processing_speed
                    ));
                });

                ui.separator();
            }

            // Option controls
            egui::CollapsingHeader::new("⚙️ Options & Settings").show(ui, |ui| {
                ui.checkbox(&mut self.show_composites, "Show composites");

                ui.horizontal(|ui| {
                    ui.checkbox(&mut self.filter_range, "Range filter:");
                    if self.filter_range {
                        ui.add(
                            egui::TextEdit::singleline(&mut self.range_start)
                                .desired_width(80.0)
                                .hint_text("Start"),
                        );
                        ui.label("-");
                        ui.add(
                            egui::TextEdit::singleline(&mut self.range_end)
                                .desired_width(80.0)
                                .hint_text("End"),
                        );
                    }
                });

                ui.checkbox(
                    &mut self.testing_problematic,
                    "Automatically test problematic number (21474836359)",
                );

                if ui.button("🔍 Test problematic number").clicked() {
                    self.test_problematic_number();
                }
            });

            // Test results
            if let Some(ref test_result) = self.test_result {
                ui.separator();
                ui.colored_label(
                    if test_result.is_composite {
                        egui::Color32::GREEN
                    } else {
                        egui::Color32::RED
                    },
                    &test_result.message,
                );
            }

            ui.separator();

            // Prime list
            if !self.primes.is_empty() && !self.calculating {
                let filtered_numbers = self.get_filtered_numbers();

                ui.label(format!(
                    "📋 Result list (showing {} numbers):",
                    filtered_numbers.len()
                ));

                egui::ScrollArea::vertical()
                    .max_height(300.0)
                    .show(ui, |ui| {
                        let chunk_size = 10; // Numbers per row

                        for chunk in filtered_numbers.chunks(chunk_size) {
                            ui.horizontal(|ui| {
                                for info in chunk {
                                    if info.is_prime {
                                        ui.colored_label(
                                            egui::Color32::from_rgb(0, 150, 0),
                                            format!("{:>8}", info.number),
                                        );
                                    } else {
                                        ui.colored_label(
                                            egui::Color32::GRAY,
                                            format!("{:>8}", info.number),
                                        );
                                    }
                                }
                            });
                        }
                    });
            }
        });

        // Status bar
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("💡 Tips:");
                ui.label("Enter a number to calculate every prime within that range");
                ui.separator();
                if self.primes.len() > 0 {
                    ui.label(format!(
                        "Currently showing: {} numbers",
                        self.get_filtered_numbers().len()
                    ));
                }
            });
        });
    }
}

// Attempt to load font files
fn load_font_data() -> Option<egui::FontData> {
    // Try different font paths in order of preference
    let font_paths = vec![
        // Windows font paths
        "C:\\Windows\\Fonts\\msyh.ttc",   // Microsoft YaHei
        "C:\\Windows\\Fonts\\simhei.ttf", // SimHei
        "C:\\Windows\\Fonts\\simsun.ttc", // SimSun
        // macOS font paths
        "/System/Library/Fonts/PingFang.ttc",
        "/System/Library/Fonts/Hiragino Sans GB.ttc",
        // Linux font paths
        "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
        "/usr/share/fonts/truetype/wqy/wqy-zenhei.ttc",
        "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
        // Fonts located in the project root
        "msyh.ttc",
        "simhei.ttf",
    ];

    for path in font_paths {
        if Path::new(path).exists() {
            if let Ok(font_bytes) = fs::read(path) {
                println!("Successfully loaded font: {}", path);
                return Some(egui::FontData::from_owned(font_bytes));
            }
        }
    }

    println!("Warning: No suitable CJK font found; using the default font");
    None
}

fn main() -> Result<(), eframe::Error> {
    // Configure fonts
    let mut fonts = egui::FontDefinitions::default();

    if let Some(font_data) = load_font_data() {
        // Register the loaded font
        fonts.font_data.insert("chinese_font".to_owned(), font_data);

        // Make it the default font
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "chinese_font".to_owned());

        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .push("chinese_font".to_owned());
    }

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([900.0, 700.0]),
        ..Default::default()
    };

    eframe::run_native(
        "High Performance Prime Calculator",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_fonts(fonts);
            Box::new(PrimeCalculatorApp::new(cc))
        }),
    )
}
