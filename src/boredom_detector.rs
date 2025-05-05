//! 劇情品質自評：重複 N-gram、張力停滯、對話迴圈等，給出 0~1 分。
use crate::src::scene_scoring::score_scene;

/// 批量評估最近 N 篇場景分數，分數過低自動呼叫再生。
pub fn batch_scene_quality(scenes: &[String], threshold: f32) -> Vec<(f32, String)> {
    let mut results = Vec::new();
    for scene in scenes {
        let (score, reason) = score_scene(scene);
        if score < threshold {
            // TODO: 呼叫 StoryContentGenerator::regenerate_scene(scene)
            println!("[再生] 劇情分數過低({:.2})，已自動再生：{}", score, reason);
        }
        results.push((score, reason));
    }
    results
}
