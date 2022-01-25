use anyhow::Result;
use rand::thread_rng;
use rand::seq::SliceRandom;
use crate::domain::VecShuffleStrategy;

/// `Vec`をシャッフルする方式(Stratery)
pub enum ShuffleStrategies{
    /// シャッフルしない
    NoShuffle,
    /// ランダムでシャッフルする
    RandomShuffle
}

impl VecShuffleStrategy for ShuffleStrategies {
    /// 与えられた`Vec`をシャッフルする。`Vec`を破壊するメソッドである点注意
    /// # Attributes
    /// * `vec` - シャッフルする配列
    /// # Returns
    /// Ok(()) - `vec`の参照を受け取り直接シャッフルする 
    fn shuffle<T>(&self, vec: &mut Vec<T>) -> Result<()> {
        match self {
            Self::NoShuffle => Ok(()),
            Self::RandomShuffle => {
                let mut rng = thread_rng();
                vec.shuffle(&mut rng);
        
                Ok(())
            }
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_shuffle_does_nothing () {
        let mut v = vec!(0,1,2,3,4);
        let clone = v.clone();

        ShuffleStrategies::NoShuffle.shuffle(&mut v).unwrap();

        assert_eq!(v, clone);
    }

    #[test]
    fn random_shuffle_shuffles_vec () {
        let mut v = vec!(0,1,2,3,4);
        let clone = v.clone();

        ShuffleStrategies::RandomShuffle.shuffle(&mut v).unwrap();

        assert_ne!(v, clone);
    }
}