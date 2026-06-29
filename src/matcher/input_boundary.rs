//! AI assistance: this file was written with AI assistance. The maintainer reviewed it and did not find errors.
//!
//! Zero-width matchers at the start or end of an [`crate::input::Input`] stream.

use crate::{
    error::{MatcherRunError, error_handler::ErrorHandler},
    input::{Input, InputStream},
    matcher::{
        MatchRunner, MatcherCombinator, any_token::AnyToken,
        negative_lookahead::{NegativeLookahead, negative_lookahead},
    },
};

/// Succeeds when no tokens remain (equivalent to [`negative_lookahead`](crate::matcher::negative_lookahead)([`AnyToken`])).
#[inline]
pub fn end_of_input() -> NegativeLookahead<AnyToken> {
    negative_lookahead(AnyToken)
}

/// Zero-width matcher: succeeds when the cursor equals [`Input::start_pos`] for this stream.
#[derive(Clone, Debug)]
pub struct StartOfInput;

/// See [`StartOfInput`].
#[inline]
pub fn start_of_input() -> StartOfInput {
    StartOfInput
}

impl MatcherCombinator for StartOfInput {}

impl<'src, Inp: Input<'src>, MRes> super::internal::MatcherImpl<'src, Inp, MRes> for StartOfInput
where
    Inp: Input<'src>,
{
    const CAN_MATCH_DIRECTLY: bool = true;
    const HAS_PROPERTY: bool = false;
    const CAN_FAIL: bool = true;

    #[inline]
    fn match_with_runner<'a, Runner, M: crate::mode::Mode>(
        &'a self,
        _runner: &mut Runner,
        _error_handler: &mut impl ErrorHandler,
        input: &mut InputStream<'src, Inp>,
    ) -> Result<bool, MatcherRunError>
    where
        Runner: MatchRunner<'a, 'src, Inp, MRes = MRes>,
        'src: 'a,
    {
        Ok(input.is_at_start())
    }
}
