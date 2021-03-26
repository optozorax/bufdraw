use crate::now;

#[derive(Clone)]
pub struct PerformanceMeasurer {
	print_skip: usize,
	trigger_counter: usize,
	counter: usize,
	total_time: f64,
	current_time: f64,
}

// Ticks per second
#[derive(Clone, Debug)]
pub struct Clock {
	current_time: f64,
}

#[derive(Clone, Debug)]
pub struct Duration {
	pub seconds: f64,
}

impl Duration {
	#[inline]
	pub fn new(start: f64, end: f64) -> Duration {
		assert!(start <= end);
		Duration {
			seconds: end - start
		}
	}

	#[inline]
	fn from_seconds(seconds: f64) -> Duration {
		assert!(seconds >= 0.0);
		Duration { seconds }
	}

	#[inline]
	pub fn fps(&self) -> f64 {
		1.0 / self.seconds
	}
}

impl Clock {
	#[inline]
	fn new(current_time: f64) -> Clock {
		Clock {
			current_time
		}
	}

	#[inline]
	pub fn now() -> Clock {
		Clock {
			current_time: now()
		}
	}

	#[inline]
	pub fn elapsed(&self) -> Duration {
		Duration::new(self.current_time, now())
	}
}

#[inline]
pub fn time<F: FnOnce(Clock)>(f: F) -> Duration {
	let clock = Clock::new(now());
	f(clock.clone());
	clock.elapsed()
}

struct AverageTimer {
	total_duration: f64,
	counter: usize,
	weight: f64,
}

pub struct AverageDuration {
	pub avg_duration: Duration,
	pub iterations: usize,
}

impl AverageTimer {
	#[inline]
	pub fn new() -> AverageTimer {
		AverageTimer { 
			total_duration: 0.0, 
			counter: 0,
			weight: 0.0,
		}
	}

	#[inline]
	pub fn time_avg<F: FnOnce(Clock)>(&mut self, f: F) -> AverageDuration {
		self.total_duration += time(f).seconds;
		self.counter += 1;
		self.weight += 1.0;
		self.get_current()
	}

	#[inline]
	pub fn get_current(&self) -> AverageDuration {
		AverageDuration { 
			avg_duration: Duration::from_seconds(
				self.total_duration / self.weight
			),
			iterations: self.counter
		}
	}
}

struct CountTrigger {
	counter: usize,
	trigger: usize,
}

impl CountTrigger {
	#[inline]
	pub fn new(trigger_count: usize) -> CountTrigger {
		assert!(trigger_count > 0);
		CountTrigger {
			counter: 0,
			trigger: trigger_count,
		}
	}

	#[inline]
	pub fn action<F: FnOnce()>(&mut self, f: F) -> bool {
		self.counter += 1;
		f();
		self.counter % self.trigger == 1 || self.trigger == 1
	}

	#[inline]
	pub fn get_count(&self) -> usize {
		self.counter
	}
}

pub struct FpsWithCounter {
	counter: CountTrigger,
	timer: AverageTimer,
}

impl FpsWithCounter {
	#[inline]
	pub fn new(trigger_count: usize) -> FpsWithCounter {
		FpsWithCounter {
			counter: CountTrigger::new(trigger_count),
			timer: AverageTimer::new(),
		}
	}

	#[inline]
	pub fn action<F: FnMut(Clock)>(&mut self, f: F) -> Option<Duration> {
		let mut duration = Duration::from_seconds(0.0);
		let is_trigger = self.counter.action(|| {
			duration = time(f)
		});
		if is_trigger {
			Some(duration)
		} else {
			None
		}
	}

	#[inline]
	pub fn action_avg<F: FnMut(Clock)>(&mut self, f: F) -> Option<Duration> {
		let mut duration = Duration::from_seconds(0.0);
		let timer = &mut self.timer;
		let is_trigger = self.counter.action(|| {
			let res = timer.time_avg(f);
			duration = res.avg_duration;
		});
		if is_trigger {
			Some(duration)
		} else {
			None
		}
	}

	#[inline]
	pub fn get_current(&self) -> AverageDuration {
		self.timer.get_current()
	}

	#[inline]
	pub fn get_count(&self) -> usize {
		self.counter.get_count()
	}
}

use std::collections::VecDeque;

pub struct FpsByLastTime {
	mas: VecDeque<f64>,
	clock: Clock,
	last_time: f64, 
}

impl FpsByLastTime {
	pub fn new(last_time: f64) -> Self {
		FpsByLastTime {
			mas: VecDeque::new(),
			clock: Clock::now(),
			last_time,
		}
	}

	pub fn clear(&mut self) {
		self.clock = Clock::now();
		self.mas.clear();
	}

	pub fn fps(&self) -> f64 {
		let now = self.clock.elapsed().seconds;
		match self.mas.front() {
			Some(t) => self.mas.len() as f64 / (now - t),
			None => 0.0,
		}
	}

	pub fn frame(&mut self) {
		let now = self.clock.elapsed().seconds;
		self.mas.push_back(self.clock.elapsed().seconds);
		while let Some(x) = self.mas.front() {
			if now - x > self.last_time {
				self.mas.pop_front();
			} else {
				break;
			}
		}
	}
}
