use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MusicPlatform {
    Custom,
    QQ,
    NetEase,
    MiGu,
    KuGou,
    Kuwo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicElement {
    /// 元素类型
    #[serde(rename = "type")]
    r#type: &'static str,
    /// 音乐数据
    /// 自动展开
    #[serde(flatten)]
    data: MusicData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MusicData {
    /// 普通音乐
    Standard {
        /// 音乐平台
        platform: MusicPlatform,
        /// 歌曲id
        id: String,
    },
    /// 自定义音乐
    Custom {
        /// 音乐平台
        platform: MusicPlatform,
        /// 跳转链接
        url: String,
        /// 音乐音频链接
        audio: String,
        /// 标题
        title: String,
        /// 歌手
        author: String,
        /// 封面
        pic: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomMusicElement {
    /// 元素类型
    #[serde(rename = "type")]
    r#type: &'static str,
    /// 音乐平台
    platform: MusicPlatform,
    /// 跳转链接
    url: String,
    /// 音乐音频链接
    audio: String,
    /// 标题
    title: String,
    /// 歌手
    author: String,
    /// 封面
    pic: String,
}

impl MusicElement {
    /// 创建一个普通音乐元素
    ///
    /// # 参数
    ///
    /// * `platform` - 音乐平台
    /// * `id` - 歌曲id
    ///
    /// # 返回值
    ///
    /// * `MusicElement` - 音乐元素
    fn new_standard(platform: MusicPlatform, id: impl Into<String>) -> Self {
        Self {
            r#type: "music",
            data: MusicData::Standard {
                platform,
                id: id.into(),
            },
        }
    }

    /// 创建一个自定义音乐元素
    ///
    /// # 参数
    ///
    /// * `url` - 跳转链接
    /// * `audio` - 音乐音频链接
    /// * `title` - 标题
    /// * `author` - 歌手
    /// * `pic` - 封面
    ///
    /// # 返回值
    ///
    /// * `MusicElement` - 自定义音乐元素
    pub fn new_custom(
        url: impl Into<String>,
        audio: impl Into<String>,
        title: impl Into<String>,
        author: impl Into<String>,
        pic: impl Into<String>,
    ) -> Self {
        Self {
            r#type: "music",
            data: MusicData::Custom {
                platform: MusicPlatform::Custom,
                url: url.into(),
                audio: audio.into(),
                title: title.into(),
                author: author.into(),
                pic: pic.into(),
            },
        }
    }
    /// 获取音乐平台
    pub fn platform(&self) -> MusicPlatform {
        match &self.data {
            MusicData::Standard { platform, .. } => *platform,
            MusicData::Custom { platform, .. } => *platform,
        }
    }
    /// 获取歌曲id
    pub fn id(&self) -> Option<&str> {
        match &self.data {
            MusicData::Standard { id, .. } => Some(id.as_str()),
            MusicData::Custom { .. } => None,
        }
    }

    /// 创建一个qq音乐元素
    ///
    /// # 参数
    ///
    /// * `id` - 歌曲id
    ///
    /// # 返回值
    ///
    /// * `MusicElement` - qq音乐元素
    pub fn qq(id: impl Into<String>) -> Self {
        Self::new_standard(MusicPlatform::QQ, id)
    }

    /// 创建一个网易云音乐元素
    ///
    /// # 参数
    ///
    /// * `id` - 歌曲id
    ///
    /// # 返回值
    ///
    /// * `MusicElement` - 网易云音乐元素
    pub fn net_ease(id: String) -> Self {
        Self::new_standard(MusicPlatform::NetEase, id)
    }

    /// 创建一个咪咕音乐元素
    ///
    /// # 参数
    ///
    /// * `id` - 歌曲id
    ///
    /// # 返回值
    ///
    /// * `MusicElement` - 咪咕音乐元素
    pub fn migu(id: String) -> Self {
        Self::new_standard(MusicPlatform::MiGu, id)
    }
    /// 创建一个酷狗音乐元素
    ///
    /// # 参数
    ///
    /// * `id` - 歌曲id
    ///
    /// # 返回值
    ///
    /// * `MusicElement` - 酷狗音乐元素
    pub fn kugou(id: String) -> Self {
        Self::new_standard(MusicPlatform::KuGou, id)
    }

    /// 创建一个酷我音乐元素
    ///
    /// # 参数
    ///
    /// * `id` - 歌曲id
    ///
    /// # 返回值
    ///
    /// * `MusicElement` - 酷我音乐元素
    pub fn kuwo(id: String) -> Self {
        Self::new_standard(MusicPlatform::Kuwo, id)
    }
}
