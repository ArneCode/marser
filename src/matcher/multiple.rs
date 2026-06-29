//! Zero-or-more repetition matcher; stops when `matcher` fails or makes no progress.

use crate::{
    error::{MatcherRunError, error_handler::ErrorHandler},
    input::{Input, InputStream},
    matcher::{MatchRunner, Matcher, repeat::run_repeat_loop},
};

/// Greedy `matcher*` at the matcher level (always reports match success after the loop).
#[derive(Clone, Debug)]
pub struct Multiple<Match> {
    matcher: Match,
}

impl<Match> Multiple<Match> {
    fn new(matcher: Match) -> Self {
        Self { matcher }
    }
}

/// See [`Multiple`].
pub fn many<Match>(matcher: Match) -> Multiple<Match> {
    Multiple::new(matcher)
}

impl<Match> super::MatcherCombinator for Multiple<Match> where Match: super::MatcherCombinator {}

impl<'src, Inp: Input<'src>, MRes, Match> super::internal::MatcherImpl<'src, Inp, MRes>
    for Multiple<Match>
where
    Match: Matcher<'src, Inp, MRes>,
    Inp: Input<'src>,
{
    const CAN_MATCH_DIRECTLY: bool = Match::CAN_MATCH_DIRECTLY;
    const HAS_PROPERTY: bool = Match::HAS_PROPERTY;
    const CAN_FAIL: bool = false;
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
            0,
            None,
            runner,
            error_handler,
            input,
        )
    }
}
