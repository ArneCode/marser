//! Bounded repetition matcher (`repeat(m, bounds)`).

use alloc::boxed::Box;
use core::{
    fmt::{Debug, Display},
    ops::{Bound, RangeBounds},
};

use crate::{
    error::{MatcherRunError, error_handler::ErrorHandler},
    input::{Input, InputStream},
    matcher::{MatchRunner, Matcher},
};

/// Decodes [`RangeBounds<usize>`] into a minimum count and inclusive maximum (`None` = unbounded).
fn decode_bounds<B: RangeBounds<usize>>(bounds: &B) -> (usize, Option<usize>) {
    let min = match bounds.start_bound() {
        Bound::Unbounded => 0,
        Bound::Included(&n) => n,
        Bound::Excluded(&n) => n.saturating_add(1),
    };

    let max_inclusive = match bounds.end_bound() {
        Bound::Unbounded => None,
        Bound::Included(&n) => Some(n),
        Bound::Excluded(&n) => {
            if n <= min {
                return (min, Some(min.wrapping_sub(1)));
            }
            Some(n - 1)
        }
    };

    (min, max_inclusive)
}

/// Greedy repetition with mandatory minimum, optional maximum, and no-progress guard.
#[inline(always)]
pub(crate) fn run_repeat_loop<'a, 'src, Inp, MRes, Runner, M, Match, EH>(
    matcher: &'a Match,
    min: usize,
    max_inclusive: Option<usize>,
    runner: &mut Runner,
    error_handler: &mut EH,
    input: &mut InputStream<'src, Inp>,
) -> Result<bool, MatcherRunError>
where
    Inp: Input<'src>,
    Match: Matcher<'src, Inp, MRes>,
    Runner: MatchRunner<'a, 'src, Inp, MRes = MRes>,
    M: crate::mode::Mode,
    EH: ErrorHandler,
    'src: 'a,
{
    if max_inclusive.is_some_and(|m| m < min) {
        return Ok(false);
    }

    let mut count = 0usize;

    while count < min {
        if !runner.run_match::<_, M, _>(matcher, error_handler, input)? {
            return Ok(false);
        }
        count += 1;
    }

    loop {
        if max_inclusive.is_some_and(|m| count >= m) {
            break;
        }
        let before = input.get_pos();
        if !runner.run_match::<_, M, _>(matcher, error_handler, input)? {
            break;
        }
        if input.get_pos().into() == before.into() {
            break;
        }
        count += 1;
    }

    Ok(count >= min && max_inclusive.is_none_or(|m| count <= m))
}

/// Greedy bounded repetition of `matcher`.
#[derive(Clone, Debug)]
pub struct Repeat<Match> {
    matcher: Match,
    min: usize,
    max_inclusive: Option<usize>,
}

impl<Match> Repeat<Match> {
    fn new(matcher: Match, min: usize, max_inclusive: Option<usize>) -> Self {
        Self {
            matcher,
            min,
            max_inclusive,
        }
    }
}

/// `repeat(matcher, bounds)` — match `matcher` between `min` and `max` times (inclusive).
///
/// `bounds` is a [`RangeBounds<usize>`] repetition count, e.g. `2..5` (2–4 times, half-open),
/// `1..`, `2..=4`, `..=3`, or `3..=3` for exactly three.
pub fn repeat<Match, B>(matcher: Match, bounds: B) -> Repeat<Match>
where
    B: RangeBounds<usize>,
{
    let (min, max_inclusive) = decode_bounds(&bounds);
    Repeat::new(matcher, min, max_inclusive)
}

impl<Match> super::MatcherCombinator for Repeat<Match> where Match: super::MatcherCombinator {}

impl<'src, Inp: Input<'src>, MRes, Match> super::internal::MatcherImpl<'src, Inp, MRes>
    for Repeat<Match>
where
    Match: Matcher<'src, Inp, MRes>,
    Inp: Input<'src>,
{
    const CAN_MATCH_DIRECTLY: bool = Match::CAN_MATCH_DIRECTLY;
    const HAS_PROPERTY: bool = Match::HAS_PROPERTY;
    const CAN_FAIL: bool = true;

    #[inline]
    fn match_with_runner<'a, Runner, M: crate::mode::Mode>(
        &'a self,
        runner: &mut Runner,
        error_handler: &mut impl ErrorHandler,
        input: &mut InputStream<'src, Inp>,
    ) -> Result<bool, MatcherRunError>
    where
        Runner: MatchRunner<'a, 'src, Inp, MRes = MRes>,
        'src: 'a,
    {
        run_repeat_loop::<Inp, MRes, Runner, M, Match, _>(
            &self.matcher,
            self.min,
            self.max_inclusive,
            runner,
            error_handler,
            input,
        )
    }
}