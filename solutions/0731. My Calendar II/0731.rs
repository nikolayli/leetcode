// Time complexity: O(n)
// Space complexity: O(n)
struct MyCalendarTwo {
    timeline: Vec<(i32, i32)>,
    overlap_timeline: Vec<(i32, i32)>,
}

impl MyCalendarTwo {
    fn new() -> Self {
        MyCalendarTwo {
            timeline: Vec::new(),
            overlap_timeline: Vec::new(),
        }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        for &(start1, end1) in &self.overlap_timeline {
            if Self::does_overlap(start1, end1, start, end) {
                return false;
            }
        }

        for &(start1, end1) in &self.timeline {
            if Self::does_overlap(start1, end1, start, end) {
                self.overlap_timeline
                    .push(Self::get_overlapped(start1, end1, start, end));
            }
        }

        self.timeline.push((start, end));
        true
    }

    fn does_overlap(start1: i32, end1: i32, start2: i32, end2: i32) -> bool {
        start1.max(start2) < end1.min(end2)
    }

    fn get_overlapped(start1: i32, end1: i32, start2: i32, end2: i32) -> (i32, i32) {
        (start1.max(start2), end1.min(end2))
    }
}
