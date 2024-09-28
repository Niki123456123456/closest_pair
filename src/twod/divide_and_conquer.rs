use super::*;

pub struct DivideAndConquer;

impl ClosestPairAlgorithm for DivideAndConquer {
    fn name(&self,) -> &'static str {
        "divide and conquer"
    }
    fn execute<'a>(&self,points : &'a[Point]) -> ClosestPair<'a> {
        todo!()
    }
    
    fn drawings<'a>(&self, points: &'a[Point]) -> Vec<Vec<Drawing>> {
        let mut drawings = vec![];
        return drawings;
    }
}