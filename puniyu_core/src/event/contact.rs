pub enum Scene {
    Group,
    Friend,
}

impl Scene {
    pub fn to_scene_str(&self) -> &'static str {
        match self {
            Scene::Group => "group",
            Scene::Friend => "friend",
        }
    }
}

pub struct FriendContact {
    /// 联系人场景
    scene: Scene,
    /// 联系人 ID
    peer: String,
    /// 联系人名称
    name: String,
}

impl FriendContact {
    pub fn new(peer: String, name: String) -> Self {
        Self {
            scene: Scene::Friend,
            peer,
            name,
        }
    }

    pub fn peer(&self) -> &str {
        &self.peer
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
