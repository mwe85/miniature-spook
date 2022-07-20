pub struct BoardState{
}

pub struct RCSConstraint{
  row_constraint: [usize; 9],
  column_constraint: [usize; 9],
  square_constraint: [usize;9]
}

pub struct HintMask{}

impl HintMask{
  pub fn fromRCSConstraint(constraint: RCSConstraint) -> HintMask{
    HintMask{}
  }
}

/** need mapping functions between like a column,row pair and some value within 0..80**/
