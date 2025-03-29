use std::time::Duration;

///////////////////////
// Shared: Main, Window
pub const POLLING_INTERVAL: Duration = Duration::from_micros(1);

/////////////////
// Main constants
pub const MIN_WINDOW_WIDTH: u16 = 50;
pub const MIN_WINDOW_HEIGHT: u16 = 20;
pub const ACTIVITY_CHECK_INTERVAL: Duration = Duration::from_millis(100);
pub const UPDATE_INTERVAL: Duration = Duration::from_micros(50);

///////////////
// UI constants
pub const MAX_WINDOW_WIDTH: u16 = 120;
pub const MAX_WINDOW_HEIGHT: u16 = 40;
pub const MIN_TIME_WINDOW: f64 = 5.0;

////////////////
// App constants
pub const INITIALIZATION_DELAY_MS: u64 = 500;
pub const GRAPH_PADDING: f64 = 1.1; // 10% padding above max detected rate
pub const INACTIVITY_THRESHOLD_MS: f64 = 50.0; // Threshold in milliseconds for adding zero values
pub const EVENT_WINDOW: Duration = Duration::from_secs(1);
pub const INITIAL_MAX_RATE: f64 = 1000.0;
pub const HISTORY_SIZE: usize = 10000; // Store 10 seconds at ~1000Hz sampling (doubled)

// App: "calculate_rate" function
pub const MIN_TIME_SPAN: f64 = 0.001;
pub const SMOOTHING_FACTOR: f64 = 0.9;
pub const SMALL_WINDOW_RANGE: std::ops::Range<usize> = 3..20;
pub const OUTLIER_WINDOW: f64 = 0.25;
