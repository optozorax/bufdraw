use std::ops::Range;
use std::cmp::Ord;
use std::ops::{Add, Div, Rem};

pub fn offset_range<Idx>(range: &Range<Idx>, offset: Idx) -> Range<Idx> where
	Idx: Add<Idx, Output = Idx> + Copy
{
	(range.start + offset)..(range.end + offset)
}

pub trait Zero {
	fn zero() -> Self;
	fn is_zero(&self) -> bool;
}

impl Zero for i32 {
	fn zero() -> Self { 0 }
	fn is_zero(&self) -> bool { *self == 0 }
}

pub fn div_ceil<T>(a: T, b: T) -> T where 
	T: Copy + Rem<T, Output = T> + PartialEq + Div<T, Output = T> + Add<T, Output = T> + Zero
{
	if (a % b).is_zero() {
		a / b
	} else {
		(a + b) / b
	}
}

pub fn div_range<T>(range: &Range<T>, div: T) -> Range<T> where
	T: Copy + Rem<T, Output = T> + PartialEq + Div<T, Output = T> + Add<T, Output = T> + Zero
{
	(range.start / div)..div_ceil(range.end, div)
}

fn pos_in_interval<T>(min: T, pos: T, max: T) -> T where
	T: Ord
{
	min.max(max.min(pos))
}

pub fn intersect_range<Idx: Ord + Copy>(a: &Range<Idx>, b: &Range<Idx>) -> Range<Idx> {
	pos_in_interval(a.start, b.start, a.end)..pos_in_interval(a.start, b.end, a.end)
}

pub fn range_to_usize(range: &Range<i32>) -> Range<usize> {
	(range.start as usize)..(range.end as usize)
}