// Time complexity: O(nlogn)
// Space complexity: O(n)
use std::collections::BTreeMap;

struct MyCalendar {
    timeline: BTreeMap<i32, i32>,
}

impl MyCalendar {
    fn new() -> Self {
        Self {
            timeline: BTreeMap::new(),
        }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        if let Some((&prev_start, &prev_end)) = self.timeline.range(start..).next() {
            if prev_start < end {
                return false;
            }
        }

        if let Some((&prev_start, &prev_end)) = self.timeline.range(..end).next_back() {
            if prev_end > start {
                return false;
            }
        }

        self.timeline.insert(start, end);
        true
    }
}
