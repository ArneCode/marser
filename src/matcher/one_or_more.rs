//! One-or-more repetition (`matcher+`).

use crate::{
    error::{MatcherRunError, error_handler::ErrorHandler},
    input::{Input, InputStream},
    matcher::{MatchRunner, Matcher, repeat::run_repeat_loop},
};

/// Requires at least one successful `matcher`, then behaves like greedy repetition.
#[derive(Clone, Debug)]
pub struct OneOrMore<Match> {
    matcher: Match,
}

impl<Match> OneOrMore<Match> {
    /// See [`one_or_more`].
    pub fn new(matcher: Match) -> Self {
        Self { matcher }
    }
}

/// e+  — match one or more repetitions of `matcher`, capturing each occurrence.
pub fn one_or_more<Match>(matcher: Match) -> OneOrMore<Match> {
    OneOrMore::new(matcher)
}

impl<Match> super::MatcherCombinator for OneOrMore<Match> where Match: super::MatcherCombinator {}

impl<'src, Inp: Input<'src>, MRes, Match> super::internal::MatcherImpl<'src, Inp, MRes>
    for OneOrMore<Match>
where
    Match: Matcher<'src, Inp, MRes>,
    Inp: Input<'src>,
{
    const CAN_MATCH_DIRECTLY: bool = Match::CAN_MATCH_DIRECTLY;
    const HAS_PROPERTY: bool = Match::HAS_PROPERTY;
    const CAN_FAIL: bool = Match::CAN_FAIL;

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
            1,
            None,
            runner,
            error_handler,
            input,
        )
    }
}
