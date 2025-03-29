use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

use crate::config;

pub struct App {
    // Event tracking
    pub event_timestamps: VecDeque<Instant>,
    recent_events_window: VecDeque<Instant>,
    last_event_time: Instant,
    pub current_pos: (i32, i32),

    // Rate calculation
    pub rate_history: VecDeque<(f64, f64)>,
    last_calculated_rate: f64,
    pub max_rate: f64,
    pub graph_max_rate: f64,

    // General state
    pub start_time: Instant,
    initialization_complete: bool,
    creation_time: Instant,
}

impl App {
    pub fn new() -> Self {
        let now = Instant::now();
        App {
            // Event tracking
            event_timestamps: VecDeque::with_capacity(config::HISTORY_SIZE),
            recent_events_window: VecDeque::with_capacity(1200),
            last_event_time: now,
            current_pos: (0, 0),

            // Rate calculation
            rate_history: VecDeque::with_capacity(config::HISTORY_SIZE),
            last_calculated_rate: 0.0,
            max_rate: 0.0,
            graph_max_rate: config::INITIAL_MAX_RATE,

            // General state
            start_time: now,
            initialization_complete: false,
            creation_time: now,
        }
    }

    pub fn check_initialization(&mut self) {
        if !self.initialization_complete
            && self.creation_time.elapsed().as_millis() >= config::INITIALIZATION_DELAY_MS as u128
        {
            self.initialization_complete = true;
            let current_time = self.start_time.elapsed().as_secs_f64();
            self.rate_history.push_back((current_time, 0.0));
        }
    }

    pub fn add_event(&mut self, timestamp: Instant, pos: Option<(i32, i32)>) {
        self.check_initialization();

        self.event_timestamps.push_back(timestamp);
        self.recent_events_window.push_back(timestamp);
        self.last_event_time = timestamp;

        if let Some(new_pos) = pos {
            self.current_pos = new_pos;
        }

        while self.event_timestamps.len() > config::HISTORY_SIZE {
            self.event_timestamps.pop_front();
        }

        let now = Instant::now();
        while let Some(front) = self.recent_events_window.front() {
            if now.duration_since(*front) > config::EVENT_WINDOW {
                self.recent_events_window.pop_front();
            } else {
                break;
            }
        }

        if self.initialization_complete {
            self.update_rate(timestamp);
        }
    }

    fn update_rate(&mut self, timestamp: Instant) {
        let current_time = timestamp.duration_since(self.start_time).as_secs_f64();
        let current_rate = self.calculate_rate();

        if current_rate > self.max_rate {
            self.max_rate = current_rate;
            self.graph_max_rate = self.max_rate * config::GRAPH_PADDING;
        }

        self.rate_history.push_back((current_time, current_rate));

        while self.rate_history.len() > config::HISTORY_SIZE {
            self.rate_history.pop_front();
        }
    }

    fn calculate_rate(&mut self) -> f64 {
        let window_size = self.recent_events_window.len();

        // Not enough data points
        if window_size < 2 {
            return 0.0;
        }

        let (oldest, newest) = match (
            self.recent_events_window.front(),
            self.recent_events_window.back(),
        ) {
            (Some(oldest), Some(newest)) => (oldest, newest),
            _ => return 0.0,
        };

        let time_span = newest.duration_since(*oldest).as_secs_f64();

        if time_span < config::MIN_TIME_SPAN {
            return self.last_calculated_rate;
        }

        let new_rate = (window_size as f64 - 1.0) / time_span;

        if let Some(recent_rate) = self.calculate_recent_rate(window_size, newest) {
            return recent_rate;
        }

        // Apply smoothing and update last calculated rate
        let smoothed_rate = config::SMOOTHING_FACTOR * new_rate
            + (1.0 - config::SMOOTHING_FACTOR) * self.last_calculated_rate;
        self.last_calculated_rate = smoothed_rate;
        smoothed_rate
    }

    fn calculate_recent_rate(&self, window_size: usize, newest: &Instant) -> Option<f64> {
        if !config::SMALL_WINDOW_RANGE.contains(&window_size) {
            return None;
        }

        let third_newest_event = self.recent_events_window[window_size - 3];
        let time_span = newest.duration_since(third_newest_event).as_secs_f64();

        if time_span < config::MIN_TIME_SPAN {
            return None;
        }

        let recent_rate = 2.0 / time_span;

        if self.is_rate_outlier(recent_rate, config::OUTLIER_WINDOW) {
            return Some(self.last_calculated_rate);
        }

        Some(recent_rate)
    }

    fn is_rate_outlier(&self, rate: f64, window: f64) -> bool {
        let recent_avg = self.calculate_avg_rate(window);
        recent_avg > 0.0 && rate > recent_avg * 2.0
    }

    pub fn calculate_current_rate(&self) -> f64 {
        let inactivity_period = Duration::from_millis(200);

        if Instant::now().duration_since(self.last_event_time) > inactivity_period {
            return 0.0;
        }

        self.last_calculated_rate
    }

    pub fn calculate_avg_rate(&self, window: f64) -> f64 {
        let now = self.rate_history.back().map(|&(t, _)| t).unwrap_or(0.0);
        let window_start = now - window;

        let recent_rates: Vec<_> = self
            .rate_history
            .iter()
            .filter(|&&(time, _)| time >= window_start)
            .map(|&(_, rate)| rate)
            .collect();

        if recent_rates.is_empty() {
            0.0
        } else {
            recent_rates.iter().sum::<f64>() / recent_rates.len() as f64
        }
    }

    pub fn ensure_data_continuity(&mut self) {
        if !self.initialization_complete {
            return;
        }

        let now = Instant::now();
        let current_time = now.duration_since(self.start_time).as_secs_f64();

        if let Some(&(last_timestamp, _)) = self.rate_history.back() {
            let time_since_last_update = current_time - last_timestamp;

            if time_since_last_update > (config::INACTIVITY_THRESHOLD_MS / 1000.0) {
                self.rate_history.push_back((current_time, 0.0));

                while self.rate_history.len() > config::HISTORY_SIZE {
                    self.rate_history.pop_front();
                }
            }
        }
    }

    pub fn is_initialized(&self) -> bool {
        self.initialization_complete
    }
}
