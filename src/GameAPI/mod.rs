use crate::SceneManager::SceneManager;
use crate::GameState::GameState;
use crate::MetaInfo::MetaInfo;

/// # Description
///     General utilities that should be exposed to game entities.
///     This allows entities to access and manipulate game data, 
///     meta information, game entities, and scenes.
pub struct GameAPI
{
    pub SceneManager: SceneManager,
    pub GameState: GameState,
    pub MetaInfo: MetaInfo
}

impl GameAPI
{
    pub fn Create() -> Self
    {
        Self
        {
            SceneManager: SceneManager::Create(),
            GameState: GameState::Create(),
            MetaInfo: MetaInfo::Create()
        }
    }
}



