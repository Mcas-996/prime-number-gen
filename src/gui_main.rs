use eframe::egui;
use prime::prime_sieve;
use std::fs;
use std::path::Path;
use std::sync::mpsc::{self, Receiver, TryRecvError};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

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

    // Heartbeat for low refresh cadence
    last_tick: Instant,
    heartbeat_value: u64,

    // Async calculation channel
    calc_rx: Option<Receiver<Result<CalcResult, String>>>,

    // Terminal-like reader state
    terminal_offset: usize,
    page_buffer: String,
    page_buffer_offset: usize,
    page_buffer_lines_per_page: usize,
    page_buffer_total_lines: usize,
    page_buffer_primes_len: usize,
}

#[derive(Clone, Debug)]
struct TestResult {
    passed: bool,
    message: String,
}

#[derive(Debug)]
struct CalcResult {
    primes: Vec<u64>,
    duration_secs: f64,
    max_value: u64,
}

impl PrimeCalculatorApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            max_value_input: "100000".to_string(),
            calculating: false,
            primes: Vec::new(),
            calculation_time: 0.0,
            processing_speed: 0,
            error_message: None,
            show_composites: false,
            filter_range: false,
            range_start: String::new(),
            range_end: String::new(),
            testing_problematic: false,
            test_result: None,
            total_primes: 0,
            last_tick: Instant::now(),
            heartbeat_value: 0,
            calc_rx: None,
            terminal_offset: 0,
            page_buffer: String::new(),
            page_buffer_offset: usize::MAX,
            page_buffer_lines_per_page: 0,
            page_buffer_total_lines: 0,
            page_buffer_primes_len: 0,
        }
    }

    fn calculate_primes(&mut self) {
        self.error_message = None;
        self.test_result = None;

        if self.calculating {
            return;
        }

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

                self.calculating = true;
                self.calculation_time = 0.0;
                self.processing_speed = 0;
                self.terminal_offset = 0;
                self.page_buffer.clear();
                self.page_buffer_primes_len = 0;

                let (tx, rx) = mpsc::channel();
                self.calc_rx = Some(rx);

                thread::spawn(move || {
                    let start_time = Instant::now();
                    let result = std::panic::catch_unwind(|| {
                        let primes = prime_sieve::segmented_sieve(max_value, 1_000_000);
                        let duration_secs = start_time.elapsed().as_secs_f64();
                        CalcResult {
                            primes,
                            duration_secs,
                            max_value,
                        }
                    });

                    let _ = tx.send(match result {
                        Ok(res) => Ok(res),
                        Err(_) => Err("An error occurred during the calculation".to_string()),
                    });
                });
            }
            Err(_) => {
                self.error_message = Some("Invalid numeric input".to_string());
                self.calculating = false;
            }
        }
    }

    fn poll_calc_results(&mut self) {
        if let Some(rx) = self.calc_rx.as_ref() {
            match rx.try_recv() {
                Ok(Ok(res)) => {
                    self.primes = res.primes;
                    self.total_primes = self.primes.len();
                    self.calculation_time = res.duration_secs;
                    self.processing_speed = if res.duration_secs > 0.0 {
                        (res.max_value as f64 / res.duration_secs) as u64
                    } else {
                        0
                    };

                    if res.max_value >= 21474836359 && self.testing_problematic {
                        self.test_problematic_number();
                    }

                    self.calculating = false;
                    self.calc_rx = None;
                }
                Ok(Err(err)) => {
                    self.error_message = Some(err);
                    self.calculating = false;
                    self.calc_rx = None;
                }
                Err(TryRecvError::Empty) => {}
                Err(TryRecvError::Disconnected) => {
                    self.error_message =
                        Some("Calculation thread disconnected unexpectedly".to_string());
                    self.calculating = false;
                    self.calc_rx = None;
                }
            }
        }
    }

    fn handle_terminal_input(&mut self, ctx: &egui::Context, lines_per_page: usize) {
        let mut delta: isize = 0;
        ctx.input(|i| {
            if i.key_pressed(egui::Key::ArrowUp) {
                delta -= 1;
            }
            if i.key_pressed(egui::Key::ArrowDown) {
                delta += 1;
            }
            if i.key_pressed(egui::Key::PageUp) {
                delta -= lines_per_page as isize;
            }
            if i.key_pressed(egui::Key::PageDown) {
                delta += lines_per_page as isize;
            }
        });

        if delta != 0 {
            self.shift_terminal(delta);
        }
    }

    fn shift_terminal(&mut self, delta: isize) {
        const PRIMES_PER_LINE: usize = 10;
        let total_lines =
            (self.primes.len() + PRIMES_PER_LINE.saturating_sub(1)) / PRIMES_PER_LINE.max(1);
        if total_lines == 0 {
            self.terminal_offset = 0;
            return;
        }

        let current = self.terminal_offset as isize;
        let mut next = current + delta;
        if next < 0 {
            next = 0;
        }
        let max_start = total_lines.saturating_sub(1) as isize;
        if next > max_start {
            next = max_start;
        }
        self.terminal_offset = next as usize;
    }

    fn test_problematic_number(&mut self) {
        let problem_number = 21474836359u64;

        // Avoid confusing output when the current results do not even cover the test number.
        if self.primes.is_empty() || *self.primes.last().unwrap_or(&0) < problem_number {
            self.test_result = Some(TestResult {
                passed: false,
                message: format!(
                    "Run a calculation up to {} before running the self-test.",
                    problem_number
                ),
            });
            return;
        }

        let is_marked_as_prime = self.primes.binary_search(&problem_number).is_ok();
        let manual_result = self.verify_composite(problem_number);

        let (passed, message) = if manual_result.is_composite {
            if is_marked_as_prime {
                (
                    false,
                    format!(
                        "Error: {} was marked as prime but is composite = {} x {}",
                        problem_number, manual_result.factor1, manual_result.factor2
                    ),
                )
            } else {
                (
                    true,
                    format!(
                        "Correct: {} marked as composite = {} x {}",
                        problem_number, manual_result.factor1, manual_result.factor2
                    ),
                )
            }
        } else if is_marked_as_prime {
            (true, format!("Correct: {} is prime", problem_number))
        } else {
            (
                false,
                format!(
                    "Error: {} should be prime but was not found in the results",
                    problem_number
                ),
            )
        };

        self.test_result = Some(TestResult { passed, message });
    }

    fn verify_composite(&self, num: u64) -> FactorResult {
        if num < 2 {
            return FactorResult {
                is_composite: false,
                factor1: 0,
                factor2: 0,
            };
        }

        if num == 2 {
            return FactorResult {
                is_composite: false,
                factor1: 0,
                factor2: 0,
            };
        }

        // Check if even
        if num % 2 == 0 {
            return FactorResult {
                is_composite: true,
                factor1: 2,
                factor2: num / 2,
            };
        }

        // Check odd divisors up to sqrt(num)
        let sqrt_num = (num as f64).sqrt() as u64 + 1;
        let mut i = 3u64;
        while i <= sqrt_num {
            if num % i == 0 {
                return FactorResult {
                    is_composite: true,
                    factor1: i,
                    factor2: num / i,
                };
            }
            i += 2; // Only check odd numbers
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
                prime_iter.next();
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

    fn format_number(&self, num: u64) -> String {
        if num >= 1_000_000_000 {
            format!("{:.2}B", num as f64 / 1_000_000_000.0)
        } else if num >= 1_000_000 {
            format!("{:.2}M", num as f64 / 1_000_000.0)
        } else if num >= 1_000 {
            format!("{:.2}K", num as f64 / 1_000.0)
        } else {
            format!("{}", num)
        }
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
        self.poll_calc_results();
        // Run the UI at 60 Hz when idle, throttle to ~2 Hz while heavy calculation runs.
        let repaint_interval = if self.calculating {
            Duration::from_millis(500)
        } else {
            Duration::from_secs_f32(1.0 / 60.0)
        };
        ctx.request_repaint_after(repaint_interval);
        if self.last_tick.elapsed() >= Duration::from_secs(2) {
            self.heartbeat_value = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs() % 32_467)
                .unwrap_or(0);
            self.last_tick = Instant::now();
        }
        ctx.set_visuals(egui::Visuals::dark());

        // Top header panel
        egui::TopBottomPanel::top("header")
            .frame(egui::Frame {
                fill: egui::Color32::from_rgb(25, 30, 40),
                inner_margin: egui::Margin::symmetric(16.0, 12.0),
                ..Default::default()
            })
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("High Performance Prime Calculator")
                            .size(18.0)
                            .color(egui::Color32::from_rgb(100, 200, 255))
                            .strong(),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let (status_text, status_color) = if self.calculating {
                            ("计算中", egui::Color32::from_rgb(255, 160, 90))
                        } else {
                            ("空闲", egui::Color32::from_rgb(120, 200, 120))
                        };
                        ui.label(
                            egui::RichText::new(status_text)
                                .size(12.0)
                                .color(status_color)
                                .strong(),
                        );
                        ui.add_space(8.0);
                        ui.label(
                            egui::RichText::new("v1.0")
                                .size(11.0)
                                .color(egui::Color32::GRAY),
                        );
                    });
                });
            });

        // Left sidebar
        egui::SidePanel::left("sidebar")
            .resizable(false)
            .exact_width(280.0)
            .frame(egui::Frame {
                fill: egui::Color32::from_rgb(20, 24, 32),
                inner_margin: egui::Margin::symmetric(12.0, 16.0),
                ..Default::default()
            })
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    // Configuration Card
                    egui::Frame {
                        fill: egui::Color32::from_rgb(30, 35, 45),
                        rounding: egui::Rounding::same(8.0),
                        inner_margin: egui::Margin::same(14.0),
                        ..Default::default()
                    }
                    .show(ui, |ui| {
                        ui.label(
                            egui::RichText::new("Configuration")
                                .size(15.0)
                                .color(egui::Color32::WHITE)
                                .strong(),
                        );
                        ui.add_space(10.0);

                        ui.label(egui::RichText::new("Max Value:").size(12.0));
                        ui.add_space(4.0);

                        let text_edit = egui::TextEdit::singleline(&mut self.max_value_input)
                            .hint_text("e.g. 100000")
                            .desired_width(250.0);
                        ui.add(text_edit);

                        ui.add_space(10.0);

                        let btn_text = if self.calculating {
                            "Calculating..."
                        } else {
                            "Calculate Primes"
                        };

                        let button = egui::Button::new(egui::RichText::new(btn_text).size(13.0))
                            .fill(egui::Color32::from_rgb(60, 120, 200))
                            .rounding(4.0);

                        if ui
                            .add_sized([250.0, 32.0], button)
                            .on_disabled_hover_text("Calculation in progress...")
                            .clicked()
                        {
                            self.calculate_primes();
                        }

                        if self.calculating {
                            ui.add_space(8.0);
                            ui.horizontal(|ui| {
                                ui.spinner();
                                ui.label("Processing...");
                            });
                        }

                        if let Some(ref error) = self.error_message {
                            ui.add_space(8.0);
                            ui.colored_label(
                                egui::Color32::from_rgb(255, 100, 100),
                                format!("Error: {}", error),
                            );
                        }
                    });

                    ui.add_space(12.0);

                    // Options Card
                    egui::Frame {
                        fill: egui::Color32::from_rgb(30, 35, 45),
                        rounding: egui::Rounding::same(8.0),
                        inner_margin: egui::Margin::same(14.0),
                        ..Default::default()
                    }
                    .show(ui, |ui| {
                        ui.label(
                            egui::RichText::new("Options")
                                .size(15.0)
                                .color(egui::Color32::WHITE)
                                .strong(),
                        );
                        ui.add_space(10.0);

                        ui.checkbox(&mut self.show_composites, "Show composites");
                        ui.add_space(6.0);

                        ui.checkbox(&mut self.filter_range, "Filter range");
                        if self.filter_range {
                            ui.add_space(4.0);
                            ui.horizontal(|ui| {
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.range_start)
                                        .desired_width(55.0)
                                        .hint_text("Start"),
                                );
                                ui.label("-");
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.range_end)
                                        .desired_width(55.0)
                                        .hint_text("End"),
                                );
                            });
                        }

                        ui.add_space(10.0);
                        ui.separator();
                        ui.add_space(10.0);

                        ui.checkbox(&mut self.testing_problematic, "Auto-test 21474836359");

                        ui.add_space(6.0);
                        if ui.button("Test 21474836359 Now").clicked() {
                            self.test_problematic_number();
                        }
                    });

                    if let Some(ref test_result) = self.test_result {
                        ui.add_space(12.0);
                        egui::Frame {
                            fill: if test_result.passed {
                                egui::Color32::from_rgb(30, 60, 40)
                            } else {
                                egui::Color32::from_rgb(80, 30, 30)
                            },
                            rounding: egui::Rounding::same(8.0),
                            inner_margin: egui::Margin::same(10.0),
                            ..Default::default()
                        }
                        .show(ui, |ui| {
                            ui.add(
                                egui::Label::new(
                                    egui::RichText::new(&test_result.message)
                                        .size(12.0)
                                        .color(egui::Color32::WHITE),
                                )
                                .wrap(true),
                            );
                        });
                    }
                });
            });

        // Main content area
        egui::CentralPanel::default()
            .frame(egui::Frame {
                fill: egui::Color32::from_rgb(20, 24, 32),
                inner_margin: egui::Margin::same(16.0),
                ..Default::default()
            })
            .show(ctx, |ui| {
                // Statistics cards
                if !self.primes.is_empty() && !self.calculating {
                    ui.horizontal(|ui| {
                        // Card 1
                        egui::Frame {
                            fill: egui::Color32::from_rgb(30, 35, 45),
                            rounding: egui::Rounding::same(8.0),
                            inner_margin: egui::Margin::same(12.0),
                            ..Default::default()
                        }
                        .show(ui, |ui| {
                            ui.set_min_width(140.0);
                            ui.vertical(|ui| {
                                ui.label(
                                    egui::RichText::new("Total Primes")
                                        .size(11.0)
                                        .color(egui::Color32::GRAY),
                                );
                                ui.add_space(4.0);
                                ui.label(
                                    egui::RichText::new(format!("{}", self.total_primes))
                                        .size(18.0)
                                        .color(egui::Color32::from_rgb(100, 255, 150))
                                        .strong(),
                                );
                            });
                        });

                        ui.add_space(10.0);

                        // Card 2
                        egui::Frame {
                            fill: egui::Color32::from_rgb(30, 35, 45),
                            rounding: egui::Rounding::same(8.0),
                            inner_margin: egui::Margin::same(12.0),
                            ..Default::default()
                        }
                        .show(ui, |ui| {
                            ui.set_min_width(140.0);
                            ui.vertical(|ui| {
                                ui.label(
                                    egui::RichText::new("Elapsed Time")
                                        .size(11.0)
                                        .color(egui::Color32::GRAY),
                                );
                                ui.add_space(4.0);
                                ui.label(
                                    egui::RichText::new(format!("{:.3}s", self.calculation_time))
                                        .size(18.0)
                                        .color(egui::Color32::from_rgb(255, 200, 100))
                                        .strong(),
                                );
                            });
                        });

                        ui.add_space(10.0);

                        // Card 3
                        egui::Frame {
                            fill: egui::Color32::from_rgb(30, 35, 45),
                            rounding: egui::Rounding::same(8.0),
                            inner_margin: egui::Margin::same(12.0),
                            ..Default::default()
                        }
                        .show(ui, |ui| {
                            ui.set_min_width(140.0);
                            ui.vertical(|ui| {
                                ui.label(
                                    egui::RichText::new("Speed")
                                        .size(11.0)
                                        .color(egui::Color32::GRAY),
                                );
                                ui.add_space(4.0);
                                ui.label(
                                    egui::RichText::new(format!(
                                        "{}/s",
                                        self.format_number(self.processing_speed)
                                    ))
                                    .size(18.0)
                                    .color(egui::Color32::from_rgb(150, 150, 255))
                                    .strong(),
                                );
                            });
                        });
                    });

                    ui.add_space(16.0);
                }

                // Results display
                egui::Frame {
                    fill: egui::Color32::from_rgb(30, 35, 45),
                    rounding: egui::Rounding::same(8.0),
                    inner_margin: egui::Margin::same(16.0),
                    ..Default::default()
                }
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new("Results")
                                .size(15.0)
                                .color(egui::Color32::WHITE)
                                .strong(),
                        );

                        if !self.primes.is_empty() {
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    let filtered = self.get_filtered_numbers();
                                    ui.label(
                                        egui::RichText::new(format!(
                                            "Showing {} numbers",
                                            filtered.len()
                                        ))
                                        .size(11.0)
                                        .color(egui::Color32::GRAY),
                                    );
                                },
                            );
                        }
                    });

                    ui.add_space(12.0);

                    if self.primes.is_empty() && !self.calculating {
                        ui.vertical_centered(|ui| {
                            ui.add_space(40.0);
                            ui.label(
                                egui::RichText::new(
                                    "Enter a number and click Calculate to see results",
                                )
                                .color(egui::Color32::GRAY),
                            );
                        });
                    } else {
                        let available_height = ui.available_height().max(200.0);
                        let line_height = 18.0;
                        let lines_per_page =
                            ((available_height - 40.0) / line_height).floor().max(4.0) as usize;
                        const PRIMES_PER_LINE: usize = 10;

                        if !self.primes.is_empty() {
                            self.handle_terminal_input(ctx, lines_per_page);
                        }

                        let total_lines =
                            (self.primes.len() + PRIMES_PER_LINE - 1) / PRIMES_PER_LINE;
                        if total_lines > 0 && self.terminal_offset > total_lines.saturating_sub(1) {
                            self.terminal_offset = total_lines.saturating_sub(1);
                        }

                        self.refresh_page_buffer(lines_per_page, total_lines);

                        egui::Frame {
                            fill: egui::Color32::from_rgb(10, 12, 16),
                            rounding: egui::Rounding::same(6.0),
                            inner_margin: egui::Margin::same(10.0),
                            stroke: egui::Stroke::new(1.0, egui::Color32::from_rgb(40, 60, 90)),
                            ..Default::default()
                        }
                        .show(ui, |ui| {
                            ui.set_min_height(available_height - 12.0);
                            ui.horizontal(|ui| {
                                ui.label(
                                    egui::RichText::new(
                                        "Prime Viewer (less-like, use ↑/↓, PgUp/PgDn)",
                                    )
                                    .monospace()
                                    .color(egui::Color32::from_rgb(140, 220, 140)),
                                );
                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        ui.label(
                                            egui::RichText::new(format!(
                                                "pos: {} / {}",
                                                self.terminal_offset + 1,
                                                total_lines.max(1)
                                            ))
                                            .monospace()
                                            .color(egui::Color32::from_rgb(160, 170, 200)),
                                        );
                                    },
                                );
                            });
                            ui.add_space(6.0);

                            egui::ScrollArea::vertical()
                                .auto_shrink([false, false])
                                .show(ui, |ui| {
                                    ui.monospace(
                                        egui::RichText::new(&self.page_buffer)
                                            .color(egui::Color32::from_rgb(180, 210, 180)),
                                    );
                                });
                            ui.add_space(8.0);
                            let start_line = self.terminal_offset;
                            let end_line = (start_line + lines_per_page).min(total_lines);
                            ui.label(
                                egui::RichText::new(format!(
                                    "Showing lines {}-{} of {}, page size ~{}",
                                    if total_lines == 0 {
                                        0
                                    } else {
                                        self.terminal_offset + 1
                                    },
                                    end_line,
                                    total_lines,
                                    lines_per_page
                                ))
                                .monospace()
                                .size(11.0)
                                .color(egui::Color32::from_rgb(120, 140, 170)),
                            );
                        });
                    }
                });
            });

        // Slow refresh heartbeat in the bottom-right corner to show the app is still alive.
        egui::Area::new("heartbeat_overlay".into())
            .anchor(egui::Align2::RIGHT_BOTTOM, [-12.0, -10.0])
            .interactable(false)
            .show(ctx, |ui| {
                ui.label(
                    egui::RichText::new(format!("tick mod32467: {}", self.heartbeat_value))
                        .size(11.0)
                        .color(egui::Color32::from_rgb(120, 200, 120)),
                );
            });
    }
}

