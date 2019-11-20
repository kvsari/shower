//! Numeric traits
use std::fmt::Debug;

use num_traits::{Float, NumAssign, NumCast};

pub trait Number: Copy + Clone + Debug + NumAssign + NumCast { }

impl <T> Number for T
where T: Copy + Clone + Debug + NumAssign + NumCast
{
}

/// Our lovable IEEE 754
pub trait Floater: Number + Float { }

impl <T> Floater for T
where T: Number + Float
{
}
