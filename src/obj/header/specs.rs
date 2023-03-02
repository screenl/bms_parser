#[derive(Debug)]
pub enum PlayMode{
    SinglePlay,
    DoublePlay,
    CouplePlay,
    BattlePlay,
}

#[derive(Debug)]
pub enum JudgeRank{
    VeryHard,
    Hard,
    Normal,
    Easy,
    VeryEasy,
} 
#[derive(Debug)]
pub enum Difficulty{
    Beginner,
    Normal,
    Hyper,
    Another,
    Insane,
}

#[derive(Debug)]
pub enum LnType{
    Rdm,
    Mgq,
}