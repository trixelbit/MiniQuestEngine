use crate::Engine::SceneManager::SceneManager;
use crate::Engine::GameState::GameState;
use crate::Engine::MetaInfo::MetaInfo;
use crate::Engine::Audio::AudioModule;
use crate::Engine::Collision::CollisionModule;
use crate::Engine::SceneBuilder::SceneBuilder;
use crate::Engine::Shader::ShaderModule;

/// # Description
///     General utilities that should be exposed to game entities.
///     This allows entities to access and manipulate game data, 
///     meta information, game entities, and scenes.
pub struct GameAPI
{
    pub SceneManager: SceneManager,
    pub GameState: GameState,
    pub MetaInfo: MetaInfo,

    pub Audio: AudioModule,
    pub Collision: CollisionModule,
    pub Shader: ShaderModule
}

impl GameAPI
{
    pub fn Create(sceneBuilderMethod: SceneBuilder) -> Self
    {
        Self
        {
            SceneManager: SceneManager::Create(sceneBuilderMethod),
            GameState: GameState::Create(),
            MetaInfo: MetaInfo::Create(),
            Audio: AudioModule::Create(),
            Collision: CollisionModule::Create(),
            Shader: ShaderModule::Create()
        }
    }
}

unsafe impl Send for GameAPI{}



