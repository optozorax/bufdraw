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
#[derive(Clone)]
pub struct Clock {
	current_time: f64,
}

#[derive(Clone)]
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
pub fn time<F: FnOnce(Clock) -> ()>(f: F) -> Duration {
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
	pub fn time_avg<F: FnOnce(Clock) -> ()>(&mut self, f: F) -> AverageDuration {
		self.total_duration += time(f).seconds;
		self.counter += 1;
		self.weight += 1.0;
		self.get_current()
	}

	#[inline]
	pub fn time_avg_weighted<F: FnOnce(Clock) -> ()>(&mut self, f: F, weight: f64) -> AverageDuration {
		self.total_duration += time(f).seconds;
		self.counter += 1;
		self.weight += weight;
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
	pub fn action<F: FnOnce() -> ()>(&mut self, f: F) -> bool {
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
	pub fn action<F: FnMut(Clock) -> ()>(&mut self, f: F) -> Option<Duration> {
		let mut duration = Duration::from_seconds(0.0);
		if self.counter.action(|| {
			duration = time(f)
		}) {
			Some(duration)
		} else {
			None
		}
	}

	#[inline]
	pub fn action_avg<F: FnMut(Clock) -> ()>(&mut self, f: F) -> Option<Duration> {
		let mut duration = Duration::from_seconds(0.0);
		let timer = &mut self.timer;
		if self.counter.action(|| {
			let res = timer.time_avg(f);
			duration = res.avg_duration;
		}) {
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