impl PrimeCalculatorApp {
    fn refresh_page_buffer(&mut self, lines_per_page: usize, total_lines: usize) {
        let needs_refresh = self.page_buffer_offset != self.terminal_offset
            || self.page_buffer_lines_per_page != lines_per_page
            || self.page_buffer_total_lines != total_lines
            || self.page_buffer_primes_len != self.primes.len();

        if !needs_refresh {
            return;
        }

        self.page_buffer_offset = self.terminal_offset;
        self.page_buffer_lines_per_page = lines_per_page;
        self.page_buffer_total_lines = total_lines;
        self.page_buffer_primes_len = self.primes.len();

        self.page_buffer.clear();

        if self.primes.is_empty() {
            if self.calculating {
                self.page_buffer.push_str("Calculating... please wait.\n");
            } else {
                self.page_buffer
                    .push_str("No data. Run a calculation to populate primes.\n");
            }
            return;
        }

        let start_line = self.terminal_offset;
        let end_line = (start_line + lines_per_page).min(total_lines);

        const PRIMES_PER_LINE: usize = 10;

        // Pre-allocate roughly enough space for the visible page to avoid reallocations.
        let approx_chars_per_prime = 8; // "  123456"
        let approx_line_len = 12 + PRIMES_PER_LINE * approx_chars_per_prime;
        let approx_capacity = (end_line.saturating_sub(start_line) * approx_line_len).max(64);
        self.page_buffer.reserve(approx_capacity);

        for line_idx in start_line..end_line {
            let start_idx = line_idx * PRIMES_PER_LINE;
            let end_idx = (start_idx + PRIMES_PER_LINE).min(self.primes.len());
            let mut line = format!("{:>8}: ", start_idx + 1);
            for (i, prime) in self.primes[start_idx..end_idx].iter().enumerate() {
                if i > 0 {
                    line.push_str("  ");
                }
                line.push_str(&format!("{:>6}", prime));
            }
            self.page_buffer.push_str(&line);
            self.page_buffer.push('\n');
        }
    }
}

fn load_font_data() -> Option<egui::FontData> {
    let font_paths = vec![
        "C:\\Windows\\Fonts\\msyh.ttc",
        "C:\\Windows\\Fonts\\simhei.ttf",
        "C:\\Windows\\Fonts\\simsun.ttc",
        "/System/Library/Fonts/PingFang.ttc",
        "/System/Library/Fonts/Hiragino Sans GB.ttc",
        "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
        "/usr/share/fonts/truetype/wqy/wqy-zenhei.ttc",
        "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
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
    let mut fonts = egui::FontDefinitions::default();

    if let Some(font_data) = load_font_data() {
        fonts.font_data.insert("chinese_font".to_owned(), font_data);

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
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1100.0, 700.0])
            .with_min_inner_size([900.0, 600.0]),
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
