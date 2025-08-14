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
pub struct CustomMusicOptions {
    /// 跳转链接
    pub url: String,
    /// 音乐音频链接
    pub audio: String,
    /// 标题
    pub title: String,
    /// 歌手
    pub author: String,
    /// 封面
    pub pic: String,
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
    pub r#type: String,
    /// 音乐平台
    pub platform: MusicPlatform,
    /// 跳转链接
    pub url: String,
    /// 音乐音频链接
    pub audio: String,
    /// 标题
    pub title: String,
    /// 歌手
    pub author: String,
    /// 封面
    pub pic: String,
}
